use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::performance::domain::models::metrics::{
    PerformanceComparison, PerformanceMetrics, PerformanceSnapshot,
};
use crate::modules::performance::ports::SnapshotManager;
use crate::shared::kernel::result::AppError;

/// In-memory snapshot manager
pub struct InMemorySnapshotManager {
    snapshots: Arc<RwLock<HashMap<String, PerformanceSnapshot>>>,
}

impl InMemorySnapshotManager {
    pub(crate) fn new() -> Self {
        Self {
            snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemorySnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SnapshotManager for InMemorySnapshotManager {
    async fn create_snapshot(
        &self,
        name: String,
        metrics: PerformanceMetrics,
    ) -> Result<PerformanceSnapshot, AppError> {
        let snapshot = PerformanceSnapshot::new(name, metrics);
        let mut snapshots = self.snapshots.write().await;
        snapshots.insert(snapshot.id.clone(), snapshot.clone());
        Ok(snapshot)
    }

    async fn get_snapshot(&self, id: &str) -> Result<PerformanceSnapshot, AppError> {
        let snapshots = self.snapshots.read().await;
        snapshots
            .get(id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Snapshot {} not found", id)))
    }

    async fn list_snapshots(&self) -> Result<Vec<PerformanceSnapshot>, AppError> {
        let snapshots = self.snapshots.read().await;
        Ok(snapshots.values().cloned().collect())
    }

    async fn delete_snapshot(&self, id: &str) -> Result<(), AppError> {
        let mut snapshots = self.snapshots.write().await;
        if snapshots.remove(id).is_some() {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Snapshot {} not found", id)))
        }
    }

    async fn compare_snapshots(
        &self,
        id1: &str,
        id2: &str,
    ) -> Result<PerformanceComparison, AppError> {
        let snapshot1 = self.get_snapshot(id1).await?;
        let snapshot2 = self.get_snapshot(id2).await?;

        Ok(PerformanceComparison {
            cpu_diff: snapshot2.metrics.cpu_usage - snapshot1.metrics.cpu_usage,
            memory_diff: snapshot2.metrics.memory_usage as i64
                - snapshot1.metrics.memory_usage as i64,
            response_time_diff: snapshot2.metrics.response_time_ms as i64
                - snapshot1.metrics.response_time_ms as i64,
            throughput_diff: snapshot2.metrics.throughput - snapshot1.metrics.throughput,
            error_rate_diff: snapshot2.metrics.error_rate - snapshot1.metrics.error_rate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_snapshot() {
        let manager = InMemorySnapshotManager::new();
        let metrics =
            crate::modules::performance::domain::models::metrics::PerformanceMetrics::new();
        let snapshot = manager
            .create_snapshot("Test".to_string(), metrics)
            .await
            .unwrap();
        assert_eq!(snapshot.name, "Test");
    }

    #[tokio::test]
    async fn test_list_snapshots() {
        let manager = InMemorySnapshotManager::new();
        let metrics =
            crate::modules::performance::domain::models::metrics::PerformanceMetrics::new();
        manager
            .create_snapshot("Test1".to_string(), metrics.clone())
            .await
            .unwrap();
        manager
            .create_snapshot("Test2".to_string(), metrics)
            .await
            .unwrap();

        let snapshots = manager.list_snapshots().await.unwrap();
        assert_eq!(snapshots.len(), 2);
    }
}
