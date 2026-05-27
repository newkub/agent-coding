use crate::modules::headless::domain::models::command::{HeadlessCommand, HeadlessConfig};
use crate::shared::kernel::result::AppError;

/// Pure function to validate command input
pub fn validate_command_input(input: &str) -> Result<(), AppError> {
    if input.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Command input cannot be empty".to_string(),
        ));
    }

    if input.len() > 10000 {
        return Err(AppError::ValidationError(
            "Command input too long (max 10000 characters)".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate command context
pub fn validate_command_context(working_directory: &str) -> Result<(), AppError> {
    if working_directory.is_empty() {
        return Err(AppError::ValidationError(
            "Working directory cannot be empty".to_string(),
        ));
    }

    // Check if path is valid (basic check)
    let path = std::path::Path::new(working_directory);
    if !path.is_absolute() {
        return Err(AppError::ValidationError(
            "Working directory must be an absolute path".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate headless config
pub fn validate_headless_config(config: &HeadlessConfig) -> Result<(), AppError> {
    if let Some(max_length) = config.max_output_length {
        if max_length == 0 {
            return Err(AppError::ValidationError(
                "Max output length must be greater than 0".to_string(),
            ));
        }
        if max_length > 1_000_000 {
            return Err(AppError::ValidationError(
                "Max output length too large (max 1,000,000 characters)".to_string(),
            ));
        }
    }

    Ok(())
}

/// Pure function to validate command for headless execution
pub fn validate_command_for_headless(command: &HeadlessCommand) -> Result<(), AppError> {
    validate_command_input(&command.input)?;
    validate_command_context(&command.context.working_directory)?;

    if command.requires_session() && command.context.session_id.is_none() {
        return Err(AppError::ValidationError(
            "Command requires a session ID but none provided".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::headless::domain::models::command::{CommandContext, CommandType};

    #[test]
    fn test_validate_command_input_empty() {
        assert!(validate_command_input("").is_err());
    }

    #[test]
    fn test_validate_command_input_too_long() {
        let long_input = "a".repeat(10001);
        assert!(validate_command_input(&long_input).is_err());
    }

    #[test]
    fn test_validate_command_input_success() {
        assert!(validate_command_input("test command").is_ok());
    }

    #[test]
    fn test_validate_command_context_relative_path() {
        assert!(validate_command_context("relative/path").is_err());
    }

    #[test]
    fn test_validate_command_context_success() {
        #[cfg(windows)]
        let path = "C:\\absolute\\path";
        #[cfg(not(windows))]
        let path = "/absolute/path";
        assert!(validate_command_context(path).is_ok());
    }

    #[test]
    fn test_validate_headless_config_invalid_max_length() {
        let mut config = HeadlessConfig::default();
        config.max_output_length = Some(0);
        assert!(validate_headless_config(&config).is_err());
    }

    #[test]
    fn test_validate_command_for_headless_missing_session() {
        let context = CommandContext::new("/test".to_string());
        let command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);
        
        assert!(validate_command_for_headless(&command).is_err());
    }

    #[test]
    fn test_validate_command_for_headless_success() {
        #[cfg(windows)]
        let path = "C:\\test";
        #[cfg(not(windows))]
        let path = "/test";
        let context = CommandContext::new(path.to_string())
            .with_session("session-123".to_string());
        let command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);
        
        assert!(validate_command_for_headless(&command).is_ok());
    }
}
