use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::guardrails::domain::models::guardrail::{
    Guardrail, GuardrailAction, GuardrailRule, GuardrailType, RuleType, Severity,
};
use crate::modules::guardrails::ports::GuardrailManager;
use crate::shared::kernel::result::AppError;

/// SQLite-backed guardrail manager.
#[derive(Clone)]
pub(crate) struct SqliteGuardrailManager {
    pool: SqlitePool,
}

impl SqliteGuardrailManager {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Seed the default production guardrail rules once.
    pub(crate) async fn initialize_default_guardrails(&self) -> Result<(), AppError> {
        let existing = self.list_guardrails().await?;
        let defaults = vec![
            Guardrail::new(
                "input-length".to_string(),
                GuardrailType::InputValidation,
                "Rejects input that exceeds the configured size limit".to_string(),
            )
            .with_rules(vec![GuardrailRule::new(
                "max-input-length".to_string(),
                RuleType::LengthCheck,
                GuardrailAction::Block,
            )
            .with_pattern("10000".to_string())
            .with_severity(Severity::High)]),
            Guardrail::new(
                "content-moderation".to_string(),
                GuardrailType::ContentModeration,
                "Flags configured unsafe content terms".to_string(),
            )
            .with_severity(Severity::High)
            .with_rules(vec![GuardrailRule::new(
                "unsafe-content".to_string(),
                RuleType::ContentClassification,
                GuardrailAction::Escalate,
            )
            .with_pattern("violence,self-harm,hate speech".to_string())
            .with_severity(Severity::High)]),
            Guardrail::new(
                "security-check".to_string(),
                GuardrailType::SecurityCheck,
                "Blocks obvious credential and secret disclosures".to_string(),
            )
            .with_severity(Severity::Critical)
            .with_rules(vec![
                GuardrailRule::new(
                    "api-key-keyword".to_string(),
                    RuleType::KeywordDetection,
                    GuardrailAction::Block,
                )
                .with_pattern("api key".to_string())
                .with_severity(Severity::Critical),
                GuardrailRule::new(
                    "password-keyword".to_string(),
                    RuleType::KeywordDetection,
                    GuardrailAction::Block,
                )
                .with_pattern("password".to_string())
                .with_severity(Severity::High),
            ]),
        ];

        for guardrail in defaults {
            if existing
                .iter()
                .any(|existing_guardrail| existing_guardrail.name == guardrail.name)
            {
                continue;
            }
            self.upsert(&guardrail).await?;
        }
        Ok(())
    }

    fn guardrail_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Guardrail, AppError> {
        let data: String = row.get("data");
        serde_json::from_str(&data)
            .map_err(|e| AppError::Database(format!("deserialize guardrail failed: {e}")))
    }

    async fn upsert(&self, guardrail: &Guardrail) -> Result<(), AppError> {
        let mut stored = guardrail.clone();
        stored.updated_at = chrono::Utc::now();
        let data = serde_json::to_string(&stored)
            .map_err(|e| AppError::Database(format!("serialize guardrail failed: {e}")))?;

        sqlx::query(
            r#"
            INSERT INTO guardrails
                (id, name, guardrail_type, enabled, severity, created_at, updated_at, data)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                guardrail_type = excluded.guardrail_type,
                enabled = excluded.enabled,
                severity = excluded.severity,
                updated_at = excluded.updated_at,
                data = excluded.data
            "#,
        )
        .bind(&stored.id)
        .bind(&stored.name)
        .bind(format!("{:?}", stored.guardrail_type))
        .bind(stored.enabled)
        .bind(format!("{:?}", stored.severity))
        .bind(stored.created_at.to_rfc3339())
        .bind(stored.updated_at.to_rfc3339())
        .bind(data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("upsert guardrail", e))
        .map(|_| ())
    }
}

#[async_trait]
impl GuardrailManager for SqliteGuardrailManager {
    async fn create_guardrail(&self, guardrail: Guardrail) -> Result<Guardrail, AppError> {
        self.upsert(&guardrail).await?;
        Ok(guardrail)
    }

    async fn get_guardrail(&self, id: &str) -> Result<Guardrail, AppError> {
        let row = sqlx::query("SELECT data FROM guardrails WHERE id = ? OR name = ?")
            .bind(id)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("load guardrail", e))?
            .ok_or_else(|| AppError::NotFound(format!("Guardrail {id} not found")))?;
        Self::guardrail_from_row(&row)
    }

    async fn list_guardrails(&self) -> Result<Vec<Guardrail>, AppError> {
        let rows = sqlx::query("SELECT data FROM guardrails ORDER BY name ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("list guardrails", e))?;
        rows.iter().map(Self::guardrail_from_row).collect()
    }

    async fn update_guardrail(&self, guardrail: Guardrail) -> Result<Guardrail, AppError> {
        self.get_guardrail(&guardrail.id).await?;
        self.upsert(&guardrail).await?;
        Ok(guardrail)
    }

    async fn delete_guardrail(&self, id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM guardrails WHERE id = ? OR name = ?")
            .bind(id)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("delete guardrail", e))?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Guardrail {id} not found")));
        }
        Ok(())
    }

    async fn get_enabled_guardrails(&self) -> Result<Vec<Guardrail>, AppError> {
        Ok(self
            .list_guardrails()
            .await?
            .into_iter()
            .filter(Guardrail::is_enabled)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;

    #[tokio::test]
    async fn test_seed_and_get_by_name() {
        let manager = SqliteGuardrailManager::new(test_pool().await);
        manager.initialize_default_guardrails().await.unwrap();
        let guardrail = manager.get_guardrail("security-check").await.unwrap();
        assert!(!guardrail.rules.is_empty());
    }
}
