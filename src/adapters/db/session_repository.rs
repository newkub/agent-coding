use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::session::domain::models::{Session, SessionId};
use crate::modules::session::ports::SessionRepository;
use crate::shared::kernel::result::AppResult;

/// In-memory session repository for fast startup and CLI wiring.
pub(crate) struct InMemorySessionRepository {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
}

impl InMemorySessionRepository {
    pub(crate) fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemorySessionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SessionRepository for InMemorySessionRepository {
    async fn save(&self, session: &Session) -> AppResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &SessionId) -> AppResult<Option<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(id).cloned())
    }

    async fn find_all(&self) -> AppResult<Vec<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.values().cloned().collect())
    }

    async fn delete(&self, id: &SessionId) -> AppResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(id);
        Ok(())
    }

    async fn find_by_name(&self, name: &str) -> AppResult<Option<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.values().find(|s| s.name == name).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample(name: &str) -> Session {
        Session::create(
            SessionId::from_string(uuid::Uuid::new_v4().to_string()),
            name.to_string(),
            Utc::now(),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn test_save_and_find_by_id() {
        let repo = InMemorySessionRepository::new();
        let session = sample("test");
        repo.save(&session).await.unwrap();

        let found = repo.find_by_id(&session.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "test");
    }

    #[tokio::test]
    async fn test_find_by_name() {
        let repo = InMemorySessionRepository::new();
        let session = sample("named");
        repo.save(&session).await.unwrap();

        let found = repo.find_by_name("named").await.unwrap();
        assert!(found.is_some());
    }

    #[tokio::test]
    async fn test_find_all_and_delete() {
        let repo = InMemorySessionRepository::new();
        let a = sample("a");
        let b = sample("b");
        repo.save(&a).await.unwrap();
        repo.save(&b).await.unwrap();

        let all = repo.find_all().await.unwrap();
        assert_eq!(all.len(), 2);

        repo.delete(&a.id).await.unwrap();
        let all = repo.find_all().await.unwrap();
        assert_eq!(all.len(), 1);
    }
}
