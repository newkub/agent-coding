use serde::{Deserialize, Serialize};

/// Toast notification kind
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

/// Toast notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastNotification {
    pub kind: ToastKind,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
