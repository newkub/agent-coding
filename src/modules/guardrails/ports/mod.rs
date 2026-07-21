use async_trait::async_trait;

use crate::modules::guardrails::domain::models::guardrail::{Guardrail, GuardrailCheck};
use crate::shared::kernel::result::AppError;

/// Port for guardrail management
#[async_trait]
pub trait GuardrailManager: Send + Sync {
    /// Create a new guardrail
    async fn create_guardrail(&self, guardrail: Guardrail) -> Result<Guardrail, AppError>;

    /// Get guardrail by ID
    async fn get_guardrail(&self, id: &str) -> Result<Guardrail, AppError>;

    /// List all guardrails
    async fn list_guardrails(&self) -> Result<Vec<Guardrail>, AppError>;

    /// Update guardrail
    async fn update_guardrail(&self, guardrail: Guardrail) -> Result<Guardrail, AppError>;

    /// Delete guardrail
    async fn delete_guardrail(&self, id: &str) -> Result<(), AppError>;

    /// Get enabled guardrails
    async fn get_enabled_guardrails(&self) -> Result<Vec<Guardrail>, AppError>;
}

/// Port for guardrail checking
#[async_trait]
pub trait GuardrailChecker: Send + Sync {
    /// Check input against all enabled guardrails
    async fn check_input(&self, input: &str) -> Result<Vec<GuardrailCheck>, AppError>;

    /// Check input against specific guardrail
    async fn check_input_against(
        &self,
        input: &str,
        guardrail_id: &str,
    ) -> Result<GuardrailCheck, AppError>;

    /// Filter output through guardrails
    async fn filter_output(&self, output: &str) -> Result<String, AppError>;

    /// Check if action should be taken based on guardrail results
    async fn should_block(&self, checks: &[GuardrailCheck]) -> Result<bool, AppError>;
}
