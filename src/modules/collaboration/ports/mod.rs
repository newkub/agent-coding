use async_trait::async_trait;
use crate::modules::collaboration::domain::models::{CollaborationSession, SharedMessage, CollaborationId, ParticipantId, CursorPosition};
use crate::shared::kernel::result::AppResult;

/// Port: Collaboration Repository
#[async_trait]
pub(crate) trait CollaborationRepository: Send + Sync {
    async fn save(&self, session: &CollaborationSession) -> AppResult<()>;
    async fn find_by_id(&self, id: &CollaborationId) -> AppResult<Option<CollaborationSession>>;
    async fn find_active(&self) -> AppResult<Vec<CollaborationSession>>;
    async fn delete(&self, id: &CollaborationId) -> AppResult<()>;
    
    // Messages
    async fn save_message(&self, message: &SharedMessage) -> AppResult<()>;
    async fn get_messages(&self, collaboration_id: &CollaborationId) -> AppResult<Vec<SharedMessage>>;
}

/// Port: Real-time Collaboration
#[async_trait]
pub(crate) trait RealtimeCollaboration: Send + Sync {
    async fn broadcast_cursor(&self, collaboration_id: &CollaborationId, participant_id: &ParticipantId, position: CursorPosition) -> AppResult<()>;
    async fn broadcast_message(&self, message: &SharedMessage) -> AppResult<()>;
    async fn subscribe(&self, collaboration_id: &CollaborationId) -> AppResult<()>;
    async fn unsubscribe(&self, collaboration_id: &CollaborationId) -> AppResult<()>;
}