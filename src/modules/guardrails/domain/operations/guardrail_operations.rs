use crate::modules::guardrails::domain::models::guardrail::{
    Guardrail, GuardrailAction, GuardrailCheck, GuardrailRule, RuleType, Severity,
};

/// Pure function to check input against guardrail rules
pub fn check_input_against_guardrail(input: &str, guardrail: &Guardrail) -> GuardrailCheck {
    let mut check = GuardrailCheck::new(guardrail.id.clone(), guardrail.name.clone());

    if !guardrail.is_enabled() {
        return check;
    }

    for rule in &guardrail.rules {
        if !rule.enabled {
            continue;
        }

        if let Some(violation) = check_rule(input, rule) {
            check.add_violation(violation);
        }
    }

    check
}

/// Pure function to check a single rule
fn check_rule(
    input: &str,
    rule: &GuardrailRule,
) -> Option<crate::modules::guardrails::domain::models::guardrail::GuardrailViolation> {
    match &rule.rule_type {
        RuleType::PatternMatch => {
            if let Some(pattern) = &rule.pattern {
                if input.contains(pattern) {
                    return Some(create_violation(
                        rule,
                        "Pattern matched in input".to_string(),
                    ));
                }
            }
        }
        RuleType::KeywordDetection => {
            if let Some(pattern) = &rule.pattern {
                let input_lower = input.to_lowercase();
                let pattern_lower = pattern.to_lowercase();
                if input_lower.contains(&pattern_lower) {
                    return Some(create_violation(
                        rule,
                        format!("Keyword '{}' detected", pattern),
                    ));
                }
            }
        }
        RuleType::LengthCheck => {
            if let Some(pattern) = &rule.pattern {
                if let Ok(max_length) = pattern.parse::<usize>() {
                    if input.len() > max_length {
                        return Some(create_violation(
                            rule,
                            format!("Input exceeds maximum length of {}", max_length),
                        ));
                    }
                }
            }
        }
        RuleType::FormatValidation => {
            if let Some(pattern) = &rule.pattern {
                if !regex::Regex::new(pattern)
                    .map(|re| re.is_match(input))
                    .unwrap_or(false)
                {
                    return Some(create_violation(
                        rule,
                        "Input format validation failed".to_string(),
                    ));
                }
            }
        }
        RuleType::ContentClassification => {
            // In a real implementation, this would use ML classification
            // For now, we'll skip
        }
        RuleType::Custom(_) => {
            // Custom rules would be handled by external logic
        }
    }

    None
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
        severity: Severity::Medium, // Would be configured per rule
        suggested_action,
    }
}

/// Pure function to determine if action should be taken
pub fn should_take_action(check: &GuardrailCheck) -> GuardrailAction {
    if check.passed {
        return GuardrailAction::Allow;
    }

    if check.has_critical_violations() {
        return GuardrailAction::Block;
    }

    // Check for block actions in violations
    for violation in &check.violations {
        // In a real implementation, we'd check the rule's action
        if matches!(violation.severity, Severity::High | Severity::Critical) {
            return GuardrailAction::Block;
        }
    }

    GuardrailAction::Warn
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

        let check = check_input_against_guardrail("This is a secret key", &guardrail);
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

        let check = check_input_against_guardrail("This is too long", &guardrail);
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
