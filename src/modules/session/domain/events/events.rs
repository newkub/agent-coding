use serde::{Deserialize, Serialize};

/// Session Events for event sourcing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEvent {
    Created {
        session_id: String,
    },
    Updated {
        session_id: String,
    },
    MessageAdded {
        session_id: String,
        message_index: usize,
    },
    Closed {
        session_id: String,
    },
}
