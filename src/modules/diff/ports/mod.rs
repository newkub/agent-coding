use async_trait::async_trait;
use crate::modules::diff::domain::models::DiffReview;
use crate::modules::diff::domain::events::DiffEvent;
use crate::shared::kernel::result::AppResult;

/// Port: Diff Parser
#[async_trait]
pub(crate) trait DiffParser: Send + Sync {
    async fn parse(&self, diff_text: &str) -> AppResult<DiffReview>;
}

/// Port: Diff Applicator
#[async_trait]
pub(crate) trait DiffApplicator: Send + Sync {
    async fn apply(&self, review: &DiffReview) -> AppResult<()>;
    async fn apply_hunk(&self, file_path: &str, hunk_id: &str) -> AppResult<()>;
}

/// Port: Diff Event Publisher
#[async_trait]
pub(crate) trait DiffEventPublisher: Send + Sync {
    async fn publish(&self, event: DiffEvent) -> AppResult<()>;
}