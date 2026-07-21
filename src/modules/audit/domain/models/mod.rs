use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Audit log entry for tracking actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: AuditId,
    pub timestamp: DateTime<Utc>,
    pub action: AuditAction,
    pub actor: Actor,
    pub resource: Resource,
    pub metadata: AuditMetadata,
    pub result: AuditResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuditId(pub String);

impl AuditId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The action that was performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    // File operations
    FileRead { path: String },
    FileWrite { path: String },
    FileDelete { path: String },
    // Command operations
    CommandExecute { command: String },
    CommandApprove { command: String },
    CommandReject { command: String },
    // Git operations
    GitCommit { message: String, files: Vec<String> },
    GitPush,
    GitBranch { name: String },
    // Session operations
    SessionCreate { name: String },
    SessionDelete { id: String },
    MessageSend { session_id: String },
    // AI operations
    AiRequest { model: String, tokens: u32 },
    // System operations
    ConfigChange { key: String },
    PluginLoad { name: String },
}

impl AuditAction {
    pub const fn category(&self) -> &'static str {
        match self {
            Self::FileRead { .. } | Self::FileWrite { .. } | Self::FileDelete { .. } => "file",
            Self::CommandExecute { .. }
            | Self::CommandApprove { .. }
            | Self::CommandReject { .. } => "command",
            Self::GitCommit { .. } | Self::GitPush | Self::GitBranch { .. } => "git",
            Self::SessionCreate { .. } | Self::SessionDelete { .. } | Self::MessageSend { .. } => {
                "session"
            }
            Self::AiRequest { .. } => "ai",
            Self::ConfigChange { .. } | Self::PluginLoad { .. } => "system",
        }
    }
}

/// Who performed the action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub type_: ActorType,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActorType {
    User,
    Ai,
    System,
}

/// What resource was affected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub type_: String,
    pub id: String,
    pub path: Option<String>,
}

/// Additional metadata for the audit entry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditMetadata {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub duration_ms: Option<u64>,
    pub extra: std::collections::HashMap<String, String>,
}

impl AuditMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub const fn with_duration(mut self, ms: u64) -> Self {
        self.duration_ms = Some(ms);
        self
    }
}

/// Result of the action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure { error: String },
    Pending,
}

impl AuditEntry {
    // Pure constructor - timestamp and ID moved to application layer
    pub fn create(
        id: AuditId,
        timestamp: DateTime<Utc>,
        action: AuditAction,
        actor: Actor,
        resource: Resource,
    ) -> Self {
        Self {
            id,
            timestamp,
            action,
            actor,
            resource,
            metadata: AuditMetadata::new(),
            result: AuditResult::Success,
        }
    }

    pub fn with_result(mut self, result: AuditResult) -> Self {
        self.result = result;
        self
    }

    pub fn with_metadata(mut self, metadata: AuditMetadata) -> Self {
        self.metadata = metadata;
        self
    }
}
