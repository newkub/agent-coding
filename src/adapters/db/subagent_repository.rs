use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::subagents::domain::models::subagent::{AgentType, Subagent, SubagentStatus};
use crate::modules::subagents::domain::operations::subagent_operations::generate_system_prompt;
use crate::modules::subagents::ports::SubagentManager;
use crate::shared::kernel::result::AppError;

/// SQLite-backed subagent catalog manager.
#[derive(Clone)]
pub(crate) struct SqliteSubagentManager {
    pool: SqlitePool,
}

impl SqliteSubagentManager {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Seed the built-in production subagent catalog once.
    pub(crate) async fn initialize_default_subagents(&self) -> Result<(), AppError> {
        let defaults = [
            (
                "code-reviewer",
                AgentType::CodeReviewer,
                "Reviews code for quality and best practices",
            ),
            (
                "bug-hunter",
                AgentType::BugHunter,
                "Identifies bugs and edge cases in code",
            ),
            (
                "refactorer",
                AgentType::Refactorer,
                "Suggests code refactoring improvements",
            ),
            (
                "documenter",
                AgentType::Documenter,
                "Generates comprehensive documentation",
            ),
            (
                "tester",
                AgentType::Tester,
                "Generates test cases and scenarios",
            ),
            (
                "security-auditor",
                AgentType::SecurityAuditor,
                "Identifies security vulnerabilities",
            ),
            (
                "performance-optimizer",
                AgentType::PerformanceOptimizer,
                "Analyzes and optimizes performance",
            ),
        ];

        let existing = self.list_subagents().await?;
        for (name, agent_type, description) in defaults {
            if existing.iter().any(|agent| agent.name == name) {
                continue;
            }
            let mut agent = Subagent::new(
                name.to_string(),
                agent_type.clone(),
                description.to_string(),
            );
            agent.capabilities = vec![description.to_string()];
            agent.config.system_prompt = generate_system_prompt(&agent_type);
            self.upsert(&agent).await?;
        }
        Ok(())
    }

    fn subagent_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Subagent, AppError> {
        let data: String = row.get("data");
        serde_json::from_str(&data)
            .map_err(|e| AppError::Database(format!("deserialize subagent failed: {e}")))
    }

    async fn upsert(&self, subagent: &Subagent) -> Result<(), AppError> {
        let mut stored = subagent.clone();
        stored.updated_at = chrono::Utc::now();
        let data = serde_json::to_string(&stored)
            .map_err(|e| AppError::Database(format!("serialize subagent failed: {e}")))?;

        sqlx::query(
            r#"
            INSERT INTO subagents
                (id, name, agent_type, status, created_at, updated_at, data)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                agent_type = excluded.agent_type,
                status = excluded.status,
                updated_at = excluded.updated_at,
                data = excluded.data
            "#,
        )
        .bind(&stored.id)
        .bind(&stored.name)
        .bind(format!("{:?}", stored.agent_type))
        .bind(format!("{:?}", stored.status))
        .bind(stored.created_at.to_rfc3339())
        .bind(stored.updated_at.to_rfc3339())
        .bind(data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("upsert subagent", e))
        .map(|_| ())
    }
}

#[async_trait]
impl SubagentManager for SqliteSubagentManager {
    async fn create_subagent(&self, subagent: Subagent) -> Result<Subagent, AppError> {
        self.upsert(&subagent).await?;
        Ok(subagent)
    }

    async fn get_subagent(&self, id: &str) -> Result<Subagent, AppError> {
        let row = sqlx::query("SELECT data FROM subagents WHERE id = ? OR name = ?")
            .bind(id)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("load subagent", e))?
            .ok_or_else(|| AppError::NotFound(format!("Subagent {id} not found")))?;
        Self::subagent_from_row(&row)
    }

    async fn list_subagents(&self) -> Result<Vec<Subagent>, AppError> {
        let rows = sqlx::query("SELECT data FROM subagents ORDER BY name ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("list subagents", e))?;
        rows.iter().map(Self::subagent_from_row).collect()
    }

    async fn update_subagent(&self, subagent: Subagent) -> Result<Subagent, AppError> {
        self.get_subagent(&subagent.id).await?;
        self.upsert(&subagent).await?;
        Ok(subagent)
    }

    async fn delete_subagent(&self, id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM subagents WHERE id = ? OR name = ?")
            .bind(id)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("delete subagent", e))?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Subagent {id} not found")));
        }
        Ok(())
    }

    async fn get_available_subagents(&self) -> Result<Vec<Subagent>, AppError> {
        Ok(self
            .list_subagents()
            .await?
            .into_iter()
            .filter(|agent| agent.status == SubagentStatus::Idle)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;

    #[tokio::test]
    async fn test_seed_and_get_by_name() {
        let manager = SqliteSubagentManager::new(test_pool().await);
        manager.initialize_default_subagents().await.unwrap();
        let agent = manager.get_subagent("code-reviewer").await.unwrap();
        assert_eq!(agent.name, "code-reviewer");
    }
}
