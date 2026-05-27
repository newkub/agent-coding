use agent_tui::modules::automation::domain::validators::automation_validators;
use agent_tui::modules::automation::domain::models::issue_pr::{Issue, AutomationConfig};

#[test]
fn test_validate_issue_for_automation_success() {
    let mut issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Please /automate this".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );
    assert!(automation_validators::validate_issue_for_automation(&issue).is_ok());
}

#[test]
fn test_validate_issue_for_automation_closed() {
    let mut issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Please /automate this".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );
    issue.state = agent_tui::modules::automation::domain::models::issue_pr::IssueState::Closed;
    assert!(automation_validators::validate_issue_for_automation(&issue).is_err());
}

#[test]
fn test_validate_automation_config_success() {
    let config = AutomationConfig::default();
    assert!(automation_validators::validate_automation_config(&config).is_ok());
}

#[test]
fn test_validate_automation_config_missing_placeholder() {
    let mut config = AutomationConfig::default();
    config.branch_name_template = "feature/test".to_string();
    assert!(automation_validators::validate_automation_config(&config).is_err());
}

#[test]
fn test_validate_repository_access_success() {
    assert!(automation_validators::validate_repository_access("owner/repo").is_ok());
}

#[test]
fn test_validate_repository_access_invalid() {
    assert!(automation_validators::validate_repository_access("invalid").is_err());
}
