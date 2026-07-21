use crate::modules::metrics::domain::models::{PerformanceMetric, TokenUsage};
use crate::shared::kernel::result::AppResult;
use async_trait::async_trait;

/// Port: Token Usage Repository
#[async_trait]
pub(crate) trait TokenUsageRepository: Send + Sync {
    async fn save(&self, usage: &TokenUsage) -> AppResult<()>;
    async fn find_by_id(&self, id: &str) -> AppResult<Option<TokenUsage>>;
    async fn find_by_session(&self, session_id: &str) -> AppResult<Vec<TokenUsage>>;
    async fn find_all(&self) -> AppResult<Vec<TokenUsage>>;
    async fn get_total_cost(&self) -> AppResult<f64>;
}

/// Port: Metrics Repository
#[async_trait]
pub(crate) trait MetricsRepository: Send + Sync {
    async fn save_metric(&self, metric: &PerformanceMetric) -> AppResult<()>;
    async fn get_by_type(
        &self,
        metric_type: crate::modules::metrics::domain::models::MetricType,
    ) -> AppResult<Vec<PerformanceMetric>>;
    async fn get_all(&self) -> AppResult<Vec<PerformanceMetric>>;
    async fn delete_older_than(&self, timestamp: chrono::DateTime<chrono::Utc>)
        -> AppResult<usize>;
}
