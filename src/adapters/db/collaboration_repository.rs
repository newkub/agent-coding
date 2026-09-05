use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::collaboration::domain::models::{
    CollaborationId, CollaborationSession, CollaborationStatus, SharedMessage,
};
use crate::modules::collaboration::ports::CollaborationRepository;
use crate::shared::kernel::result::{AppError, AppResult};

/// SQLite-backed collaboration repository.
///
/// Sessions and messages are stored as JSON blobs in `data`; `status` and
/// `collaboration_id` are derived columns for `find_active` and per-session
/// message lookups.
pub(crate) struct SqliteCollaborationRepository {
    pool: SqlitePool,
}

impl SqliteCollaborationRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    const fn status_str(status: CollaborationStatus) -> &'static str {
        match status {
            CollaborationStatus::Active => "active",
            CollaborationStatus::Paused => "paused",
            CollaborationStatus::Ended => "ended",
        }
    }

    fn row_to_session(row: &sqlx::sqlite::SqliteRow) -> AppResult<CollaborationSession> {
        let data: String = row.get("data");
        serde_json::from_str::<CollaborationSession>(&data).map_err(|e| {
            AppError::Database(format!("failed to decode collaboration session row: {e}"))
        })
    }

    fn row_to_message(row: &sqlx::sqlite::SqliteRow) -> AppResult<SharedMessage> {
        let data: String = row.get("data");
        serde_json::from_str::<SharedMessage>(&data).map_err(|e| {
            AppError::Database(format!("failed to decode collaboration message row: {e}"))
        })
    }
}

#[async_trait]
impl CollaborationRepository for SqliteCollaborationRepository {
    async fn save(&self, session: &CollaborationSession) -> AppResult<()> {
        let data = serde_json::to_string(session).map_err(|e| {
            AppError::Database(format!("failed to encode collaboration session: {e}"))
        })?;

        sqlx::query(
            r#"
            INSERT INTO collaboration_sessions
                (id, name, status, session_id, created_at, data)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                status = excluded.status,
                session_id = excluded.session_id,
                data = excluded.data
            "#,
        )
        .bind(session.id.as_str())
        .bind(&session.name)
        .bind(Self::status_str(session.status))
        .bind(&session.session_id)
        .bind(session.created_at.to_rfc3339())
        .bind(&data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("collaboration_sessions.save", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &CollaborationId) -> AppResult<Option<CollaborationSession>> {
        let row = sqlx::query("SELECT data FROM collaboration_sessions WHERE id = ?")
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("collaboration_sessions.find_by_id", e))?;

        row.as_ref().map(Self::row_to_session).transpose()
    }

    async fn find_active(&self) -> AppResult<Vec<CollaborationSession>> {
        let rows = sqlx::query(
            "SELECT data FROM collaboration_sessions WHERE status = 'active' ORDER BY created_at",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| db_err("collaboration_sessions.find_active", e))?;

        rows.iter().map(Self::row_to_session).collect()
    }

    async fn delete(&self, id: &CollaborationId) -> AppResult<()> {
        sqlx::query("DELETE FROM collaboration_messages WHERE collaboration_id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("collaboration_messages.delete", e))?;
        sqlx::query("DELETE FROM collaboration_sessions WHERE id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("collaboration_sessions.delete", e))?;
        Ok(())
    }

    async fn save_message(&self, message: &SharedMessage) -> AppResult<()> {
        let data = serde_json::to_string(message).map_err(|e| {
            AppError::Database(format!("failed to encode collaboration message: {e}"))
        })?;

        sqlx::query(
            r#"
            INSERT INTO collaboration_messages
                (id, collaboration_id, sender_id, timestamp, data)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&message.id)
        .bind(message.collaboration_id.as_str())
        .bind(message.sender_id.as_str())
        .bind(message.timestamp.to_rfc3339())
        .bind(&data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("collaboration_messages.save", e))?;

        Ok(())
    }

    async fn get_messages(
        &self,
        collaboration_id: &CollaborationId,
    ) -> AppResult<Vec<SharedMessage>> {
        let rows = sqlx::query(
            "SELECT data FROM collaboration_messages WHERE collaboration_id = ? ORDER BY timestamp",
        )
        .bind(collaboration_id.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| db_err("collaboration_messages.get_messages", e))?;

        rows.iter().map(Self::row_to_message).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;
    use crate::modules::collaboration::domain::models::{
        Participant, ParticipantId, ParticipantRole, SharedMessageType,
    };
    use chrono::Utc;

    async fn repo() -> SqliteCollaborationRepository {
        SqliteCollaborationRepository::new(test_pool().await)
    }

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

    fn sample_message(id: &str, session: &CollaborationSession, content: &str) -> SharedMessage {
        SharedMessage {
            id: id.to_string(),
            collaboration_id: session.id.clone(),
            sender_id: ParticipantId::from_string("owner-1".to_string()),
            content: content.to_string(),
            timestamp: Utc::now(),
            message_type: SharedMessageType::Chat,
        }
    }

    #[tokio::test]
    async fn test_save_find_and_active() {
        let repo = repo().await;
        let session = sample_session("s1");
        repo.save(&session).await.unwrap();

        let found = repo
            .find_by_id(&CollaborationId::from_string("s1".to_string()))
            .await
            .unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().participants.len(), 1);

        let active = repo.find_active().await.unwrap();
        assert_eq!(active.len(), 1);
    }

    #[tokio::test]
    async fn test_ended_session_not_active() {
        let repo = repo().await;
        let mut session = sample_session("s1");
        session.status = CollaborationStatus::Ended;
        repo.save(&session).await.unwrap();

        assert!(repo.find_active().await.unwrap().is_empty());
        assert!(repo.find_by_id(&session.id).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_delete_removes_session_and_messages() {
        let repo = repo().await;
        let session = sample_session("s1");
        repo.save(&session).await.unwrap();
        repo.save_message(&sample_message("m1", &session, "hello"))
            .await
            .unwrap();

        repo.delete(&session.id).await.unwrap();
        assert!(repo.find_by_id(&session.id).await.unwrap().is_none());
        assert!(repo.get_messages(&session.id).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_messages_per_session() {
        let repo = repo().await;
        let session = sample_session("s1");
        repo.save(&session).await.unwrap();
        for i in 0..2 {
            repo.save_message(&sample_message(&format!("m{i}"), &session, "msg"))
                .await
                .unwrap();
        }

        let messages = repo.get_messages(&session.id).await.unwrap();
        assert_eq!(messages.len(), 2);
    }
}
