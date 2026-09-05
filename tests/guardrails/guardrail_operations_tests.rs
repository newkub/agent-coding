use agent_tui::modules::guardrails::domain::models::guardrail::{
    Guardrail, GuardrailAction, GuardrailRule, RuleType,
};
use agent_tui::modules::guardrails::domain::operations::guardrail_operations::{
    check_input_against_guardrail, filter_output, should_take_action,
};

#[test]
fn test_check_guardrail_pass() {
    let guardrail = Guardrail::new(
        "Test Guardrail".to_string(),
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailType::SecurityCheck,
        "Test description".to_string(),
    );
    let check = check_input_against_guardrail("safe input", &guardrail).unwrap();
    assert!(check.passed);
}

#[test]
fn test_should_take_action() {
    let guardrail = Guardrail::new(
        "Test Guardrail".to_string(),
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailType::SecurityCheck,
        "Test description".to_string(),
    );
    let check = check_input_against_guardrail("safe input", &guardrail).unwrap();
    let action = should_take_action(&check);
    assert_eq!(action, GuardrailAction::Allow);
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
        agent_tui::modules::guardrails::domain::models::guardrail::GuardrailType::OutputFiltering,
        "Test".to_string(),
    )
    .with_rules(vec![rule]);

    let filtered = filter_output("The password is secret", &guardrail);
    assert!(filtered.contains("[REDACTED]"));
}
