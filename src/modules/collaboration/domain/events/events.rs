use serde::{Deserialize, Serialize};

/// Collaboration Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum CollaborationEvent {
    SessionJoined { session_id: String, user_id: String },
    SessionLeft { session_id: String, user_id: String },
    ContentShared { session_id: String },
    CursorMoved { session_id: String, user_id: String, position: (u16, u16) },
}