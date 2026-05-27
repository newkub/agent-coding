use async_trait::async_trait;
use crate::modules::session::domain::{models::{Session, SessionId}, events::SessionEvent};
use crate::shared::kernel::result::AppResult;

/// Port: Session Repository
#[async_trait]
pub(crate) trait SessionRepository: Send + Sync {
    async fn save(&self, session: &Session) -> AppResult<()>;
    async fn find_by_id(&self, id: &SessionId) -> AppResult<Option<Session>>;
    async fn find_all(&self) -> AppResult<Vec<Session>>;
    async fn delete(&self, id: &SessionId) -> AppResult<()>;
    async fn find_by_name(&self, name: &str) -> AppResult<Option<Session>>;
}

/// Port: Session Event Publisher
#[async_trait]
pub(crate) trait SessionEventPublisher: Send + Sync {
    async fn publish(&self, event: SessionEvent) -> AppResult<()>;
}