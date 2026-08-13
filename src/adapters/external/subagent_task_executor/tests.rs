use super::*;
use crate::modules::subagents::domain::models::subagent::{TaskContext, TaskType};

#[tokio::test]
async fn test_cancel_task() {
    let executor = DefaultSubagentTaskExecutor::new();
    let context = TaskContext::new();
    let task = SubagentTask::new(
        "agent-1".to_string(),
        TaskType::CodeReview,
        "Review this code".to_string(),
        context,
    );

    let mut tasks = executor.tasks.write().await;
    let task_id = task.id.clone();
    tasks.insert(task_id.clone(), task);
    drop(tasks);

    let result = executor.cancel_task(&task_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_task_without_api_key_fails() {
    // Ensure no key is present so the executor fails fast instead of
    // producing placeholder output.
    std::env::remove_var("OPENAI_API_KEY");
    let executor = DefaultSubagentTaskExecutor::new();
    let context = TaskContext::new();
    let mut task = SubagentTask::new(
        "agent-1".to_string(),
        TaskType::CodeReview,
        "Review this code".to_string(),
        context,
    );

    let result = executor.execute_task(&mut task).await;
    assert!(result.is_err(), "executor must fail without an API key");
    assert_eq!(task.status, TaskStatus::Failed);
    assert!(task.error.is_some());
}
