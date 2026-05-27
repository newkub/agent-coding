use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A participant in a collaboration session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: ParticipantId,
    pub name: String,
    pub role: ParticipantRole,
    pub joined_at: DateTime<Utc>,
    pub is_online: bool,
    pub cursor_position: Option<CursorPosition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParticipantId(pub String);

impl ParticipantId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticipantRole {
    Owner,
    Editor,
    Viewer,
}

/// Cursor position for real-time collaboration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPosition {
    pub file_path: Option<String>,
    pub line: u32,
    pub column: u32,
}

/// Collaboration session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub id: CollaborationId,
    pub name: String,
    pub participants: Vec<Participant>,
    pub session_id: String, // The AI session being collaborated on
    pub created_at: DateTime<Utc>,
    pub status: CollaborationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CollaborationId(pub String);

impl CollaborationId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CollaborationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollaborationStatus {
    Active,
    Paused,
    Ended,
}

impl CollaborationSession {
    // Pure constructor - timestamp and ID moved to application layer
    pub fn create(
        id: CollaborationId,
        name: String,
        owner: Participant,
        session_id: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            participants: vec![owner],
            session_id,
            created_at,
            status: CollaborationStatus::Active,
        }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    pub fn remove_participant(&mut self, id: &ParticipantId) {
        self.participants.retain(|p| p.id != *id);
    }

    pub fn get_online_participants(&self) -> Vec<&Participant> {
        self.participants.iter().filter(|p| p.is_online).collect()
    }

    pub fn update_cursor(&mut self, participant_id: &ParticipantId, position: CursorPosition) {
        if let Some(p) = self.participants.iter_mut().find(|p| p.id == *participant_id) {
            p.cursor_position = Some(position);
        }
    }
}

/// Shared message in collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedMessage {
    pub id: String,
    pub collaboration_id: CollaborationId,
    pub sender_id: ParticipantId,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: SharedMessageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedMessageType {
    Chat,
    Suggestion,
    Action,
}