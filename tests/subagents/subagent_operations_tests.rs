use agent_tui::modules::subagents::domain::operations::subagent_operations::{
    select_subagent_for_task, estimate_task_complexity, calculate_task_priority, generate_system_prompt,
    ComplexityLevel, TaskPriority
};
use agent_tui::modules::subagents::domain::models::subagent::{Subagent, AgentType, SubagentStatus, TaskType};

#[test]
fn test_select_subagent_for_task() {
    let mut agent = Subagent::new(
        "Code Reviewer".to_string(),
        AgentType::CodeReviewer,
        "Reviews code".to_string(),
    );
    agent.status = SubagentStatus::Idle;
    
    let subagents = vec![agent];
    let selected = select_subagent_for_task(&subagents, &TaskType::CodeReview);
    assert!(selected.is_some());
}

#[test]
fn test_estimate_task_complexity() {
    let complexity = estimate_task_complexity(&TaskType::BugDetection, 5000);
    assert_eq!(complexity, ComplexityLevel::High);
}

#[test]
fn test_calculate_task_priority() {
    let priority = calculate_task_priority(&TaskType::SecurityAudit, ComplexityLevel::Medium);
    assert_eq!(priority, TaskPriority::Critical);
}

#[test]
fn test_generate_system_prompt() {
    let prompt = generate_system_prompt(&AgentType::CodeReviewer);
    assert!(prompt.contains("code reviewer"));
}
