use agent_tui::modules::automation::domain::models::issue_pr::{
    AutomationConfig, AutomationWorkflow, Issue, IssueState, PRState, PullRequest, WorkflowStatus,
};

#[test]
fn test_issue_creation() {
    let issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );
    assert_eq!(issue.number, 1);
    assert_eq!(issue.state, IssueState::Open);
}

#[test]
fn test_issue_automatable_with_label() {
    let mut issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );
    issue.labels.push("automated".to_string());
    assert!(issue.is_automatable());
}

#[test]
fn test_pull_request_creation() {
    let pr = PullRequest::new(
        1,
        "Test PR".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "feature/test".to_string(),
        "main".to_string(),
        "owner/repo".to_string(),
    );
    assert_eq!(pr.number, 1);
    assert_eq!(pr.state, PRState::Open);
}

#[test]
fn test_automation_workflow_creation() {
    let issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );
    let workflow = AutomationWorkflow::new(issue);
    assert_eq!(workflow.status, WorkflowStatus::Pending);
}

#[test]
fn test_automation_config_default() {
    let config = AutomationConfig::default();
    assert!(config.auto_create_branch);
    assert!(config.auto_commit);
}
