// Shared kernel - re-export for external use
pub mod shared;

// Modules - re-export for external use
pub mod modules;

// Adapters - private implementation details
pub(crate) mod adapters;

// Presentation - private implementation details
pub(crate) mod presentation;

// Re-exports for common types from shared kernel
pub use shared::kernel::encryption::{EncryptionError, SessionEncryptor};
pub use shared::kernel::result::{AppError, AppResult};
pub use shared::kernel::types::{Column, Tab, UIState};

// Re-exports for commonly used domain types
pub use modules::audit::domain::models::{AuditAction, AuditEntry, AuditResult};
pub use modules::collaboration::domain::models::{
    CollaborationSession, Participant, SharedMessage,
};
pub use modules::session::domain::models::{Message, MessageRole, Session, SessionId};

// Re-exports for configuration types (for testing)
pub use adapters::config::settings::{
    AISettings, AccessibilitySettings, AppSettings, MemorySettings, SecuritySettings, UISettings,
};
