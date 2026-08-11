use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::time::Instant;

use crate::modules::performance::domain::models::metrics::PerformanceMetrics;
use crate::modules::performance::ports::MetricsCollector;
use crate::shared::kernel::result::AppError;

/// Raw scalar snapshot read from `sysinfo` while the system handle is locked.
struct SystemSnapshot {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
}

/// Production metrics collector backed by the `sysinfo` crate.
///
/// Collects real CPU usage and memory usage from the host operating system.
/// The collector owns a `sysinfo::System` handle wrapped in an [`Arc`] so the
/// blocking refresh can run on `spawn_blocking` without borrowing from the
/// async caller.
pub(crate) struct SystemMetricsCollector {
    system: Arc<Mutex<System>>,
}

impl SystemMetricsCollector {
    pub(crate) fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self {
            system: Arc::new(Mutex::new(system)),
        }
    }

    /// Refresh the underlying `sysinfo::System` and read the scalar values out
    /// while still holding the lock, so callers never observe a stale handle.
    fn snapshot(system: &Arc<Mutex<System>>) -> Result<SystemSnapshot, AppError> {
        let mut system = system
            .lock()
            .map_err(|e| AppError::State(format!("metrics collector mutex poisoned: {e}")))?;
        system.refresh_cpu_usage();
        system.refresh_memory();
        Ok(SystemSnapshot {
            cpu_usage: system.global_cpu_usage(),
            memory_used: system.used_memory(),
            memory_total: system.total_memory(),
        })
    }
}

impl Default for SystemMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MetricsCollector for SystemMetricsCollector {
    async fn collect_metrics(&self) -> Result<PerformanceMetrics, AppError> {
        // sysinfo is synchronous, so run the refresh on the blocking pool to
        // avoid stalling the async runtime. We clone the `Arc` so the
        // `spawn_blocking` closure is `'static`.
        let system = Arc::clone(&self.system);
        let snapshot = tokio::task::spawn_blocking(move || Self::snapshot(&system))
            .await
            .map_err(|e| AppError::State(format!("metrics collector task join error: {e}")))??;

        // response_time_ms / throughput / error_rate are application-level
        // metrics that the host collector cannot measure directly. We expose
        // them as zero so downstream consumers can layer their own probes on
        // top instead of receiving misleading placeholder values.
        Ok(PerformanceMetrics {
            cpu_usage: f64::from(snapshot.cpu_usage),
            memory_usage: snapshot.memory_used,
            memory_total: snapshot.memory_total,
            response_time_ms: 0,
            throughput: 0.0,
            error_rate: 0.0,
            ..PerformanceMetrics::new()
        })
    }

    async fn collect_metrics_over_period(
        &self,
        duration_seconds: u64,
    ) -> Result<Vec<PerformanceMetrics>, AppError> {
        if duration_seconds == 0 {
            return Ok(Vec::new());
        }

        let interval = std::cmp::min(duration_seconds, 10);
        let mut metrics_list =
            Vec::with_capacity(usize::try_from(duration_seconds / interval).unwrap_or(0) + 1);

        let start = Instant::now();
        let target = std::time::Duration::from_secs(duration_seconds);

        loop {
            metrics_list.push(self.collect_metrics().await?);
            let elapsed = start.elapsed();
            if elapsed >= target {
                break;
            }
            let remaining = target - elapsed;
            let sleep = std::cmp::min(remaining, std::time::Duration::from_secs(interval));
            tokio::time::sleep(sleep).await;
        }

        Ok(metrics_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collect_metrics() {
        let collector = SystemMetricsCollector::new();
        let metrics = collector.collect_metrics().await.unwrap();
        // CPU usage is a percentage and must be finite.
        assert!(metrics.cpu_usage.is_finite());
        // Total memory must be non-zero on any real host.
        assert!(metrics.memory_total > 0);
        // Used memory cannot exceed total memory.
        assert!(metrics.memory_usage <= metrics.memory_total);
    }

    #[tokio::test]
    async fn test_collect_metrics_over_period_zero() {
        let collector = SystemMetricsCollector::new();
        let metrics = collector.collect_metrics_over_period(0).await.unwrap();
        assert!(metrics.is_empty());
    }
}
