//! Subagent domain model - Tests

use super::types::*;

#[test]
fn test_subagent_creation() {
    let agent = Subagent::new(
        "Code Reviewer".to_string(),
        AgentType::CodeReviewer,
        "Reviews code for quality".to_string(),
    );
    assert_eq!(agent.name, "Code Reviewer");
    assert!(agent.is_available());
}

#[test]
fn test_subagent_can_handle() {
    let agent = Subagent::new(
        "Code Reviewer".to_string(),
        AgentType::CodeReviewer,
        "Reviews code".to_string(),
    );
    assert!(agent.can_handle(&TaskType::CodeReview));
    assert!(!agent.can_handle(&TaskType::BugDetection));
}

#[test]
fn test_subagent_task_creation() {
    let context = TaskContext::new();
    let task = SubagentTask::new(
        "agent-1".to_string(),
        TaskType::CodeReview,
        "Review this code".to_string(),
        context,
    );
    assert_eq!(task.status, TaskStatus::Pending);
}

#[test]
fn test_subagent_task_complete() {
    let context = TaskContext::new();
    let mut task = SubagentTask::new(
        "agent-1".to_string(),
        TaskType::CodeReview,
        "Review this code".to_string(),
        context,
    );
    task.complete("Code looks good".to_string());
    assert_eq!(task.status, TaskStatus::Completed);
    assert!(task.is_completed());
}
