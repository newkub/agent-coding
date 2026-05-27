use serde::{Deserialize, Serialize};

/// Share Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ShareEvent {
    SessionShared { session_id: String, target: String },
    ExportCompleted { format: String },
}