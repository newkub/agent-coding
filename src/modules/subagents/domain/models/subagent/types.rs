//! Subagent domain model - Types

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
