use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::headless::ports::HeadlessSessionManager;
use crate::shared::kernel::result::AppError;

/// In-memory session manager for headless mode
pub(crate) struct InMemorySessionManager {
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
}

#[derive(Clone)]
struct SessionData {
    id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    last_accessed: chrono::DateTime<chrono::Utc>,
    data: HashMap<String, String>,
}

impl InMemorySessionManager {
    pub(crate) fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemorySessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HeadlessSessionManager for InMemorySessionManager {
    async fn create_session(&self) -> Result<String, AppError> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session_data = SessionData {
            id: session_id.clone(),
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            data: HashMap::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session_data);

        Ok(session_id)
    }

    async fn load_session(&self, session_id: &str) -> Result<(), AppError> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            session.last_accessed = chrono::Utc::now();
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Session {} not found",
                session_id
            )))
        }
    }

    async fn save_session(&self, session_id: &str) -> Result<(), AppError> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            session.last_accessed = chrono::Utc::now();
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Session {} not found",
                session_id
            )))
        }
    }

    async fn list_sessions(&self) -> Result<Vec<String>, AppError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.keys().cloned().collect())
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), AppError> {
        let mut sessions = self.sessions.write().await;

        if sessions.remove(session_id).is_some() {
            Ok(())
        } else {
            Err(AppError::NotFound(format!(
                "Session {} not found",
                session_id
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_session() {
        let manager = InMemorySessionManager::new();
        let session_id = manager.create_session().await.unwrap();
        assert!(!session_id.is_empty());
    }

    #[tokio::test]
    async fn test_list_sessions() {
        let manager = InMemorySessionManager::new();
        manager.create_session().await.unwrap();
        manager.create_session().await.unwrap();

        let sessions = manager.list_sessions().await.unwrap();
        assert_eq!(sessions.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_session() {
        let manager = InMemorySessionManager::new();
        let session_id = manager.create_session().await.unwrap();

        manager.delete_session(&session_id).await.unwrap();
        let sessions = manager.list_sessions().await.unwrap();
        assert_eq!(sessions.len(), 0);
    }
}
