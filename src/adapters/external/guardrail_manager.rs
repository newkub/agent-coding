use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::guardrails::domain::models::guardrail::{Guardrail, GuardrailType};
use crate::modules::guardrails::ports::GuardrailManager;
use crate::shared::kernel::result::AppError;

/// In-memory guardrail manager
pub(crate) struct InMemoryGuardrailManager {
    guardrails: Arc<RwLock<HashMap<String, Guardrail>>>,
}

impl InMemoryGuardrailManager {
    pub(crate) fn new() -> Self {
        Self {
            guardrails: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) async fn initialize_default_guardrails(&self) -> Result<(), AppError> {
        let default_guardrails = vec![
            (
                "input-length",
                GuardrailType::InputValidation,
                "Validates input length",
            ),
            (
                "content-moderation",
                GuardrailType::ContentModeration,
                "Filters inappropriate content",
            ),
            (
                "security-check",
                GuardrailType::SecurityCheck,
                "Checks for security issues",
            ),
        ];

        for (name, guardrail_type, description) in default_guardrails {
            let guardrail =
                Guardrail::new(name.to_string(), guardrail_type, description.to_string());
            self.create_guardrail(guardrail).await?;
        }

        Ok(())
    }
}

impl Default for InMemoryGuardrailManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GuardrailManager for InMemoryGuardrailManager {
    async fn create_guardrail(&self, guardrail: Guardrail) -> Result<Guardrail, AppError> {
        let mut guardrails = self.guardrails.write().await;
        guardrails.insert(guardrail.id.clone(), guardrail.clone());
        Ok(guardrail)
    }

    async fn get_guardrail(&self, id: &str) -> Result<Guardrail, AppError> {
        let guardrails = self.guardrails.read().await;
        guardrails
            .get(id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Guardrail {} not found", id)))
    }

    async fn list_guardrails(&self) -> Result<Vec<Guardrail>, AppError> {
        let guardrails = self.guardrails.read().await;
        Ok(guardrails.values().cloned().collect())
    }

    async fn update_guardrail(&self, guardrail: Guardrail) -> Result<Guardrail, AppError> {
        let mut guardrails = self.guardrails.write().await;
        if guardrails.contains_key(&guardrail.id) {
            guardrails.insert(guardrail.id.clone(), guardrail.clone());
            Ok(guardrail)
        } else {
            Err(AppError::NotFound(format!(
                "Guardrail {} not found",
                guardrail.id
            )))
        }
    }

    async fn delete_guardrail(&self, id: &str) -> Result<(), AppError> {
        let mut guardrails = self.guardrails.write().await;
        if guardrails.remove(id).is_some() {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Guardrail {} not found", id)))
        }
    }

    async fn get_enabled_guardrails(&self) -> Result<Vec<Guardrail>, AppError> {
        let guardrails = self.guardrails.read().await;
        Ok(guardrails
            .values()
            .filter(|g| g.is_enabled())
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_guardrail() {
        let manager = InMemoryGuardrailManager::new();
        let guardrail = Guardrail::new(
            "Test".to_string(),
            GuardrailType::InputValidation,
            "Test".to_string(),
        );
        let result = manager.create_guardrail(guardrail).await;
        assert!(result.is_ok());
    }
}
