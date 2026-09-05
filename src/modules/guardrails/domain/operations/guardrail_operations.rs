use crate::modules::guardrails::domain::models::guardrail::{
    Guardrail, GuardrailAction, GuardrailCheck, GuardrailRule, RuleType,
};
use crate::shared::kernel::result::AppError;

/// Check input against guardrail rules.
///
/// Unsupported rule types return an explicit error instead of being skipped.
pub fn check_input_against_guardrail(
    input: &str,
    guardrail: &Guardrail,
) -> Result<GuardrailCheck, AppError> {
    let mut check = GuardrailCheck::new(guardrail.id.clone(), guardrail.name.clone());

    if !guardrail.is_enabled() {
        return Ok(check);
    }

    for rule in &guardrail.rules {
        if !rule.enabled {
            continue;
        }

        if let Some(violation) = check_rule(input, rule)? {
            check.add_violation(violation);
        }
    }

    Ok(check)
}

/// Check a single rule and return an explicit error for unsupported types.
fn check_rule(
    input: &str,
    rule: &GuardrailRule,
) -> Result<
    Option<crate::modules::guardrails::domain::models::guardrail::GuardrailViolation>,
    AppError,
> {
    match &rule.rule_type {
        RuleType::PatternMatch => {
            if let Some(pattern) = &rule.pattern {
                if input.contains(pattern) {
                    return Ok(Some(create_violation(
                        rule,
                        "Pattern matched in input".to_string(),
                    )));
                }
            }
        }
        RuleType::KeywordDetection => {
            if let Some(pattern) = &rule.pattern {
                let input_lower = input.to_lowercase();
                let pattern_lower = pattern.to_lowercase();
                if input_lower.contains(&pattern_lower) {
                    return Ok(Some(create_violation(
                        rule,
                        format!("Keyword '{pattern}' detected"),
                    )));
                }
            }
        }
        RuleType::LengthCheck => {
            if let Some(pattern) = &rule.pattern {
                let max_length = pattern.parse::<usize>().map_err(|_| {
                    AppError::ValidationError(format!(
                        "length-check rule '{}' requires a numeric pattern",
                        rule.name
                    ))
                })?;
                if input.len() > max_length {
                    return Ok(Some(create_violation(
                        rule,
                        format!("Input exceeds maximum length of {max_length}"),
                    )));
                }
            }
        }
        RuleType::FormatValidation => {
            if let Some(pattern) = &rule.pattern {
                let regex = regex::Regex::new(pattern).map_err(|e| {
                    AppError::ValidationError(format!(
                        "format-validation rule '{}' has an invalid pattern: {e}",
                        rule.name
                    ))
                })?;
                if !regex.is_match(input) {
                    return Ok(Some(create_violation(
                        rule,
                        "Input format validation failed".to_string(),
                    )));
                }
            }
        }
        RuleType::ContentClassification => {
            let Some(pattern) = &rule.pattern else {
                return Err(AppError::ValidationError(format!(
                    "content-classification rule '{}' requires comma-separated terms",
                    rule.name
                )));
            };
            let input_lower = input.to_lowercase();
            let matched = pattern
                .split(',')
                .map(str::trim)
                .filter(|term| !term.is_empty())
                .find(|term| input_lower.contains(&term.to_lowercase()));
            if let Some(term) = matched {
                return Ok(Some(create_violation(
                    rule,
                    format!("Classified content term '{term}' detected"),
                )));
            }
        }
        RuleType::Custom(name) => {
            return Err(AppError::State(format!(
                "custom guardrail rule '{name}' is not registered"
            )));
        }
    }

    Ok(None)
}

/// Pure function to create violation from rule
fn create_violation(
    rule: &GuardrailRule,
    message: String,
) -> crate::modules::guardrails::domain::models::guardrail::GuardrailViolation {
    let suggested_action = match rule.action {
        GuardrailAction::Block => "Block this input".to_string(),
        GuardrailAction::Warn => "Warn user about this input".to_string(),
        GuardrailAction::Modify => "Modify this input to comply".to_string(),
        GuardrailAction::Escalate => "Escalate to human review".to_string(),
        GuardrailAction::Allow => "Allow this input".to_string(),
    };

    crate::modules::guardrails::domain::models::guardrail::GuardrailViolation {
        rule_id: rule.id.clone(),
        rule_name: rule.name.clone(),
        message,
        severity: rule.severity.clone(),
        action: rule.action.clone(),
        suggested_action,
    }
}

/// Pure function to determine if action should be taken
pub fn should_take_action(check: &GuardrailCheck) -> GuardrailAction {
    if check.passed {
        return GuardrailAction::Allow;
    }

    for action in [
        GuardrailAction::Block,
        GuardrailAction::Escalate,
        GuardrailAction::Modify,
        GuardrailAction::Warn,
    ] {
        if check.violations.iter().any(|v| v.action == action) {
            return action;
        }
    }

    GuardrailAction::Allow
}

/// Pure function to filter output based on guardrails
pub fn filter_output(output: &str, guardrail: &Guardrail) -> String {
    if !guardrail.is_enabled() {
        return output.to_string();
    }

    let mut filtered = output.to_string();

    for rule in &guardrail.rules {
        if !rule.enabled {
            continue;
        }

        if let Some(pattern) = &rule.pattern {
            if rule.action == GuardrailAction::Modify {
                filtered = filtered.replace(pattern, "[REDACTED]");
            }
        }
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_input_pattern_match() {
        let rule = GuardrailRule::new(
            "Test Rule".to_string(),
            RuleType::PatternMatch,
            GuardrailAction::Block,
        )
        .with_pattern("secret".to_string());

        let guardrail = Guardrail::new(
            "Test Guardrail".to_string(),
            crate::modules::guardrails::domain::models::guardrail::GuardrailType::InputValidation,
            "Test".to_string(),
        )
        .with_rules(vec![rule]);

        let check = check_input_against_guardrail("This is a secret key", &guardrail).unwrap();
        assert!(!check.passed);
    }

    #[test]
    fn test_check_input_length() {
        let rule = GuardrailRule::new(
            "Length Rule".to_string(),
            RuleType::LengthCheck,
            GuardrailAction::Block,
        )
        .with_pattern("10".to_string());

        let guardrail = Guardrail::new(
            "Length Guardrail".to_string(),
            crate::modules::guardrails::domain::models::guardrail::GuardrailType::InputValidation,
            "Test".to_string(),
        )
        .with_rules(vec![rule]);

        let check = check_input_against_guardrail("This is too long", &guardrail).unwrap();
        assert!(!check.passed);
    }

    #[test]
    fn test_filter_output() {
        let rule = GuardrailRule::new(
            "Redact Rule".to_string(),
            RuleType::PatternMatch,
            GuardrailAction::Modify,
        )
        .with_pattern("password".to_string());

        let guardrail = Guardrail::new(
            "Output Filter".to_string(),
            crate::modules::guardrails::domain::models::guardrail::GuardrailType::OutputFiltering,
            "Test".to_string(),
        )
        .with_rules(vec![rule]);

        let filtered = filter_output("The password is secret", &guardrail);
        assert!(filtered.contains("[REDACTED]"));
    }
}
