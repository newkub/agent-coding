//! Command tests

use agent_tui::modules::sandbox::domain::models::{Command, CommandId, SecurityLevel};

#[test]
fn test_command_id_new() {
    let id = CommandId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.to_string().is_empty());
}

#[test]
fn test_command_id_display() {
    let id = CommandId::from_string(uuid::Uuid::new_v4().to_string());
    let _ = format!("{}", id);
}

#[test]
fn test_command_new() {
    let cmd = Command::create(CommandId::from_string(uuid::Uuid::new_v4().to_string()), "ls -la".to_string(), chrono::Utc::now());
    assert_eq!(cmd.command, "ls -la");
    assert_eq!(cmd.security_level, SecurityLevel::Safe);
    assert!(cmd.timeout_ms.is_some());
}

#[test]
fn test_command_with_working_dir() {
    let cmd = Command::create(CommandId::from_string(uuid::Uuid::new_v4().to_string()), "pwd".to_string(), chrono::Utc::now())
        .with_working_dir("/home/user".to_string());
    assert_eq!(cmd.working_dir, Some("/home/user".to_string()));
}

#[test]
fn test_command_with_env() {
    let cmd = Command::create(CommandId::from_string(uuid::Uuid::new_v4().to_string()), "echo $NAME".to_string(), chrono::Utc::now())
        .with_env("NAME".to_string(), "value".to_string());
    assert_eq!(cmd.environment.get("NAME"), Some(&"value".to_string()));
}

#[test]
fn test_command_id_clone() {
    let id1 = CommandId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}

#[test]
fn test_command_serialization() {
    let cmd = Command::create(CommandId::from_string(uuid::Uuid::new_v4().to_string()), "ls".to_string(), chrono::Utc::now())
        .with_working_dir("/home".to_string());
    
    let json = serde_json::to_string(&cmd).unwrap();
    let parsed: Command = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.command, "ls");
}
