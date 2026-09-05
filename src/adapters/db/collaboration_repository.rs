use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::collaboration::domain::models::{
    CollaborationId, CollaborationSession, CollaborationStatus, SharedMessage,
};
use crate::modules::collaboration::ports::CollaborationRepository;
use crate::shared::kernel::result::AppResult;

/// In-memory collaboration repository for fast startup and CLI wiring.
///
/// Sessions are keyed by `CollaborationId`; each session owns an ordered
/// list of shared messages.
pub(crate) struct InMemoryCollaborationRepository {
    sessions: Arc<RwLock<HashMap<String, CollaborationSession>>>,
    messages: Arc<RwLock<HashMap<String, Vec<SharedMessage>>>>,
}

impl InMemoryCollaborationRepository {
    pub(crate) fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryCollaborationRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CollaborationRepository for InMemoryCollaborationRepository {
    async fn save(&self, session: &CollaborationSession) -> AppResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.as_str().to_string(), session.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &CollaborationId) -> AppResult<Option<CollaborationSession>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(id.as_str()).cloned())
    }

    async fn find_active(&self) -> AppResult<Vec<CollaborationSession>> {
        let sessions = self.sessions.read().await;
        let mut active: Vec<CollaborationSession> = sessions
            .values()
            .filter(|s| s.status == CollaborationStatus::Active)
            .cloned()
            .collect();
        active.sort_by_key(|s| s.created_at);
        Ok(active)
    }

    async fn delete(&self, id: &CollaborationId) -> AppResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(id.as_str());
        let mut messages = self.messages.write().await;
        messages.remove(id.as_str());
        Ok(())
    }

    async fn save_message(&self, message: &SharedMessage) -> AppResult<()> {
        let mut messages = self.messages.write().await;
        messages
            .entry(message.collaboration_id.as_str().to_string())
            .or_default()
            .push(message.clone());
        Ok(())
    }

    async fn get_messages(
        &self,
        collaboration_id: &CollaborationId,
    ) -> AppResult<Vec<SharedMessage>> {
        let messages = self.messages.read().await;
        Ok(messages
            .get(collaboration_id.as_str())
            .cloned()
            .unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::collaboration::domain::models::{
        Participant, ParticipantId, ParticipantRole, SharedMessageType,
    };
    use chrono::Utc;

    fn sample_session(id: &str) -> CollaborationSession {
        let owner = Participant {
            id: ParticipantId::from_string("owner-1".to_string()),
            name: "owner".to_string(),
            role: ParticipantRole::Owner,
            joined_at: Utc::now(),
            is_online: true,
            cursor_position: None,
        };
        CollaborationSession::create(
            CollaborationId::from_string(id.to_string()),
            format!("session-{id}"),
            owner,
            "ai-session".to_string(),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn test_save_find_and_active() {
        let repo = InMemoryCollaborationRepository::new();
        let session = sample_session("s1");
        repo.save(&session).await.unwrap();

        let found = repo
            .find_by_id(&CollaborationId::from_string("s1".to_string()))
            .await
            .unwrap();
        assert!(found.is_some());

        let active = repo.find_active().await.unwrap();
        assert_eq!(active.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_removes_session_and_messages() {
        let repo = InMemoryCollaborationRepository::new();
        let session = sample_session("s1");
        repo.save(&session).await.unwrap();
        repo.save_message(&SharedMessage {
            id: "m1".to_string(),
            collaboration_id: session.id.clone(),
            sender_id: ParticipantId::from_string("owner-1".to_string()),
            content: "hello".to_string(),
            timestamp: Utc::now(),
            message_type: SharedMessageType::Chat,
        })
        .await
        .unwrap();

        repo.delete(&session.id).await.unwrap();
        assert!(repo.find_by_id(&session.id).await.unwrap().is_none());
        assert!(repo.get_messages(&session.id).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_messages_per_session() {
        let repo = InMemoryCollaborationRepository::new();
        let session = sample_session("s1");
        repo.save(&session).await.unwrap();
        for i in 0..2 {
            repo.save_message(&SharedMessage {
                id: format!("m{i}"),
                collaboration_id: session.id.clone(),
                sender_id: ParticipantId::from_string("owner-1".to_string()),
                content: format!("msg {i}"),
                timestamp: Utc::now(),
                message_type: SharedMessageType::Chat,
            })
            .await
            .unwrap();
        }

        let messages = repo.get_messages(&session.id).await.unwrap();
        assert_eq!(messages.len(), 2);
    }
}
