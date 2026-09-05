// Database handler - online SQLite backup, restore, and integrity checks.

use std::fs;
use std::path::{Path, PathBuf};

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Row, SqlitePool};

use crate::presentation::cli::commands::DatabaseCommands;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(command: DatabaseCommands) -> AppResult<()> {
    match command {
        DatabaseCommands::Backup { path } => backup_database(PathBuf::from(path)).await,
        DatabaseCommands::Restore { path, yes } => restore_database(PathBuf::from(path), yes).await,
        DatabaseCommands::Verify => verify_database(&DIContainer::database_path()).await,
    }
}

async fn backup_database(path: PathBuf) -> AppResult<()> {
    let source = DIContainer::database_path();
    if !source.exists() {
        return Err(AppError::NotFound(format!(
            "database does not exist: {}",
            source.display()
        )));
    }
    if same_file(&source, &path)? {
        return Err(AppError::ValidationError(
            "backup path must differ from the active database path".to_string(),
        ));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let options = SqliteConnectOptions::new().filename(&source);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::query("VACUUM INTO ?")
        .bind(path.to_string_lossy().to_string())
        .execute(&pool)
        .await?;
    pool.close().await;

    verify_database(&path).await?;
    output::print_info(&format!(
        "Database backed up to {} and verified",
        path.display()
    ));
    Ok(())
}

async fn restore_database(path: PathBuf, yes: bool) -> AppResult<()> {
    if !path.exists() {
        return Err(AppError::NotFound(format!(
            "backup does not exist: {}",
            path.display()
        )));
    }
    let target = DIContainer::database_path();
    if same_file(&path, &target)? {
        return Err(AppError::ValidationError(
            "restore source must differ from the active database path".to_string(),
        ));
    }
    if target.exists() && !yes {
        return Err(AppError::PermissionDenied(
            "restore would overwrite the active database; rerun with --yes".to_string(),
        ));
    }

    verify_database(&path).await?;
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(&path, &target)?;
    verify_database(&target).await?;
    output::print_info(&format!(
        "Database restored from {} to {} and verified",
        path.display(),
        target.display()
    ));
    Ok(())
}

async fn verify_database(path: &Path) -> AppResult<()> {
    if !path.exists() {
        return Err(AppError::NotFound(format!(
            "database does not exist: {}",
            path.display()
        )));
    }
    let options = SqliteConnectOptions::new().filename(path).read_only(true);
    let pool = SqlitePool::connect_with(options).await?;
    let result = sqlx::query("PRAGMA integrity_check").fetch_one(&pool).await;
    pool.close().await;
    let row = result?;
    let integrity: String = row.get(0);
    if integrity == "ok" {
        output::print_info(&format!(
            "Database integrity check passed: {}",
            path.display()
        ));
        Ok(())
    } else {
        Err(AppError::Database(format!(
            "integrity check failed for {}: {}",
            path.display(),
            integrity
        )))
    }
}

fn same_file(left: &Path, right: &Path) -> AppResult<bool> {
    let left = absolute_path(left)?;
    let right = absolute_path(right)?;
    Ok(left == right)
}

fn absolute_path(path: &Path) -> AppResult<PathBuf> {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    Ok(path.normalize())
}

trait NormalizePath {
    fn normalize(&self) -> PathBuf;
}

impl NormalizePath for PathBuf {
    fn normalize(&self) -> PathBuf {
        self.components().collect()
    }
}
