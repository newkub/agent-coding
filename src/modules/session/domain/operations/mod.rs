use super::models::{Message, Session};

/// Pure domain operation: Validate session name
pub fn validate_session_name(name: &str) -> Result<(), SessionValidationError> {
    if name.trim().is_empty() {
        return Err(SessionValidationError::EmptyName);
    }
    if name.len() > 255 {
        return Err(SessionValidationError::NameTooLong);
    }
    // Check for invalid characters
    if name
        .chars()
        .any(|c| c.is_control() || c == '/' || c == '\\')
    {
        return Err(SessionValidationError::InvalidCharacters);
    }
    Ok(())
}

/// Pure domain operation: Calculate session stats
pub fn calculate_session_stats(session: &Session) -> SessionStats {
    SessionStats {
        message_count: session.message_count(),
        total_tokens: session.total_tokens(),
        created_at: session.created_at,
        updated_at: session.updated_at,
    }
}

/// Session statistics
#[derive(Debug, Clone)]
pub struct SessionStats {
    pub message_count: usize,
    pub total_tokens: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Validation errors
#[derive(Debug, Clone)]
pub enum SessionValidationError {
    EmptyName,
    NameTooLong,
    InvalidCharacters,
}

impl std::fmt::Display for SessionValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(f, "Session name cannot be empty"),
            Self::NameTooLong => write!(f, "Session name cannot exceed 255 characters"),
            Self::InvalidCharacters => write!(f, "Session name contains invalid characters"),
        }
    }
}

/// Pure domain operation: Create new session
pub fn create_session(name: String) -> Result<Session, SessionValidationError> {
    validate_session_name(&name)?;
    let id = crate::modules::session::domain::models::SessionId::from_string(
        uuid::Uuid::new_v4().to_string(),
    );
    let now = chrono::Utc::now();
    Ok(Session::create(id, name, now, now))
}

/// Pure domain operation: Add message to session (returns new session)
pub fn add_message(session: &Session, message: Message) -> Session {
    let mut new_session = session.clone();
    new_session.add_message(message);
    new_session
}
