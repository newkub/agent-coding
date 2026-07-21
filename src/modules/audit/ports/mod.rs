use crate::modules::audit::domain::models::AuditEntry;
use crate::shared::kernel::result::AppResult;
use async_trait::async_trait;

/// Port: Audit Repository
#[async_trait]
pub(crate) trait AuditRepository: Send + Sync {
    async fn save(&self, entry: &AuditEntry) -> AppResult<()>;
    async fn find_by_id(
        &self,
        id: &crate::modules::audit::domain::models::AuditId,
    ) -> AppResult<Option<AuditEntry>>;
    async fn query(
        &self,
        filters: crate::modules::audit::application::usecases::AuditQuery,
    ) -> AppResult<Vec<AuditEntry>>;
    async fn count(
        &self,
        filters: crate::modules::audit::application::usecases::AuditQuery,
    ) -> AppResult<usize>;
    async fn delete_older_than(&self, timestamp: chrono::DateTime<chrono::Utc>)
        -> AppResult<usize>;
}
