use serde::{Deserialize, Serialize};

/// Sandbox Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum SandboxEvent {
    SandboxCreated { sandbox_id: String },
    CommandExecuted { sandbox_id: String, command: String },
    SandboxDestroyed { sandbox_id: String },
}