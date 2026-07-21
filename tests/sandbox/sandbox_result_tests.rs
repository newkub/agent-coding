//! Command Result tests

use agent_tui::modules::sandbox::domain::models::{CommandId, CommandResult};

#[test]
fn test_command_result_success() {
    let result = CommandResult {
        command_id: CommandId::from_string(uuid::Uuid::new_v4().to_string()),
        exit_code: Some(0),
        stdout: "output".to_string(),
        stderr: "".to_string(),
        duration_ms: 100,
        executed_at: chrono::Utc::now(),
        sandboxed: false,
    };
    assert!(result.success());
}

#[test]
fn test_command_result_failure() {
    let result = CommandResult {
        command_id: CommandId::from_string(uuid::Uuid::new_v4().to_string()),
        exit_code: Some(1),
        stdout: "".to_string(),
        stderr: "error".to_string(),
        duration_ms: 50,
        executed_at: chrono::Utc::now(),
        sandboxed: false,
    };
    assert!(!result.success());
}

#[test]
fn test_command_result_no_exit_code() {
    let result = CommandResult {
        command_id: CommandId::from_string(uuid::Uuid::new_v4().to_string()),
        exit_code: None,
        stdout: "".to_string(),
        stderr: "".to_string(),
        duration_ms: 0,
        executed_at: chrono::Utc::now(),
        sandboxed: false,
    };
    assert!(!result.success());
}

#[test]
fn test_command_result_serialization() {
    let result = CommandResult {
        command_id: CommandId::from_string(uuid::Uuid::new_v4().to_string()),
        exit_code: Some(0),
        stdout: "hello".to_string(),
        stderr: "".to_string(),
        duration_ms: 100,
        executed_at: chrono::Utc::now(),
        sandboxed: true,
    };

    let json = serde_json::to_string(&result).unwrap();
    let parsed: CommandResult = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.exit_code, Some(0));
}
