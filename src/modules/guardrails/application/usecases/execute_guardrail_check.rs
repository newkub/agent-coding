use crate::modules::guardrails::domain::models::guardrail::{
    Guardrail, GuardrailAction, GuardrailCheck,
};
use crate::modules::guardrails::domain::validators::guardrail_validators;
use crate::modules::guardrails::ports::{GuardrailChecker, GuardrailManager};
use crate::shared::kernel::result::AppError;

/// Use case for executing guardrail checks
pub(crate) struct ExecuteGuardrailCheckUseCase<M, C>
where
    M: GuardrailManager,
    C: GuardrailChecker,
{
    manager: M,
    checker: C,
}

impl<M, C> ExecuteGuardrailCheckUseCase<M, C>
where
    M: GuardrailManager,
    C: GuardrailChecker,
{
    pub(crate) const fn new(manager: M, checker: C) -> Self {
        Self { manager, checker }
    }

    /// Check input against all guardrails
    pub(crate) async fn check_input(&self, input: &str) -> Result<Vec<GuardrailCheck>, AppError> {
        self.checker.check_input(input).await
    }

    /// Check input and determine if it should be blocked
    pub(crate) async fn check_and_validate(
        &self,
        input: &str,
    ) -> Result<GuardrailResult, AppError> {
        let checks = self.check_input(input).await?;
        let should_block = self.checker.should_block(&checks).await?;

        let action = if should_block {
            GuardrailAction::Block
        } else if checks.iter().any(|c| !c.passed) {
            GuardrailAction::Warn
        } else {
            GuardrailAction::Allow
        };

        Ok(GuardrailResult {
            action,
            checks,
            passed: !should_block,
        })
    }

    /// Filter output through guardrails
    pub(crate) async fn filter_output(&self, output: &str) -> Result<String, AppError> {
        self.checker.filter_output(output).await
    }

    /// List all guardrails
    pub(crate) async fn list_guardrails(&self) -> Result<Vec<Guardrail>, AppError> {
        self.manager.list_guardrails().await
    }

    /// Create a new guardrail
    pub(crate) async fn create_guardrail(
        &self,
        guardrail: Guardrail,
    ) -> Result<Guardrail, AppError> {
        guardrail_validators::validate_guardrail(&guardrail)?;
        self.manager.create_guardrail(guardrail).await
    }

    /// Enable/disable a guardrail
    pub(crate) async fn toggle_guardrail(
        &self,
        id: &str,
        enabled: bool,
    ) -> Result<Guardrail, AppError> {
        let mut guardrail = self.manager.get_guardrail(id).await?;
        guardrail.enabled = enabled;
        self.manager.update_guardrail(guardrail).await
    }

    /// Delete a guardrail
    pub(crate) async fn delete_guardrail(&self, id: &str) -> Result<(), AppError> {
        self.manager.delete_guardrail(id).await
    }
}

#[derive(Debug, Clone)]
pub(crate) struct GuardrailResult {
    pub action: GuardrailAction,
    pub checks: Vec<GuardrailCheck>,
    pub passed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations would go here
}
