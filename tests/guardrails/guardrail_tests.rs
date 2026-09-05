use agent_tui::modules::guardrails::domain::models::guardrail::{
    Guardrail, GuardrailCheck, GuardrailRule, GuardrailType,
};

#[test]
fn test_guardrail_creation() {
    let guardrail = Guardrail::new(
        "Test Guardrail".to_string(),
        GuardrailType::SecurityCheck,
        "Test description".to_string(),
    );
    assert_eq!(guardrail.name, "Test Guardrail");
    assert_eq!(guardrail.guardrail_type, GuardrailType::SecurityCheck);
}

#[test]
fn test_guardrail_rule_creation() {
    let rule = GuardrailRule::new(
        "no-hardcoded-secrets".to_string(),
        agent_tui::modules::guardrails::domain::models::guardrail::RuleType::PatternMatch,
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailAction::Block,
    );
    assert_eq!(rule.name, "no-hardcoded-secrets");
}

#[test]
fn test_guardrail_check_creation() {
    let check = GuardrailCheck::new("check-1".to_string(), "Test Guardrail".to_string());
    assert!(check.passed);
}
