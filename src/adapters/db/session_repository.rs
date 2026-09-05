use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::session::domain::models::{Session, SessionId};
use crate::modules::session::ports::SessionRepository;
use crate::shared::kernel::result::{AppError, AppResult};

/// SQLite-backed session repository.
///
/// The full `Session` (messages, metadata, tags, ...) is stored as a JSON blob
/// in `data` so it round-trips losslessly; `name`, `model`, `created_at` and
/// `updated_at` are kept as derived columns for filtering and ordering.
pub(crate) struct SqliteSessionRepository {
    pool: SqlitePool,
}

impl SqliteSessionRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn row_to_session(row: &sqlx::sqlite::SqliteRow) -> AppResult<Session> {
        let data: String = row.get("data");
        serde_json::from_str::<Session>(&data)
            .map_err(|e| AppError::Database(format!("failed to decode session row: {e}")))
    }
}

#[async_trait]
impl SessionRepository for SqliteSessionRepository {
    async fn save(&self, session: &Session) -> AppResult<()> {
        let data = serde_json::to_string(session)
            .map_err(|e| AppError::Database(format!("failed to encode session: {e}")))?;

        sqlx::query(
            r#"
            INSERT INTO sessions (id, name, model, created_at, updated_at, data)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                model = excluded.model,
                updated_at = excluded.updated_at,
                data = excluded.data
            "#,
        )
        .bind(session.id.as_str())
        .bind(&session.name)
        .bind(&session.metadata.model)
        .bind(session.created_at.to_rfc3339())
        .bind(session.updated_at.to_rfc3339())
        .bind(&data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("sessions.save", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &SessionId) -> AppResult<Option<Session>> {
        let row = sqlx::query("SELECT data FROM sessions WHERE id = ?")
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("sessions.find_by_id", e))?;

        row.as_ref().map(Self::row_to_session).transpose()
    }

    async fn find_all(&self) -> AppResult<Vec<Session>> {
        let rows = sqlx::query("SELECT data FROM sessions ORDER BY updated_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("sessions.find_all", e))?;

        rows.iter().map(Self::row_to_session).collect()
    }

    async fn delete(&self, id: &SessionId) -> AppResult<()> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("sessions.delete", e))?;
        Ok(())
    }

    async fn find_by_name(&self, name: &str) -> AppResult<Option<Session>> {
        let row = sqlx::query("SELECT data FROM sessions WHERE name = ? LIMIT 1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("sessions.find_by_name", e))?;

        row.as_ref().map(Self::row_to_session).transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;
    use chrono::Utc;

    async fn repo() -> SqliteSessionRepository {
        SqliteSessionRepository::new(test_pool().await)
    }

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
        let repo = repo().await;
        let session = sample("test");
        repo.save(&session).await.unwrap();

        let found = repo.find_by_id(&session.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "test");
    }

    #[tokio::test]
    async fn test_find_by_name() {
        let repo = repo().await;
        let session = sample("named");
        repo.save(&session).await.unwrap();

        let found = repo.find_by_name("named").await.unwrap();
        assert!(found.is_some());
    }

    #[tokio::test]
    async fn test_find_all_and_delete() {
        let repo = repo().await;
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

    #[tokio::test]
    async fn test_save_upsert_updates_messages() {
        let repo = repo().await;
        let mut session = sample("chat");
        repo.save(&session).await.unwrap();

        session.add_message(crate::modules::session::domain::models::Message::create(
            "m1".to_string(),
            crate::modules::session::domain::models::MessageRole::User,
            "hello".to_string(),
            Utc::now(),
        ));
        repo.save(&session).await.unwrap();

        let found = repo.find_by_id(&session.id).await.unwrap().unwrap();
        assert_eq!(found.message_count(), 1);
        assert_eq!(repo.find_all().await.unwrap().len(), 1);
    }
}
