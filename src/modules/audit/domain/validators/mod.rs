use super::models::{AuditAction, AuditEntry};

/// Pure validator: Validate audit entry structure
pub fn validate_entry(entry: &AuditEntry) -> Result<(), ValidationError> {
    if entry.actor.id.is_empty() {
        return Err(ValidationError::InvalidActor(
            "Actor ID cannot be empty".to_string(),
        ));
    }

    if entry.resource.id.is_empty() {
        return Err(ValidationError::InvalidResource(
            "Resource ID cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Pure validator: Validate action parameters
pub fn validate_action(action: &AuditAction) -> Result<(), ValidationError> {
    match action {
        AuditAction::FileRead { path }
        | AuditAction::FileWrite { path }
        | AuditAction::FileDelete { path }
            if path.is_empty() =>
        {
            return Err(ValidationError::InvalidAction(
                "File path cannot be empty".to_string(),
            ));
        }
        AuditAction::CommandExecute { command }
        | AuditAction::CommandApprove { command }
        | AuditAction::CommandReject { command }
            if command.is_empty() =>
        {
            return Err(ValidationError::InvalidAction(
                "Command cannot be empty".to_string(),
            ));
        }
        AuditAction::GitCommit { message, files } => {
            if message.is_empty() {
                return Err(ValidationError::InvalidAction(
                    "Commit message cannot be empty".to_string(),
                ));
            }
            if files.is_empty() {
                return Err(ValidationError::InvalidAction(
                    "Commit must include at least one file".to_string(),
                ));
            }
        }
        AuditAction::GitBranch { name } if name.is_empty() => {
            return Err(ValidationError::InvalidAction(
                "Branch name cannot be empty".to_string(),
            ));
        }
        AuditAction::SessionCreate { name } if name.is_empty() => {
            return Err(ValidationError::InvalidAction(
                "Session name cannot be empty".to_string(),
            ));
        }
        AuditAction::SessionDelete { id } if id.is_empty() => {
            return Err(ValidationError::InvalidAction(
                "Session ID cannot be empty".to_string(),
            ));
        }
        AuditAction::MessageSend { session_id } if session_id.is_empty() => {
            return Err(ValidationError::InvalidAction(
                "Session ID cannot be empty".to_string(),
            ));
        }
        AuditAction::AiRequest { model, tokens } => {
            if model.is_empty() {
                return Err(ValidationError::InvalidAction(
                    "Model name cannot be empty".to_string(),
                ));
            }
            if *tokens == 0 {
                return Err(ValidationError::InvalidAction(
                    "Token count must be greater than 0".to_string(),
                ));
            }
        }
        AuditAction::ConfigChange { key } if key.is_empty() => {
            return Err(ValidationError::InvalidAction(
                "Configuration key cannot be empty".to_string(),
            ));
        }
        AuditAction::PluginLoad { name } if name.is_empty() => {
            return Err(ValidationError::InvalidAction(
                "Plugin name cannot be empty".to_string(),
            ));
        }
        _ => {}
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidActor(String),
    InvalidResource(String),
    InvalidAction(String),
}
