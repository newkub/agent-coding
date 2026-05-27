use agent_tui::modules::subagents::domain::validators::subagent_validators;
use agent_tui::modules::subagents::domain::models::subagent::{Subagent, SubagentConfig, SubagentTask, TaskContext, TaskType, AgentType};

#[test]
fn test_validate_subagent_config_invalid_temperature() {
    let mut config = SubagentConfig::default();
    config.temperature = 3.0;
    assert!(subagent_validators::validate_subagent_config(&config).is_err());
}

#[test]
fn test_validate_subagent_config_success() {
    let config = SubagentConfig::default();
    assert!(subagent_validators::validate_subagent_config(&config).is_ok());
}

#[test]
fn test_validate_subagent_empty_name() {
    let agent = Subagent::new(
        String::new(),
        AgentType::CodeReviewer,
        "Description".to_string(),
    );
    assert!(subagent_validators::validate_subagent(&agent).is_err());
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
    assert!(subagent_validators::validate_subagent_task(&task).is_err());
}

#[test]
fn test_validate_task_context_invalid_repo() {
    assert!(subagent_validators::validate_task_context(None, Some("invalid")).is_err());
}

#[test]
fn test_validate_task_context_success() {
    assert!(subagent_validators::validate_task_context(Some("file.rs"), Some("owner/repo")).is_ok());
}
