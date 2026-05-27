use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Security level for command execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// No restrictions, direct execution
    Unrestricted,
    /// Safe commands only (read-only operations)
    Safe,
    /// Commands run in sandbox
    Sandboxed,
    /// Maximum security, every command requires approval
    Strict,
}

/// A command to be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: CommandId,
    pub command: String,
    pub working_dir: Option<String>,
    pub security_level: SecurityLevel,
    pub environment: std::collections::HashMap<String, String>,
    pub timeout_ms: Option<u64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(pub String);

impl CommandId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CommandId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Command {
    // Pure constructor - timestamp and ID moved to application layer
    pub fn create(
        id: CommandId,
        command: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            command,
            working_dir: None,
            security_level: SecurityLevel::Safe,
            environment: std::collections::HashMap::new(),
            timeout_ms: Some(30_000), // 30s default
            created_at,
        }
    }

    pub fn with_working_dir(mut self, dir: String) -> Self {
        self.working_dir = Some(dir);
        self
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_id: CommandId,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub executed_at: DateTime<Utc>,
    pub sandboxed: bool,
}

impl CommandResult {
    pub fn success(&self) -> bool {
        self.exit_code == Some(0)
    }
}

/// Approval rule for automatic command handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRule {
    pub id: RuleId,
    pub name: String,
    pub pattern: String, // regex pattern
    pub action: RuleAction,
    pub security_level: SecurityLevel,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuleId(pub String);

impl RuleId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleAction {
    AutoApprove,
    AutoReject,
    RequireApproval,
    RunInSandbox,
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub image: String,
    pub memory_limit_mb: u64,
    pub cpu_limit: f64,
    pub network_enabled: bool,
    pub read_only_filesystem: bool,
    pub allowed_paths: Vec<String>,
    pub denied_paths: Vec<String>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            image: "alpine:latest".to_string(),
            memory_limit_mb: 512,
            cpu_limit: 1.0,
            network_enabled: false,
            read_only_filesystem: true,
            allowed_paths: Vec::new(),
            denied_paths: vec![
                "/etc".to_string(),
                "/root".to_string(),
                "/home".to_string(),
            ],
        }
    }
}