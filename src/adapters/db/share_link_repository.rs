use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::modules::share::domain::models::share_link::ShareLink;
use crate::modules::share::ports::ShareLinkRepository;
use crate::shared::kernel::result::AppError;

/// SQLite implementation of ShareLinkRepository
#[derive(Clone)]
pub(crate) struct SqliteShareLinkRepository {
    pool: SqlitePool,
}

impl SqliteShareLinkRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Initialize the share_links table
    pub(crate) async fn init_table(&self) -> Result<(), AppError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS share_links (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                token TEXT UNIQUE NOT NULL,
                created_at TEXT NOT NULL,
                expires_at TEXT,
                access_count INTEGER NOT NULL DEFAULT 0,
                max_access INTEGER,
                is_active INTEGER NOT NULL DEFAULT 1,
                can_read INTEGER NOT NULL DEFAULT 1,
                can_write INTEGER NOT NULL DEFAULT 0,
                can_execute INTEGER NOT NULL DEFAULT 0,
                can_delete INTEGER NOT NULL DEFAULT 0
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Helper method to convert sqlx row to ShareLink
    fn row_to_share_link(&self, row: sqlx::sqlite::SqliteRow) -> ShareLink {
        use crate::modules::share::domain::models::share_link::SharePermissions;

        ShareLink {
            id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap_or_default(),
            session_id: Uuid::parse_str(&row.get::<String, _>("session_id")).unwrap_or_default(),
            token: row.get("token"),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
                .unwrap()
                .with_timezone(&chrono::Utc),
            expires_at: row.get::<Option<String>, _>("expires_at").and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|d| d.with_timezone(&chrono::Utc))
            }),
            access_count: row.get::<i64, _>("access_count") as u32,
            max_access: row.get::<Option<i64>, _>("max_access").map(|m| m as u32),
            is_active: row.get::<i32, _>("is_active") != 0,
            permissions: SharePermissions {
                can_read: row.get::<i32, _>("can_read") != 0,
                can_write: row.get::<i32, _>("can_write") != 0,
                can_execute: row.get::<i32, _>("can_execute") != 0,
                can_delete: row.get::<i32, _>("can_delete") != 0,
            },
        }
    }
}

#[async_trait]
impl ShareLinkRepository for SqliteShareLinkRepository {
    async fn save(&self, link: &ShareLink) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO share_links (
                id, session_id, token, created_at, expires_at,
                access_count, max_access, is_active,
                can_read, can_write, can_execute, can_delete
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(link.id.to_string())
        .bind(link.session_id.to_string())
        .bind(&link.token)
        .bind(link.created_at.to_rfc3339())
        .bind(link.expires_at.map(|d| d.to_rfc3339()))
        .bind(link.access_count as i64)
        .bind(link.max_access.map(|m| m as i64))
        .bind(link.is_active as i32)
        .bind(link.permissions.can_read as i32)
        .bind(link.permissions.can_write as i32)
        .bind(link.permissions.can_execute as i32)
        .bind(link.permissions.can_delete as i32)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ShareLink>, AppError> {
        let row = sqlx::query("SELECT * FROM share_links WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| self.row_to_share_link(r)))
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<ShareLink>, AppError> {
        let row = sqlx::query("SELECT * FROM share_links WHERE token = ?")
            .bind(token)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| self.row_to_share_link(r)))
    }

    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<ShareLink>, AppError> {
        let rows = sqlx::query("SELECT * FROM share_links WHERE session_id = ?")
            .bind(session_id.to_string())
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|r| self.row_to_share_link(r))
            .collect())
    }

    async fn update(&self, link: &ShareLink) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE share_links SET
                access_count = ?,
                is_active = ?
            WHERE id = ?
            "#,
        )
        .bind(link.access_count as i64)
        .bind(link.is_active as i32)
        .bind(link.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM share_links WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn deactivate_by_session_id(&self, session_id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE share_links SET is_active = 0 WHERE session_id = ?")
            .bind(session_id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
