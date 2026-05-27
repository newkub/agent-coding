use crate::modules::guardrails::domain::models::guardrail::{Guardrail, GuardrailRule, GuardrailType};
use crate::shared::kernel::result::AppError;

/// Pure function to validate guardrail
pub fn validate_guardrail(guardrail: &Guardrail) -> Result<(), AppError> {
    if guardrail.name.is_empty() {
        return Err(AppError::ValidationError(
            "Guardrail name cannot be empty".to_string(),
        ));
    }

    if guardrail.description.is_empty() {
        return Err(AppError::ValidationError(
            "Guardrail description cannot be empty".to_string(),
        ));
    }

    if guardrail.rules.is_empty() {
        return Err(AppError::ValidationError(
            "Guardrail must have at least one rule".to_string(),
        ));
    }

    for rule in &guardrail.rules {
        validate_guardrail_rule(rule)?;
    }

    Ok(())
}

/// Pure function to validate guardrail rule
pub fn validate_guardrail_rule(rule: &GuardrailRule) -> Result<(), AppError> {
    if rule.name.is_empty() {
        return Err(AppError::ValidationError(
            "Rule name cannot be empty".to_string(),
        ));
    }

    // Validate pattern based on rule type
    if let Some(pattern) = &rule.pattern {
        match &rule.rule_type {
            crate::modules::guardrails::domain::models::guardrail::RuleType::PatternMatch
            | crate::modules::guardrails::domain::models::guardrail::RuleType::KeywordDetection
                if pattern.is_empty() => {
                    return Err(AppError::ValidationError(
                        "Pattern cannot be empty for this rule type".to_string(),
                    ));
                }
            crate::modules::guardrails::domain::models::guardrail::RuleType::LengthCheck
                if pattern.parse::<usize>().is_err() => {
                    return Err(AppError::ValidationError(
                        "Pattern must be a valid number for length check".to_string(),
                    ));
                }
            crate::modules::guardrails::domain::models::guardrail::RuleType::FormatValidation
                if regex::Regex::new(pattern).is_err() => {
                    return Err(AppError::ValidationError(
                        "Pattern must be a valid regex for format validation".to_string(),
                    ));
                }
            _ => {}
        }
    }

    Ok(())
}

/// Pure function to validate input for guardrail check
pub fn validate_input_for_check(input: &str) -> Result<(), AppError> {
    if input.is_empty() {
        return Err(AppError::ValidationError(
            "Input cannot be empty for guardrail check".to_string(),
        ));
    }

    if input.len() > 1_000_000 {
        return Err(AppError::ValidationError(
            "Input too long for guardrail check (max 1,000,000 characters)".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate guardrail type compatibility
pub const fn validate_guardrail_type_compatibility(
    guardrail_type: &GuardrailType,
    rule_type: &crate::modules::guardrails::domain::models::guardrail::RuleType,
) -> Result<(), AppError> {
    match (guardrail_type, rule_type) {
        (GuardrailType::InputValidation, _) => Ok(()),
        (GuardrailType::OutputFiltering, _) => Ok(()),
        (GuardrailType::ContentModeration, _) => Ok(()),
        (GuardrailType::SecurityCheck, _) => Ok(()),
        (GuardrailType::PermissionCheck, _) => Ok(()),
        (GuardrailType::RateLimiting, _) => Ok(()),
        (GuardrailType::DataPrivacy, _) => Ok(()),
        (GuardrailType::Compliance, _) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::guardrails::domain::models::guardrail::{GuardrailAction, RuleType};

    #[test]
    fn test_validate_guardrail_empty_name() {
        let guardrail = Guardrail::new(
            String::new(),
            GuardrailType::InputValidation,
            "Test".to_string(),
        );
        assert!(validate_guardrail(&guardrail).is_err());
    }

    #[test]
    fn test_validate_guardrail_no_rules() {
        let guardrail = Guardrail::new(
            "Test".to_string(),
            GuardrailType::InputValidation,
            "Test".to_string(),
        );
        assert!(validate_guardrail(&guardrail).is_err());
    }

    #[test]
    fn test_validate_guardrail_rule_invalid_regex() {
        let rule = GuardrailRule::new(
            "Test Rule".to_string(),
            RuleType::FormatValidation,
            GuardrailAction::Block,
        ).with_pattern("[invalid".to_string());
        
        assert!(validate_guardrail_rule(&rule).is_err());
    }

    #[test]
    fn test_validate_input_for_check_empty() {
        assert!(validate_input_for_check("").is_err());
    }

    #[test]
    fn test_validate_input_for_check_success() {
        assert!(validate_input_for_check("test input").is_ok());
    }
}
