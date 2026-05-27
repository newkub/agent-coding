use super::models::{CollaborationSession, Participant, SharedMessage};

/// Pure domain operation: Validate participant name
pub fn validate_participant_name(name: &str) -> Result<(), ParticipantValidationError> {
    if name.trim().is_empty() {
        return Err(ParticipantValidationError::EmptyName);
    }
    if name.len() > 100 {
        return Err(ParticipantValidationError::NameTooLong);
    }
    Ok(())
}

/// Pure domain operation: Calculate collaboration statistics
pub fn calculate_collaboration_stats(session: &CollaborationSession) -> CollaborationStats {
    CollaborationStats {
        total_participants: session.participants.len(),
        online_participants: session.get_online_participants().len(),
        message_count: 0, // Would need message repository for accurate count
        created_at: session.created_at,
        status: session.status,
    }
}

/// Pure domain operation: Check if participant can join session
pub fn can_participant_join(session: &CollaborationSession, participant: &Participant) -> bool {
    // Check if participant already exists
    if session.participants.iter().any(|p| p.id == participant.id) {
        return false;
    }
    
    // Check if session is active
    if session.status != crate::modules::collaboration::domain::models::CollaborationStatus::Active {
        return false;
    }
    
    true
}

/// Pure domain operation: Generate participant display name
pub fn generate_display_name(participant: &Participant) -> String {
    match participant.role {
        crate::modules::collaboration::domain::models::ParticipantRole::Owner => {
            format!("{} (Owner)", participant.name)
        }
        crate::modules::collaboration::domain::models::ParticipantRole::Editor => {
            format!("{} (Editor)", participant.name)
        }
        crate::modules::collaboration::domain::models::ParticipantRole::Viewer => {
            format!("{} (Viewer)", participant.name)
        }
    }
}

/// Pure domain operation: Format message for display
pub fn format_message(message: &SharedMessage) -> String {
    let timestamp = message.timestamp.format("%H:%M:%S").to_string();
    let type_prefix = match message.message_type {
        crate::modules::collaboration::domain::models::SharedMessageType::Chat => "",
        crate::modules::collaboration::domain::models::SharedMessageType::Suggestion => "[Suggestion] ",
        crate::modules::collaboration::domain::models::SharedMessageType::Action => "[Action] ",
    };
    format!("[{}] {}{}", timestamp, type_prefix, message.content)
}

/// Pure domain operation: Check if cursor positions conflict
pub fn cursor_positions_conflict(pos1: &Option<crate::modules::collaboration::domain::models::CursorPosition>, pos2: &Option<crate::modules::collaboration::domain::models::CursorPosition>) -> bool {
    match (pos1, pos2) {
        (Some(p1), Some(p2)) => {
            p1.file_path == p2.file_path && p1.line == p2.line && p1.column == p2.column
        }
        _ => false,
    }
}

/// Collaboration statistics
#[derive(Debug, Clone)]
pub struct CollaborationStats {
    pub total_participants: usize,
    pub online_participants: usize,
    pub message_count: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: crate::modules::collaboration::domain::models::CollaborationStatus,
}

/// Validation errors
#[derive(Debug, Clone)]
pub enum ParticipantValidationError {
    EmptyName,
    NameTooLong,
}

impl std::fmt::Display for ParticipantValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(f, "Participant name cannot be empty"),
            Self::NameTooLong => write!(f, "Participant name cannot exceed 100 characters"),
        }
    }
}
