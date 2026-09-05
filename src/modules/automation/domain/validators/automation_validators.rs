use crate::modules::automation::domain::models::issue_pr::{AutomationConfig, Issue};
use crate::shared::kernel::result::AppError;

/// Pure function to validate issue for automation
pub fn validate_issue_for_automation(issue: &Issue) -> Result<(), AppError> {
    if issue.state != crate::modules::automation::domain::models::issue_pr::IssueState::Open {
        return Err(AppError::ValidationError(
            "Issue must be open for automation".to_string(),
        ));
    }

    if issue.title.is_empty() {
        return Err(AppError::ValidationError(
            "Issue title cannot be empty".to_string(),
        ));
    }

    if !issue.is_automatable() {
        return Err(AppError::ValidationError(
            "Issue is not marked for automation (missing 'automated' label or /automate keyword)"
                .to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate automation config
pub fn validate_automation_config(config: &AutomationConfig) -> Result<(), AppError> {
    if config.branch_name_template.is_empty() {
        return Err(AppError::ValidationError(
            "Branch name template cannot be empty".to_string(),
        ));
    }

    if config.commit_message_template.is_empty() {
        return Err(AppError::ValidationError(
            "Commit message template cannot be empty".to_string(),
        ));
    }

    // Check that templates contain required placeholders
    if !config.branch_name_template.contains("{number}") {
        return Err(AppError::ValidationError(
            "Branch name template must contain {number} placeholder".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate repository access
pub fn validate_repository_access(repository: &str) -> Result<(), AppError> {
    if repository.is_empty() {
        return Err(AppError::ValidationError(
            "Repository name cannot be empty".to_string(),
        ));
    }

    // Validate repository format (owner/repo)
    let parts: Vec<&str> = repository.split('/').collect();
    if parts.len() != 2 {
        return Err(AppError::ValidationError(
            "Repository must be in format 'owner/repo'".to_string(),
        ));
    }

    if parts[0].is_empty() || parts[1].is_empty() {
        return Err(AppError::ValidationError(
            "Repository owner and name cannot be empty".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_issue_for_automation_success() {
        let issue = Issue::new(
            1,
            "Test Issue".to_string(),
            "Please /automate this".to_string(),
            "user".to_string(),
            "owner/repo".to_string(),
        );
        assert!(validate_issue_for_automation(&issue).is_ok());
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
        issue.state = crate::modules::automation::domain::models::issue_pr::IssueState::Closed;
        assert!(validate_issue_for_automation(&issue).is_err());
    }

    #[test]
    fn test_validate_issue_for_automation_not_automatable() {
        let issue = Issue::new(
            1,
            "Test Issue".to_string(),
            "Description".to_string(),
            "user".to_string(),
            "owner/repo".to_string(),
        );
        assert!(validate_issue_for_automation(&issue).is_err());
    }

    #[test]
    fn test_validate_automation_config_success() {
        let config = AutomationConfig::default();
        assert!(validate_automation_config(&config).is_ok());
    }

    #[test]
    fn test_validate_automation_config_missing_placeholder() {
        let mut config = AutomationConfig::default();
        config.branch_name_template = "feature/test".to_string();
        assert!(validate_automation_config(&config).is_err());
    }

    #[test]
    fn test_validate_repository_access_success() {
        assert!(validate_repository_access("owner/repo").is_ok());
    }

    #[test]
    fn test_validate_repository_access_invalid_format() {
        assert!(validate_repository_access("invalid").is_err());
        assert!(validate_repository_access("/repo").is_err());
        assert!(validate_repository_access("owner/").is_err());
    }
}
