use crate::modules::subagents::domain::models::subagent::{Subagent, SubagentConfig, SubagentTask};
use crate::shared::kernel::result::AppError;

/// Pure function to validate subagent configuration
pub fn validate_subagent_config(config: &SubagentConfig) -> Result<(), AppError> {
    if config.model.is_empty() {
        return Err(AppError::ValidationError(
            "Model name cannot be empty".to_string(),
        ));
    }

    if config.temperature < 0.0 || config.temperature > 2.0 {
        return Err(AppError::ValidationError(
            "Temperature must be between 0.0 and 2.0".to_string(),
        ));
    }

    if let Some(max_tokens) = config.max_tokens {
        if max_tokens == 0 {
            return Err(AppError::ValidationError(
                "Max tokens must be greater than 0".to_string(),
            ));
        }
        if max_tokens > 128000 {
            return Err(AppError::ValidationError(
                "Max tokens too large (max 128000)".to_string(),
            ));
        }
    }

    Ok(())
}

/// Pure function to validate subagent
pub fn validate_subagent(subagent: &Subagent) -> Result<(), AppError> {
    if subagent.name.is_empty() {
        return Err(AppError::ValidationError(
            "Subagent name cannot be empty".to_string(),
        ));
    }

    if subagent.description.is_empty() {
        return Err(AppError::ValidationError(
            "Subagent description cannot be empty".to_string(),
        ));
    }

    if subagent.capabilities.is_empty() {
        return Err(AppError::ValidationError(
            "Subagent must have at least one capability".to_string(),
        ));
    }

    validate_subagent_config(&subagent.config)?;

    Ok(())
}

/// Pure function to validate subagent task
pub fn validate_subagent_task(task: &SubagentTask) -> Result<(), AppError> {
    if task.input.is_empty() {
        return Err(AppError::ValidationError(
            "Task input cannot be empty".to_string(),
        ));
    }

    if task.input.len() > 100000 {
        return Err(AppError::ValidationError(
            "Task input too long (max 100000 characters)".to_string(),
        ));
    }

    if task.subagent_id.is_empty() {
        return Err(AppError::ValidationError(
            "Subagent ID cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate task context
pub fn validate_task_context(
    file_path: Option<&str>,
    repository: Option<&str>,
) -> Result<(), AppError> {
    if let Some(path) = file_path {
        if path.is_empty() {
            return Err(AppError::ValidationError(
                "File path cannot be empty".to_string(),
            ));
        }
    }

    if let Some(repo) = repository {
        if repo.is_empty() {
            return Err(AppError::ValidationError(
                "Repository cannot be empty".to_string(),
            ));
        }

        // Validate repository format (owner/repo)
        let parts: Vec<&str> = repo.split('/').collect();
        if parts.len() != 2 {
            return Err(AppError::ValidationError(
                "Repository must be in format 'owner/repo'".to_string(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::subagents::domain::models::subagent::{AgentType, TaskContext, TaskType};

    #[test]
    fn test_validate_subagent_config_invalid_temperature() {
        let mut config = SubagentConfig::default();
        config.temperature = 3.0;
        assert!(validate_subagent_config(&config).is_err());
    }

    #[test]
    fn test_validate_subagent_config_success() {
        let config = SubagentConfig::default();
        assert!(validate_subagent_config(&config).is_ok());
    }

    #[test]
    fn test_validate_subagent_empty_name() {
        let agent = Subagent::new(
            String::new(),
            AgentType::CodeReviewer,
            "Description".to_string(),
        );
        assert!(validate_subagent(&agent).is_err());
    }

    #[test]
    fn test_validate_subagent_task_empty_input() {
        let context = TaskContext::new();
        let task = SubagentTask::new(
            "agent-1".to_string(),
            TaskType::CodeReview,
            String::new(),
            context,
        );
        assert!(validate_subagent_task(&task).is_err());
    }

    #[test]
    fn test_validate_task_context_invalid_repo() {
        assert!(validate_task_context(None, Some("invalid")).is_err());
    }

    #[test]
    fn test_validate_task_context_success() {
        assert!(validate_task_context(Some("file.rs"), Some("owner/repo")).is_ok());
    }
}
