use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Subagent entity - specialized AI agent for specific tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subagent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub agent_type: AgentType,
    pub capabilities: Vec<String>,
    pub config: SubagentConfig,
    pub status: SubagentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentType {
    CodeReviewer,
    BugHunter,
    Refactorer,
    Documenter,
    Tester,
    SecurityAuditor,
    PerformanceOptimizer,
    DependencyManager,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubagentStatus {
    Idle,
    Active,
    Busy,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubagentConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub system_prompt: String,
    pub tools: Vec<String>,
    pub parameters: HashMap<String, String>,
}

impl Default for SubagentConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_tokens: Some(4096),
            system_prompt: String::new(),
            tools: Vec::new(),
            parameters: HashMap::new(),
        }
    }
}

/// Subagent task entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubagentTask {
    pub id: String,
    pub subagent_id: String,
    pub task_type: TaskType,
    pub input: String,
    pub context: TaskContext,
    pub output: Option<String>,
    pub status: TaskStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    CodeReview,
    BugDetection,
    Refactoring,
    Documentation,
    TestGeneration,
    SecurityAudit,
    PerformanceAnalysis,
    DependencyUpdate,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskContext {
    pub session_id: Option<String>,
    pub file_path: Option<String>,
    pub repository: Option<String>,
    pub branch: Option<String>,
    pub additional_data: HashMap<String, String>,
}

impl Subagent {
    pub fn new(name: String, agent_type: AgentType, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            agent_type,
            capabilities: Vec::new(),
            config: SubagentConfig::default(),
            status: SubagentStatus::Idle,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_config(mut self, config: SubagentConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub const fn is_available(&self) -> bool {
        matches!(self.status, SubagentStatus::Idle)
    }

    pub const fn can_handle(&self, task_type: &TaskType) -> bool {
        match (&self.agent_type, task_type) {
            (AgentType::CodeReviewer, TaskType::CodeReview) => true,
            (AgentType::BugHunter, TaskType::BugDetection) => true,
            (AgentType::Refactorer, TaskType::Refactoring) => true,
            (AgentType::Documenter, TaskType::Documentation) => true,
            (AgentType::Tester, TaskType::TestGeneration) => true,
            (AgentType::SecurityAuditor, TaskType::SecurityAudit) => true,
            (AgentType::PerformanceOptimizer, TaskType::PerformanceAnalysis) => true,
            (AgentType::DependencyManager, TaskType::DependencyUpdate) => true,
            _ => false,
        }
    }
}

impl SubagentTask {
    pub fn new(
        subagent_id: String,
        task_type: TaskType,
        input: String,
        context: TaskContext,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            subagent_id,
            task_type,
            input,
            context,
            output: None,
            status: TaskStatus::Pending,
            started_at: now,
            completed_at: None,
            error: None,
            metadata: HashMap::new(),
        }
    }

    pub fn complete(&mut self, output: String) {
        self.output = Some(output);
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn fail(&mut self, error: String) {
        self.error = Some(error);
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Utc::now());
    }

    pub const fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled)
    }
}

impl Default for TaskContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskContext {
    pub fn new() -> Self {
        Self {
            session_id: None,
            file_path: None,
            repository: None,
            branch: None,
            additional_data: HashMap::new(),
        }
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_file(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn with_repository(mut self, repository: String) -> Self {
        self.repository = Some(repository);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
