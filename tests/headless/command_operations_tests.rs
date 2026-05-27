use agent_tui::modules::headless::domain::operations::command_operations::{
    parse_command, format_output, truncate_output, extract_arguments
};
use agent_tui::modules::headless::domain::models::command::OutputFormat;

#[test]
fn test_parse_command_chat() {
    assert_eq!(parse_command("/chat hello"), Ok(agent_tui::modules::headless::domain::models::command::CommandType::Chat));
}

#[test]
fn test_parse_command_read() {
    assert_eq!(parse_command("/read file.txt"), Ok(agent_tui::modules::headless::domain::models::command::CommandType::FileRead));
}

#[test]
fn test_parse_command_default() {
    assert_eq!(parse_command("hello world"), Ok(agent_tui::modules::headless::domain::models::command::CommandType::Chat));
}

#[test]
fn test_format_output_text() {
    let output = format_output("test", &OutputFormat::Text, false);
    assert_eq!(output, "test");
}

#[test]
fn test_format_output_json() {
    let output = format_output("test", &OutputFormat::Json, false);
    assert!(output.contains("\"output\""));
}

#[test]
fn test_truncate_output() {
    let output = truncate_output("hello world", 5);
    assert_eq!(output, "hello... (truncated)");
}

#[test]
fn test_extract_arguments() {
    let args = extract_arguments("/read file.txt --line 10");
    assert_eq!(args, vec!["file.txt", "--line", "10"]);
}
