//! Subagent domain model - Implementation

use chrono::Utc;
use std::collections::HashMap;

use super::types::{
    AgentType, Subagent, SubagentConfig, SubagentStatus, SubagentTask, TaskContext, TaskStatus,
    TaskType,
};

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
        matches!(
            self.status,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
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
