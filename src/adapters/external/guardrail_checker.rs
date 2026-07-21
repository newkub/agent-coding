use async_trait::async_trait;

use crate::modules::guardrails::domain::models::guardrail::{GuardrailAction, GuardrailCheck};
use crate::modules::guardrails::domain::operations::guardrail_operations::{
    check_input_against_guardrail, filter_output, should_take_action,
};
use crate::modules::guardrails::domain::validators::guardrail_validators;
use crate::modules::guardrails::ports::{GuardrailChecker, GuardrailManager};
use crate::shared::kernel::result::AppError;

/// Default guardrail checker
pub struct DefaultGuardrailChecker<M>
where
    M: GuardrailManager,
{
    manager: M,
}

impl<M> DefaultGuardrailChecker<M>
where
    M: GuardrailManager,
{
    pub(crate) const fn new(manager: M) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl<M> GuardrailChecker for DefaultGuardrailChecker<M>
where
    M: GuardrailManager,
{
    async fn check_input(&self, input: &str) -> Result<Vec<GuardrailCheck>, AppError> {
        guardrail_validators::validate_input_for_check(input)?;

        let guardrails = self.manager.get_enabled_guardrails().await?;
        let mut checks = Vec::new();

        for guardrail in guardrails {
            let check = check_input_against_guardrail(input, &guardrail);
            checks.push(check);
        }

        Ok(checks)
    }

    async fn check_input_against(
        &self,
        input: &str,
        guardrail_id: &str,
    ) -> Result<GuardrailCheck, AppError> {
        guardrail_validators::validate_input_for_check(input)?;

        let guardrail = self.manager.get_guardrail(guardrail_id).await?;
        let check = check_input_against_guardrail(input, &guardrail);

        Ok(check)
    }

    async fn filter_output(&self, output: &str) -> Result<String, AppError> {
        let guardrails = self.manager.get_enabled_guardrails().await?;
        let mut filtered = output.to_string();

        for guardrail in guardrails {
            filtered = filter_output(&filtered, &guardrail);
        }

        Ok(filtered)
    }

    async fn should_block(&self, checks: &[GuardrailCheck]) -> Result<bool, AppError> {
        for check in checks {
            let action = should_take_action(check);
            if matches!(action, GuardrailAction::Block) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations would go here
}
