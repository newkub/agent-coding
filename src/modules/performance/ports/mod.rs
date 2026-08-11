use async_trait::async_trait;

use crate::modules::performance::domain::models::metrics::{
    OptimizationSuggestion, PerformanceMetrics, PerformanceSnapshot,
};
use crate::shared::kernel::result::AppError;

/// Port for performance metrics collection
#[async_trait]
pub(crate) trait MetricsCollector: Send + Sync {
    /// Collect current performance metrics
    async fn collect_metrics(&self) -> Result<PerformanceMetrics, AppError>;

    /// Collect metrics over a time period
    async fn collect_metrics_over_period(
        &self,
        duration_seconds: u64,
    ) -> Result<Vec<PerformanceMetrics>, AppError>;
}

/// Port for performance snapshot management
#[async_trait]
pub(crate) trait SnapshotManager: Send + Sync {
    /// Create a performance snapshot
    async fn create_snapshot(
        &self,
        name: String,
        metrics: PerformanceMetrics,
    ) -> Result<PerformanceSnapshot, AppError>;

    /// Get snapshot by ID
    async fn get_snapshot(&self, id: &str) -> Result<PerformanceSnapshot, AppError>;

    /// List all snapshots
    async fn list_snapshots(&self) -> Result<Vec<PerformanceSnapshot>, AppError>;

    /// Delete snapshot
    async fn delete_snapshot(&self, id: &str) -> Result<(), AppError>;

    /// Compare snapshots
    async fn compare_snapshots(
        &self,
        id1: &str,
        id2: &str,
    ) -> Result<crate::modules::performance::domain::models::metrics::PerformanceComparison, AppError>;
}

/// Port for optimization suggestion management
#[async_trait]
pub(crate) trait OptimizationManager: Send + Sync {
    /// Generate optimization suggestions from metrics
    async fn generate_suggestions(
        &self,
        metrics: &PerformanceMetrics,
    ) -> Result<Vec<OptimizationSuggestion>, AppError>;

    /// Apply an optimization suggestion
    async fn apply_suggestion(&self, suggestion_id: &str) -> Result<(), AppError>;

    /// List all suggestions
    async fn list_suggestions(&self) -> Result<Vec<OptimizationSuggestion>, AppError>;

    /// Dismiss a suggestion
    async fn dismiss_suggestion(&self, suggestion_id: &str) -> Result<(), AppError>;
}
