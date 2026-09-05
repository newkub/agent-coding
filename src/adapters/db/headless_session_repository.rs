use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::headless::ports::HeadlessSessionManager;
use crate::shared::kernel::result::AppError;

/// SQLite-backed headless session manager.
///
/// Session metadata is persisted in `headless_sessions` so headless sessions
/// survive restarts. `data` holds an opaque JSON key/value payload reserved
/// for session state; `last_accessed` is refreshed on load/save.
pub(crate) struct SqliteHeadlessSessionManager {
    pool: SqlitePool,
}

impl SqliteHeadlessSessionManager {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Bump `last_accessed`; returns `NotFound` when the session is unknown.
    async fn touch(&self, session_id: &str) -> Result<(), AppError> {
        let result = sqlx::query("UPDATE headless_sessions SET last_accessed = ? WHERE id = ?")
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(session_id)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("headless_sessions.touch", e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Session {session_id} not found"
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl HeadlessSessionManager for SqliteHeadlessSessionManager {
    async fn create_session(&self) -> Result<String, AppError> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO headless_sessions (id, data, created_at, last_accessed) VALUES (?, ?, ?, ?)",
        )
        .bind(&session_id)
        .bind("{}")
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("headless_sessions.create", e))?;

        Ok(session_id)
    }

    async fn load_session(&self, session_id: &str) -> Result<(), AppError> {
        self.touch(session_id).await
    }

    async fn save_session(&self, session_id: &str) -> Result<(), AppError> {
        self.touch(session_id).await
    }

    async fn list_sessions(&self) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query("SELECT id FROM headless_sessions ORDER BY created_at")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("headless_sessions.list", e))?;

        Ok(rows.iter().map(|row| row.get::<String, _>("id")).collect())
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM headless_sessions WHERE id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("headless_sessions.delete", e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Session {session_id} not found"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;

    async fn manager() -> SqliteHeadlessSessionManager {
        SqliteHeadlessSessionManager::new(test_pool().await)
    }

    #[tokio::test]
    async fn test_create_session() {
        let manager = manager().await;
        let session_id = manager.create_session().await.unwrap();
        assert!(!session_id.is_empty());
    }

    #[tokio::test]
    async fn test_list_sessions() {
        let manager = manager().await;
        manager.create_session().await.unwrap();
        manager.create_session().await.unwrap();

        let sessions = manager.list_sessions().await.unwrap();
        assert_eq!(sessions.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_session() {
        let manager = manager().await;
        let session_id = manager.create_session().await.unwrap();

        manager.delete_session(&session_id).await.unwrap();
        let sessions = manager.list_sessions().await.unwrap();
        assert_eq!(sessions.len(), 0);
    }

    #[tokio::test]
    async fn test_load_missing_session() {
        let manager = manager().await;
        let result = manager.load_session("missing").await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_load_existing_session() {
        let manager = manager().await;
        let session_id = manager.create_session().await.unwrap();
        assert!(manager.load_session(&session_id).await.is_ok());
        assert!(manager.save_session(&session_id).await.is_ok());
    }
}
