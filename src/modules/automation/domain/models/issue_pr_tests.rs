#[cfg(test)]
mod tests {
    use crate::modules::automation::domain::models::issue_pr::{
        AutomationWorkflow, Issue, IssueState, StepStatus, WorkflowStatus,
    };

    #[test]
    fn test_issue_creation() {
        let issue = Issue::new(
            1,
            "Test Issue".to_string(),
            "Test body".to_string(),
            "user".to_string(),
            "repo".to_string(),
        );
        assert_eq!(issue.number, 1);
        assert_eq!(issue.state, IssueState::Open);
    }

    #[test]
    fn test_issue_automatable_with_label() {
        let mut issue = Issue::new(
            1,
            "Test Issue".to_string(),
            "Test body".to_string(),
            "user".to_string(),
            "repo".to_string(),
        );
        issue.labels.push("automated".to_string());
        assert!(issue.is_automatable());
    }

    #[test]
    fn test_issue_automatable_with_keyword() {
        let issue = Issue::new(
            1,
            "Test Issue".to_string(),
            "Please /automate this".to_string(),
            "user".to_string(),
            "repo".to_string(),
        );
        assert!(issue.is_automatable());
    }

    #[test]
    fn test_workflow_creation() {
        let issue = Issue::new(
            1,
            "Test Issue".to_string(),
            "Test body".to_string(),
            "user".to_string(),
            "repo".to_string(),
        );
        let workflow = AutomationWorkflow::new(issue);
        assert_eq!(workflow.status, WorkflowStatus::Pending);
        assert!(workflow.pr.is_none());
    }

    #[test]
    fn test_workflow_add_step() {
        let issue = Issue::new(
            1,
            "Test Issue".to_string(),
            "Test body".to_string(),
            "user".to_string(),
            "repo".to_string(),
        );
        let mut workflow = AutomationWorkflow::new(issue);
        workflow.add_step("Create branch".to_string());
        assert_eq!(workflow.steps.len(), 1);
        assert_eq!(workflow.steps[0].status, StepStatus::Pending);
    }
}
