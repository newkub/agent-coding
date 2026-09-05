use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::automation::domain::models::issue_pr::AutomationWorkflow;
use crate::modules::automation::ports::AutomationWorkflowRepository;
use crate::shared::kernel::result::AppError;

/// SQLite-backed automation workflow repository.
pub(crate) struct SqliteAutomationWorkflowRepository {
    pool: SqlitePool,
}

impl SqliteAutomationWorkflowRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn workflow_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<AutomationWorkflow, AppError> {
        let data: String = row.get("data");
        serde_json::from_str(&data)
            .map_err(|e| AppError::Database(format!("deserialize automation workflow failed: {e}")))
    }

    async fn upsert(&self, workflow: &AutomationWorkflow) -> Result<(), AppError> {
        let mut stored = workflow.clone();
        stored.updated_at = chrono::Utc::now();
        let data = serde_json::to_string(&stored).map_err(|e| {
            AppError::Database(format!("serialize automation workflow failed: {e}"))
        })?;

        sqlx::query(
            r#"
            INSERT INTO automation_workflows
                (id, issue_number, repository, status, created_at, updated_at, data)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                issue_number = excluded.issue_number,
                repository = excluded.repository,
                status = excluded.status,
                updated_at = excluded.updated_at,
                data = excluded.data
            "#,
        )
        .bind(&stored.id)
        .bind(i64::from(stored.issue.number))
        .bind(&stored.issue.repository)
        .bind(format!("{:?}", stored.status))
        .bind(stored.created_at.to_rfc3339())
        .bind(stored.updated_at.to_rfc3339())
        .bind(data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("upsert automation workflow", e))
        .map(|_| ())
    }
}

#[async_trait]
impl AutomationWorkflowRepository for SqliteAutomationWorkflowRepository {
    async fn save(&self, workflow: &AutomationWorkflow) -> Result<(), AppError> {
        self.upsert(workflow).await
    }

    async fn update(&self, workflow: &AutomationWorkflow) -> Result<(), AppError> {
        self.upsert(workflow).await
    }

    async fn find_by_id(&self, workflow_id: &str) -> Result<AutomationWorkflow, AppError> {
        let row = sqlx::query("SELECT data FROM automation_workflows WHERE id = ?")
            .bind(workflow_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("load automation workflow", e))?
            .ok_or_else(|| AppError::NotFound(format!("Workflow {workflow_id} not found")))?;
        Self::workflow_from_row(&row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;
    use crate::modules::automation::domain::models::issue_pr::{Issue, WorkflowStatus};

    #[tokio::test]
    async fn test_workflow_round_trip() {
        let pool = test_pool().await;
        let repository = SqliteAutomationWorkflowRepository::new(pool);
        let workflow = AutomationWorkflow::new(Issue::new(
            42,
            "Issue".to_string(),
            "Body".to_string(),
            "user".to_string(),
            "owner/repo".to_string(),
        ));

        repository.save(&workflow).await.unwrap();
        let loaded = repository.find_by_id(&workflow.id).await.unwrap();
        assert_eq!(loaded.status, WorkflowStatus::Pending);
    }
}
