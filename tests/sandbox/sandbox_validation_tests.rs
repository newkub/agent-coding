//! Validation tests

use agent_tui::modules::sandbox::domain::operations::{validate_command, CommandValidationError};

#[test]
fn test_validate_command_valid() {
    assert!(validate_command("ls -la").is_ok());
    assert!(validate_command("echo hello").is_ok());
}

#[test]
fn test_validate_command_empty() {
    assert!(matches!(
        validate_command(""),
        Err(CommandValidationError::EmptyCommand)
    ));
}

#[test]
fn test_validate_command_too_long() {
    let long_cmd = "a".repeat(10001);
    assert!(matches!(
        validate_command(&long_cmd),
        Err(CommandValidationError::CommandTooLong)
    ));
}

#[test]
fn test_validate_command_invalid_chars() {
    assert!(matches!(
        validate_command("test\0invalid"),
        Err(CommandValidationError::InvalidCharacters)
    ));
}

#[test]
fn test_command_validation_error_display() {
    assert_eq!(
        format!("{}", CommandValidationError::EmptyCommand),
        "Command cannot be empty"
    );
    assert_eq!(
        format!("{}", CommandValidationError::CommandTooLong),
        "Command exceeds maximum length"
    );
    assert_eq!(
        format!("{}", CommandValidationError::InvalidCharacters),
        "Command contains invalid characters"
    );
}
