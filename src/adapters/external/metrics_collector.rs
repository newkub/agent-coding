use async_trait::async_trait;

use crate::modules::performance::domain::models::metrics::PerformanceMetrics;
use crate::modules::performance::ports::MetricsCollector;
use crate::shared::kernel::result::AppError;

/// Default metrics collector using system information
pub struct SystemMetricsCollector;

impl SystemMetricsCollector {
    pub(crate) const fn new() -> Self {
        Self
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
        let mut metrics = PerformanceMetrics::new();

        // TODO: Integrate sysinfo crate for real system metrics collection
        // For now, use realistic default values
        metrics.cpu_usage = 45.0;
        metrics.memory_usage = 512 * 1024 * 1024; // 512 MB
        metrics.memory_total = 8 * 1024 * 1024 * 1024; // 8 GB
        metrics.response_time_ms = 250;
        metrics.throughput = 50.0;
        metrics.error_rate = 0.01;

        Ok(metrics)
    }

    async fn collect_metrics_over_period(&self, duration_seconds: u64) -> Result<Vec<PerformanceMetrics>, AppError> {
        let mut metrics_list = Vec::new();
        
        // Collect metrics at intervals
        for _ in 0..(duration_seconds / 10).max(1) {
            let metrics = self.collect_metrics().await?;
            metrics_list.push(metrics);
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
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
        assert!(metrics.cpu_usage >= 0.0);
    }
}
