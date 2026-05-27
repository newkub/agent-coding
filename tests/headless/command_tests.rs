use agent_tui::modules::headless::domain::models::command::{
    HeadlessCommand, CommandType, CommandContext, CommandStatus, HeadlessConfig, OutputFormat
};

#[test]
fn test_headless_command_creation() {
    let context = CommandContext::new("/test".to_string());
    let command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);
    assert_eq!(command.status, CommandStatus::Pending);
    assert!(command.is_interactive());
}

#[test]
fn test_headless_command_complete() {
    let context = CommandContext::new("/test".to_string());
    let mut command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);
    command.complete("Response".to_string());
    assert_eq!(command.status, CommandStatus::Completed);
}

#[test]
fn test_headless_command_fail() {
    let context = CommandContext::new("/test".to_string());
    let mut command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);
    command.fail("Error".to_string());
    assert_eq!(command.status, CommandStatus::Failed);
}

#[test]
fn test_command_context_with_session() {
    let context = CommandContext::new("/test".to_string())
        .with_session("session-123".to_string());
    assert_eq!(context.session_id, Some("session-123".to_string()));
}

#[test]
fn test_headless_config_default() {
    let config = HeadlessConfig::default();
    assert_eq!(config.output_format, OutputFormat::Text);
    assert!(!config.stream_responses);
}
