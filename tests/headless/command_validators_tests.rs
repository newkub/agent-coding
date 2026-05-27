use agent_tui::modules::headless::domain::validators::command_validators;
use agent_tui::modules::headless::domain::models::command::{HeadlessCommand, HeadlessConfig, CommandContext, CommandType};

#[test]
fn test_validate_command_input_empty() {
    assert!(command_validators::validate_command_input("").is_err());
}

#[test]
fn test_validate_command_input_too_long() {
    let long_input = "a".repeat(10001);
    assert!(command_validators::validate_command_input(&long_input).is_err());
}

#[test]
fn test_validate_command_input_success() {
    assert!(command_validators::validate_command_input("test command").is_ok());
}

#[test]
fn test_validate_command_context_relative_path() {
    assert!(command_validators::validate_command_context("relative/path").is_err());
}

#[test]
fn test_validate_command_context_success() {
    #[cfg(windows)]
    let path = "C:\\absolute\\path";
    #[cfg(not(windows))]
    let path = "/absolute/path";
    assert!(command_validators::validate_command_context(path).is_ok());
}

#[test]
fn test_validate_headless_config_invalid_max_length() {
    let mut config = HeadlessConfig::default();
    config.max_output_length = Some(0);
    assert!(command_validators::validate_headless_config(&config).is_err());
}

#[test]
fn test_validate_command_for_headless_missing_session() {
    let context = CommandContext::new("/test".to_string());
    let command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);
    assert!(command_validators::validate_command_for_headless(&command).is_err());
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
    assert!(command_validators::validate_command_for_headless(&command).is_ok());
}
