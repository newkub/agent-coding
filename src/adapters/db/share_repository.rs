use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::modules::session::domain::models::Session;
use crate::modules::share::domain::models::{ExportMetadata, ExportedSession};
use crate::modules::share::ports::ShareRepository;
use crate::shared::kernel::result::{AppError, AppResult};

/// SQLite implementation of `ShareRepository` for session export/import.
///
/// Sessions are stored as JSON blobs in the `shared_sessions` table so that
/// the full nested structure (messages, metadata, ...) round-trips losslessly
/// through export/import without requiring a relational schema for every field.
pub(crate) struct SqliteShareRepository {
    pool: SqlitePool,
}

impl SqliteShareRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Initialize the `shared_sessions` table. Idempotent.
    pub(crate) async fn init_table(&self) -> AppResult<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS shared_sessions (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    fn row_to_session(&self, row: sqlx::sqlite::SqliteRow) -> Result<Session, AppError> {
        let data: String = row.get("data");
        serde_json::from_str::<Session>(&data)
            .map_err(|e| AppError::State(format!("Failed to decode session: {}", e)))
    }
}

#[async_trait]
impl ShareRepository for SqliteShareRepository {
    async fn save(&self, session: &Session) -> AppResult<()> {
        let data = serde_json::to_string(session)
            .map_err(|e| AppError::State(format!("Failed to encode session: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO shared_sessions (id, name, data, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                data = excluded.data,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(session.id.as_str())
        .bind(&session.name)
        .bind(&data)
        .bind(session.created_at.to_rfc3339())
        .bind(session.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_name(&self, name: &str) -> AppResult<Option<Session>> {
        let row = sqlx::query("SELECT * FROM shared_sessions WHERE name = ? LIMIT 1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        row.map(|r| self.row_to_session(r)).transpose()
    }

    async fn export_to_json(&self, session_id: &str) -> AppResult<String> {
        let row = sqlx::query("SELECT * FROM shared_sessions WHERE id = ?")
            .bind(session_id)
            .fetch_optional(&self.pool)
            .await?;

        let row =
            row.ok_or_else(|| AppError::NotFound(format!("Session {} not found", session_id)))?;
        let session = self.row_to_session(row)?;

        let exported = ExportedSession::new(
            session,
            ExportMetadata {
                exported_by: None,
                application_version: Some(env!("CARGO_PKG_VERSION").to_string()),
                include_messages: true,
                include_context: true,
            },
        );

        exported
            .to_json()
            .map_err(|e| AppError::State(format!("Failed to serialize export: {}", e)))
    }

    async fn import_from_json(&self, json: &str) -> AppResult<Session> {
        let exported: ExportedSession = serde_json::from_str(json)
            .map_err(|e| AppError::State(format!("Invalid export JSON: {}", e)))?;

        if exported.version != "1.0" {
            return Err(AppError::State(format!(
                "Unsupported export version: {}",
                exported.version
            )));
        }

        let session = exported.session;
        self.save(&session).await?;
        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::session::domain::models::{Session, SessionId, SessionMetadata};
    use chrono::Utc;

    async fn setup() -> SqliteShareRepository {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let repo = SqliteShareRepository::new(pool);
        repo.init_table().await.unwrap();
        repo
    }

    fn sample_session(name: &str) -> Session {
        let now = Utc::now();
        let mut session = Session::create(
            SessionId::from_string(uuid::Uuid::new_v4().to_string()),
            name.to_string(),
            now,
            now,
        );
        session.metadata = SessionMetadata {
            provider: Some("test".to_string()),
            model: Some("gpt-4".to_string()),
            tags: vec!["export".to_string()],
            pinned: false,
        };
        session
    }

    #[tokio::test]
    async fn test_save_and_find_by_name() {
        let repo = setup().await;
        let session = sample_session("coding-session");
        repo.save(&session).await.unwrap();

        let found = repo.find_by_name("coding-session").await.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.id.as_str(), session.id.as_str());
        assert_eq!(found.name, "coding-session");
        assert_eq!(found.metadata.provider.as_deref(), Some("test"));
    }

    #[tokio::test]
    async fn test_find_by_name_missing() {
        let repo = setup().await;
        let found = repo.find_by_name("nope").await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_save_upsert() {
        let repo = setup().await;
        let mut session = sample_session("upsert-session");
        repo.save(&session).await.unwrap();

        session.metadata.pinned = true;
        repo.save(&session).await.unwrap();

        let found = repo.find_by_name("upsert-session").await.unwrap().unwrap();
        assert!(found.metadata.pinned);
    }

    #[tokio::test]
    async fn test_export_to_json_roundtrip() {
        let repo = setup().await;
        let session = sample_session("export-me");
        let id = session.id.as_str().to_string();
        repo.save(&session).await.unwrap();

        let json = repo.export_to_json(&id).await.unwrap();
        assert!(json.contains("\"version\": \"1.0\""));
        assert!(json.contains("export-me"));
    }

    #[tokio::test]
    async fn test_export_to_json_missing() {
        let repo = setup().await;
        let result = repo.export_to_json("missing-id").await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_import_from_json() {
        let repo = setup().await;
        let session = sample_session("import-source");
        let exported = ExportedSession::new(session.clone(), ExportMetadata::default());
        let json = exported.to_json().unwrap();

        let imported = repo.import_from_json(&json).await.unwrap();
        assert_eq!(imported.id.as_str(), session.id.as_str());
        assert_eq!(imported.name, "import-source");

        // Verify persisted
        let found = repo.find_by_name("import-source").await.unwrap().unwrap();
        assert_eq!(found.id.as_str(), session.id.as_str());
    }

    #[tokio::test]
    async fn test_import_from_json_invalid_version() {
        let repo = setup().await;
        let bad_json = serde_json::json!({
            "version": "2.0",
            "exported_at": Utc::now().to_rfc3339(),
            "session": sample_session("bad"),
            "metadata": ExportMetadata::default()
        })
        .to_string();

        let result = repo.import_from_json(&bad_json).await;
        assert!(matches!(result, Err(AppError::State(_))));
    }

    #[tokio::test]
    async fn test_import_from_json_invalid_json() {
        let repo = setup().await;
        let result = repo.import_from_json("not json").await;
        assert!(matches!(result, Err(AppError::State(_))));
    }
}
