pub(crate) mod audit_repository;
pub(crate) mod collaboration_repository;
pub(crate) mod encrypted_sqlite_repository;
pub(crate) mod headless_session_repository;
pub(crate) mod macro_repository;
pub(crate) mod migrations;
pub(crate) mod session_repository;
pub(crate) mod share_link_repository;
pub(crate) mod share_repository;

use crate::shared::kernel::result::AppError;

/// Convert a `sqlx` failure into [`AppError::Database`] and log it, so every
/// repository surfaces database errors consistently and observably.
pub(crate) fn db_err(operation: &str, err: sqlx::Error) -> AppError {
    tracing::error!(operation, error = %err, "sqlite operation failed");
    AppError::Database(format!("{operation}: {err}"))
}

/// Build an in-memory SQLite pool for tests. `max_connections(1)` is required
/// because every connection to `sqlite::memory:` gets its own private database.
#[cfg(test)]
pub(crate) async fn test_pool() -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to open in-memory sqlite database");
    migrations::run_migrations(&pool)
        .await
        .expect("failed to run migrations on in-memory database");
    pool
}
