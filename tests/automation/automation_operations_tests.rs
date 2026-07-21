use agent_tui::modules::automation::domain::models::issue_pr::{AutomationConfig, Issue};
use agent_tui::modules::automation::domain::operations::automation_operations::{
    determine_target_branch, extract_labels, generate_branch_name, generate_commit_message,
    generate_pr_body, generate_pr_title,
};

#[test]
fn test_generate_branch_name() {
    let issue = Issue::new(
        1,
        "Test Feature".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );

    let config = AutomationConfig {
        branch_name_template: "feature/{number}-{title}".to_string(),
        ..Default::default()
    };

    let branch = generate_branch_name(&issue, &config);
    assert!(branch.contains("1"));
    assert!(branch.contains("test-feature"));
}

#[test]
fn test_generate_commit_message() {
    let issue = Issue::new(
        1,
        "Add new feature".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );

    let config = AutomationConfig {
        commit_message_template: "feat: {title}".to_string(),
        ..Default::default()
    };

    let message = generate_commit_message(&issue, &config);
    assert!(message.contains("Add new feature"));
    assert!(message.contains("Closes #1"));
}

#[test]
fn test_generate_pr_title() {
    let issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );

    let title = generate_pr_title(&issue);
    assert_eq!(title, "Test Issue (#1)");
}

#[test]
fn test_extract_labels() {
    let mut issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );
    issue.labels.push("bug".to_string());

    let config = AutomationConfig::default();
    let labels = extract_labels(&issue, &config);
    assert!(labels.contains(&"bug".to_string()));
    assert!(labels.contains(&"automated".to_string()));
}

#[test]
fn test_determine_target_branch() {
    let mut issue = Issue::new(
        1,
        "Test Issue".to_string(),
        "Description".to_string(),
        "user".to_string(),
        "owner/repo".to_string(),
    );
    issue.labels.push("develop".to_string());

    let branch = determine_target_branch(&issue);
    assert_eq!(branch, "develop");
}
