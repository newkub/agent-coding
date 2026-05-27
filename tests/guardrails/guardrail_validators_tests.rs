use agent_tui::modules::guardrails::domain::validators::guardrail_validators;
use agent_tui::modules::guardrails::domain::models::guardrail::{Guardrail, GuardrailRule};

#[test]
fn test_validate_guardrail_empty_name() {
    let guardrail = Guardrail::new(
        String::new(),
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailType::SecurityCheck,
        "Description".to_string(),
    );
    assert!(guardrail_validators::validate_guardrail(&guardrail).is_err());
}

#[test]
fn test_validate_guardrail_success() {
    let rule = GuardrailRule::new(
        "test-rule".to_string(),
        agent_tui::modules::guardrails::domain::models::guardrail::RuleType::Custom("Description".to_string()),
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailAction::Block,
    );
    let guardrail = Guardrail::new(
        "Test Guardrail".to_string(),
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailType::SecurityCheck,
        "Description".to_string(),
    ).with_rules(vec![rule]);
    assert!(guardrail_validators::validate_guardrail(&guardrail).is_ok());
}

#[test]
fn test_validate_guardrail_rule_empty_name() {
    let rule = GuardrailRule::new(
        String::new(),
        agent_tui::modules::guardrails::domain::models::guardrail::RuleType::Custom("Description".to_string()),
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailAction::Block,
    );
    assert!(guardrail_validators::validate_guardrail_rule(&rule).is_err());
}

#[test]
fn test_validate_guardrail_rule_success() {
    let rule = GuardrailRule::new(
        "test-rule".to_string(),
        agent_tui::modules::guardrails::domain::models::guardrail::RuleType::Custom("Description".to_string()),
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailAction::Block,
    );
    assert!(guardrail_validators::validate_guardrail_rule(&rule).is_ok());
}
