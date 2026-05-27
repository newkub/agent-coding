// Shared kernel - re-export for external use
pub mod shared;

// Modules - re-export for external use
pub mod modules;

// Adapters - private implementation details
pub(crate) mod adapters;

// Presentation - private implementation details
pub(crate) mod presentation;

// Re-exports for common types from shared kernel
pub use shared::kernel::types::{Tab, Column, UIState};
pub use shared::kernel::result::{AppResult, AppError};
pub use shared::kernel::encryption::{SessionEncryptor, EncryptionError};

// Re-exports for commonly used domain types
pub use modules::session::domain::models::{Session, SessionId, Message, MessageRole};
pub use modules::audit::domain::models::{AuditEntry, AuditAction, AuditResult};
pub use modules::collaboration::domain::models::{CollaborationSession, Participant, SharedMessage};

// Re-exports for configuration types (for testing)
pub use adapters::config::settings::{AppSettings, AISettings, UISettings, AccessibilitySettings, MemorySettings, SecuritySettings};
