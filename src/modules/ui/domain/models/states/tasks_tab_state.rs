use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct TasksTabState {
    pub tasks: Vec<Task>,
    pub selected_task_index: usize,
    pub filter: TaskFilter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<String>,
    pub subtasks: Vec<Subtask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtask {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
}

impl TabState for TasksTabState {
    fn tab(&self) -> Tab {
        Tab::Tasks
    }
}
