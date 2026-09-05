use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::macros::domain::models::{Macro, MacroId};
use crate::modules::macros::ports::MacroRepository;
use crate::shared::kernel::result::{AppError, AppResult};

/// SQLite-backed macro repository.
///
/// The full `Macro` is stored as a JSON blob in `data` (with `steps` also
/// broken out for inspection). The `recording` flag distinguishes an active
/// recording from a finished macro so `finish_recording` can atomically end a
/// recording, matching the in-memory two-map semantics.
pub(crate) struct SqliteMacroRepository {
    pool: SqlitePool,
}

impl SqliteMacroRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn row_to_macro(row: &sqlx::sqlite::SqliteRow) -> AppResult<Macro> {
        let data: String = row.get("data");
        serde_json::from_str::<Macro>(&data)
            .map_err(|e| AppError::Database(format!("failed to decode macro row: {e}")))
    }

    async fn upsert(&self, macro_def: &Macro, recording: bool) -> AppResult<()> {
        let data = serde_json::to_string(macro_def)
            .map_err(|e| AppError::Database(format!("failed to encode macro: {e}")))?;
        let steps = serde_json::to_string(&macro_def.steps)
            .map_err(|e| AppError::Database(format!("failed to encode macro steps: {e}")))?;

        // `save` must not disturb an in-progress recording; `save_recording`
        // always (re)marks the macro as actively recording.
        let conflict_update = if recording { ", recording = 1" } else { "" };
        let sql = format!(
            "INSERT INTO macros \
                 (id, name, steps, usage_count, recording, created_at, updated_at, data) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(id) DO UPDATE SET \
                 name = excluded.name, \
                 steps = excluded.steps, \
                 usage_count = excluded.usage_count, \
                 updated_at = excluded.updated_at, \
                 data = excluded.data{conflict_update}"
        );

        sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(macro_def.id.as_str())
            .bind(&macro_def.name)
            .bind(&steps)
            .bind(i64::from(macro_def.usage_count))
            .bind(i32::from(recording))
            .bind(macro_def.created_at.to_rfc3339())
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(&data)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("macros.save", e))?;

        Ok(())
    }
}

#[async_trait]
impl MacroRepository for SqliteMacroRepository {
    async fn save(&self, macro_def: &Macro) -> AppResult<()> {
        self.upsert(macro_def, false).await
    }

    async fn save_recording(&self, macro_def: &Macro) -> AppResult<()> {
        self.upsert(macro_def, true).await
    }

    async fn finish_recording(&self, id: &MacroId) -> AppResult<Option<Macro>> {
        // Atomically clear the recording flag; zero affected rows means the
        // recording is not active (either finished or unknown).
        let result = sqlx::query("UPDATE macros SET recording = 0 WHERE id = ? AND recording = 1")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("macros.finish_recording", e))?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        self.find_by_id(id).await
    }

    async fn find_by_id(&self, id: &MacroId) -> AppResult<Option<Macro>> {
        let row = sqlx::query("SELECT data FROM macros WHERE id = ?")
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("macros.find_by_id", e))?;

        row.as_ref().map(Self::row_to_macro).transpose()
    }

    async fn find_all(&self) -> AppResult<Vec<Macro>> {
        let rows = sqlx::query("SELECT data FROM macros ORDER BY created_at")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("macros.find_all", e))?;

        rows.iter().map(Self::row_to_macro).collect()
    }

    async fn delete(&self, id: &MacroId) -> AppResult<()> {
        sqlx::query("DELETE FROM macros WHERE id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("macros.delete", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;
    use chrono::Utc;

    async fn repo() -> SqliteMacroRepository {
        SqliteMacroRepository::new(test_pool().await)
    }

    fn sample_macro(id: &str, name: &str) -> Macro {
        Macro::create(
            MacroId::from_string(id.to_string()),
            name.to_string(),
            String::new(),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn test_save_and_find() {
        let repo = repo().await;
        repo.save(&sample_macro("m1", "build")).await.unwrap();

        let found = repo
            .find_by_id(&MacroId::from_string("m1".to_string()))
            .await
            .unwrap();
        assert_eq!(found.unwrap().name, "build");
        assert_eq!(repo.find_all().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_recording_lifecycle() {
        let repo = repo().await;
        let macro_def = sample_macro("m1", "rec");
        repo.save_recording(&macro_def).await.unwrap();

        let finished = repo.finish_recording(&macro_def.id).await.unwrap();
        assert!(finished.is_some());
        // A second finish returns None - the recording is no longer active.
        assert!(repo
            .finish_recording(&macro_def.id)
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn test_delete() {
        let repo = repo().await;
        let macro_def = sample_macro("m1", "rec");
        repo.save_recording(&macro_def).await.unwrap();
        repo.delete(&macro_def.id).await.unwrap();

        assert!(repo.find_by_id(&macro_def.id).await.unwrap().is_none());
        assert!(repo
            .finish_recording(&macro_def.id)
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn test_save_preserves_steps() {
        let repo = repo().await;
        let mut macro_def = sample_macro("m1", "steps");
        macro_def.add_step(crate::modules::macros::domain::models::MacroStep::Input {
            text: "hello".to_string(),
        });
        repo.save(&macro_def).await.unwrap();

        let found = repo.find_by_id(&macro_def.id).await.unwrap().unwrap();
        assert_eq!(found.step_count(), 1);
    }
}
