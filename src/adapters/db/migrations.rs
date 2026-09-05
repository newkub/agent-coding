//! Embedded SQLite migrations.
//!
//! Migrations are applied in order and recorded in the `_migrations` table so
//! each one runs exactly once. Migration SQL may contain multiple statements;
//! it is executed through `sqlx::raw_sql` which forwards the whole script to
//! SQLite.

use sqlx::SqlitePool;

use crate::shared::kernel::result::AppResult;

/// Embedded migrations, applied in declaration order.
const MIGRATIONS: &[(&str, &str)] = &[
    (
        "0001_sessions",
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            model TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            data TEXT NOT NULL
        )
        "#,
    ),
    (
        "0002_audit",
        r#"
        CREATE TABLE IF NOT EXISTS audit_entries (
            id TEXT PRIMARY KEY,
            timestamp TEXT NOT NULL,
            category TEXT NOT NULL,
            actor_id TEXT NOT NULL,
            resource_id TEXT NOT NULL,
            data TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_audit_entries_timestamp
            ON audit_entries (timestamp);
        CREATE INDEX IF NOT EXISTS idx_audit_entries_category
            ON audit_entries (category);
        "#,
    ),
    (
        "0003_collaboration",
        r#"
        CREATE TABLE IF NOT EXISTS collaboration_sessions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            status TEXT NOT NULL,
            session_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            data TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS collaboration_messages (
            id TEXT PRIMARY KEY,
            collaboration_id TEXT NOT NULL,
            sender_id TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            data TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_collaboration_messages_session
            ON collaboration_messages (collaboration_id);
        "#,
    ),
    (
        "0004_macros",
        r#"
        CREATE TABLE IF NOT EXISTS macros (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            steps TEXT NOT NULL,
            usage_count INTEGER NOT NULL DEFAULT 0,
            recording INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            data TEXT NOT NULL
        )
        "#,
    ),
    (
        "0005_headless_sessions",
        r#"
        CREATE TABLE IF NOT EXISTS headless_sessions (
            id TEXT PRIMARY KEY,
            data TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_accessed TEXT NOT NULL
        )
        "#,
    ),
    (
        "0006_share",
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
        );
        CREATE TABLE IF NOT EXISTS shared_sessions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    ),
];

/// Run all embedded migrations against the pool. Idempotent.
pub(crate) async fn run_migrations(pool: &SqlitePool) -> AppResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            id TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    for (id, sql) in MIGRATIONS {
        apply_migration(pool, id, sql).await?;
    }

    Ok(())
}

/// Apply a single migration if it has not been recorded in `_migrations`.
///
/// `sql` must be `'static` because `sqlx::raw_sql` requires an owned or
/// static statement batch.
pub(crate) async fn apply_migration(
    pool: &SqlitePool,
    id: &str,
    sql: &'static str,
) -> AppResult<()> {
    let applied = sqlx::query("SELECT id FROM _migrations WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .is_some();

    if applied {
        return Ok(());
    }

    sqlx::raw_sql(sql).execute(pool).await?;
    sqlx::query("INSERT INTO _migrations (id, applied_at) VALUES (?, ?)")
        .bind(id)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(pool)
        .await?;

    tracing::info!(migration = id, "applied sqlite migration");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn test_pool() -> SqlitePool {
        SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_run_migrations_is_idempotent() {
        let pool = test_pool().await;
        run_migrations(&pool).await.unwrap();
        run_migrations(&pool).await.unwrap();

        let rows = sqlx::query("SELECT id FROM _migrations ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(rows.len(), MIGRATIONS.len());
    }

    #[tokio::test]
    async fn test_apply_migration_skips_applied() {
        let pool = test_pool().await;
        sqlx::query("CREATE TABLE _migrations (id TEXT PRIMARY KEY, applied_at TEXT NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();

        let sql = "CREATE TABLE widgets (id TEXT PRIMARY KEY)";
        apply_migration(&pool, "0001_widgets", sql).await.unwrap();
        // Second call must not fail even though the table already exists.
        apply_migration(&pool, "0001_widgets", sql).await.unwrap();

        sqlx::query("INSERT INTO widgets (id) VALUES ('w1')")
            .execute(&pool)
            .await
            .unwrap();
    }
}
