use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::ui::domain::models::{NoteItem, SnippetItem};
use crate::modules::ui::ports::UiContentRepository;
use crate::shared::kernel::result::AppResult;

/// SQLite-backed storage for UI-owned notes and snippets.
pub(crate) struct SqliteUiContentRepository {
    pool: SqlitePool,
}

impl SqliteUiContentRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UiContentRepository for SqliteUiContentRepository {
    async fn list_notes(&self) -> AppResult<Vec<NoteItem>> {
        let rows = sqlx::query("SELECT id, title, content FROM ui_notes ORDER BY created_at ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("list UI notes", e))?;
        Ok(rows
            .iter()
            .map(|row| NoteItem {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
            })
            .collect())
    }

    async fn replace_notes(&self, notes: &[NoteItem]) -> AppResult<()> {
        sqlx::query("DELETE FROM ui_notes")
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("clear UI notes", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        for note in notes {
            let id = if note.id.is_empty() {
                uuid::Uuid::new_v4().to_string()
            } else {
                note.id.clone()
            };
            sqlx::query(
                "INSERT INTO ui_notes (id, title, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(id)
            .bind(&note.title)
            .bind(&note.content)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("insert UI note", e))?;
        }
        Ok(())
    }

    async fn list_snippets(&self) -> AppResult<Vec<SnippetItem>> {
        let rows =
            sqlx::query("SELECT id, name, language, code FROM ui_snippets ORDER BY created_at ASC")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| db_err("list UI snippets", e))?;
        Ok(rows
            .iter()
            .map(|row| SnippetItem {
                id: row.get("id"),
                name: row.get("name"),
                language: row.get("language"),
                code: row.get("code"),
            })
            .collect())
    }

    async fn replace_snippets(&self, snippets: &[SnippetItem]) -> AppResult<()> {
        sqlx::query("DELETE FROM ui_snippets")
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("clear UI snippets", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        for snippet in snippets {
            let id = if snippet.id.is_empty() {
                uuid::Uuid::new_v4().to_string()
            } else {
                snippet.id.clone()
            };
            sqlx::query(
                "INSERT INTO ui_snippets (id, name, language, code, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(id)
            .bind(&snippet.name)
            .bind(&snippet.language)
            .bind(&snippet.code)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("insert UI snippet", e))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;

    #[tokio::test]
    async fn test_notes_and_snippets_round_trip() {
        let repo = SqliteUiContentRepository::new(test_pool().await);
        repo.replace_notes(&[NoteItem {
            id: "note-1".to_string(),
            title: "Note".to_string(),
            content: "Content".to_string(),
        }])
        .await
        .unwrap();
        repo.replace_snippets(&[SnippetItem {
            id: "snippet-1".to_string(),
            name: "Snippet".to_string(),
            language: "rust".to_string(),
            code: "fn main() {}".to_string(),
        }])
        .await
        .unwrap();

        assert_eq!(repo.list_notes().await.unwrap()[0].id, "note-1");
        assert_eq!(repo.list_snippets().await.unwrap()[0].id, "snippet-1");
    }
}
