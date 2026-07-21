//! Session domain tests - 100% coverage target

use agent_tui::modules::session::domain::models::*;
use agent_tui::modules::session::domain::operations::*;
use agent_tui::modules::session::types::*;
use chrono::Utc;

fn create_test_message(role: MessageRole, content: String) -> Message {
    Message::create(uuid::Uuid::new_v4().to_string(), role, content, Utc::now())
}

fn create_test_session(name: String) -> Session {
    Session::create(
        SessionId::from_string(uuid::Uuid::new_v4().to_string()),
        name,
        Utc::now(),
        Utc::now(),
    )
}

#[test]
fn test_session_id_new() {
    let id = SessionId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.as_str().is_empty());
}

#[test]
fn test_session_id_from_string() {
    let id = SessionId::from_string("test-id".to_string());
    assert_eq!(id.as_str(), "test-id");
}

#[test]
fn test_session_id_display() {
    let id = SessionId::from_string("display-test".to_string());
    assert_eq!(format!("{}", id), "display-test");
}

#[test]
fn test_session_id_default() {
    let id = SessionId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.as_str().is_empty());
}

#[test]
fn test_message_role_as_str() {
    assert_eq!(MessageRole::System.as_str(), "system");
    assert_eq!(MessageRole::User.as_str(), "user");
    assert_eq!(MessageRole::Assistant.as_str(), "assistant");
    assert_eq!(MessageRole::Tool.as_str(), "tool");
}

#[test]
fn test_message_new() {
    let msg = create_test_message(MessageRole::User, "Hello".to_string());
    assert_eq!(msg.role, MessageRole::User);
    assert_eq!(msg.content, "Hello");
    assert!(!msg.id.is_empty());
}

#[test]
fn test_message_system() {
    let msg = create_test_message(MessageRole::System, "System prompt".to_string());
    assert_eq!(msg.role, MessageRole::System);
    assert_eq!(msg.content, "System prompt");
}

#[test]
fn test_message_user() {
    let msg = create_test_message(MessageRole::User, "User message".to_string());
    assert_eq!(msg.role, MessageRole::User);
    assert_eq!(msg.content, "User message");
}

#[test]
fn test_message_assistant() {
    let msg = create_test_message(MessageRole::Assistant, "Assistant response".to_string());
    assert_eq!(msg.role, MessageRole::Assistant);
    assert_eq!(msg.content, "Assistant response");
}

#[test]
fn test_message_with_metadata() {
    let mut msg = create_test_message(MessageRole::User, "Test".to_string());
    msg.metadata = Some(MessageMetadata {
        model: Some("gpt-4".to_string()),
        tokens_used: Some(100),
        tool_calls: None,
    });
    assert!(msg.metadata.is_some());
    assert_eq!(
        msg.metadata.as_ref().unwrap().model.as_deref(),
        Some("gpt-4")
    );
}

#[test]
fn test_session_new() {
    let session = create_test_session("Test Session".to_string());
    assert_eq!(session.name, "Test Session");
    assert!(session.messages.is_empty());
    assert!(!session.id.as_str().is_empty());
}

#[test]
fn test_session_add_message() {
    let mut session = create_test_session("Test".to_string());
    let msg = create_test_message(MessageRole::User, "Hello".to_string());
    let count = session.message_count();
    session.add_message(msg);
    assert_eq!(session.message_count(), count + 1);
}

#[test]
fn test_session_total_tokens() {
    let mut session = create_test_session("Test".to_string());
    let mut msg1 = create_test_message(MessageRole::User, "Hello".to_string());
    msg1.metadata = Some(MessageMetadata {
        tokens_used: Some(10),
        ..Default::default()
    });
    let mut msg2 = create_test_message(MessageRole::Assistant, "Hi".to_string());
    msg2.metadata = Some(MessageMetadata {
        tokens_used: Some(20),
        ..Default::default()
    });
    session.add_message(msg1);
    session.add_message(msg2);
    assert_eq!(session.total_tokens(), 30);
}

#[test]
fn test_session_metadata_default() {
    let meta = SessionMetadata::default();
    assert!(meta.provider.is_none());
    assert!(meta.model.is_none());
    assert!(meta.tags.is_empty());
    assert!(!meta.pinned);
}

#[test]
fn test_session_metadata_with_values() {
    let mut meta = SessionMetadata::default();
    meta.provider = Some("openai".to_string());
    meta.model = Some("gpt-4".to_string());
    meta.tags = vec!["test".to_string(), "demo".to_string()];
    meta.pinned = true;
    assert_eq!(meta.provider.as_deref(), Some("openai"));
    assert_eq!(meta.model.as_deref(), Some("gpt-4"));
    assert_eq!(meta.tags.len(), 2);
    assert!(meta.pinned);
}

#[test]
fn test_validate_session_name_valid() {
    assert!(validate_session_name("Valid Name").is_ok());
    assert!(validate_session_name("Test Session 123").is_ok());
    assert!(validate_session_name("a").is_ok());
}

#[test]
fn test_validate_session_name_empty() {
    assert!(matches!(
        validate_session_name(""),
        Err(SessionValidationError::EmptyName)
    ));
    assert!(matches!(
        validate_session_name("   "),
        Err(SessionValidationError::EmptyName)
    ));
}

#[test]
fn test_validate_session_name_too_long() {
    let long_name = "a".repeat(256);
    assert!(matches!(
        validate_session_name(&long_name),
        Err(SessionValidationError::NameTooLong)
    ));
}

#[test]
fn test_validate_session_name_invalid_chars() {
    assert!(matches!(
        validate_session_name("test/name"),
        Err(SessionValidationError::InvalidCharacters)
    ));
    assert!(matches!(
        validate_session_name("test\\name"),
        Err(SessionValidationError::InvalidCharacters)
    ));
}

#[test]
fn test_calculate_session_stats() {
    let mut session = create_test_session("Test".to_string());
    let mut msg = create_test_message(MessageRole::User, "Hello".to_string());
    msg.metadata = Some(MessageMetadata {
        tokens_used: Some(50),
        ..Default::default()
    });
    session.add_message(msg);

    let stats = calculate_session_stats(&session);
    assert_eq!(stats.message_count, 1);
    assert_eq!(stats.total_tokens, 50);
}

#[test]
fn test_create_session_valid() {
    let session = create_session("New Session".to_string());
    assert!(session.is_ok());
    assert_eq!(session.unwrap().name, "New Session");
}

#[test]
fn test_create_session_invalid() {
    let result = create_session("".to_string());
    assert!(result.is_err());
}

#[test]
fn test_add_message_to_session() {
    let session = create_test_session("Test".to_string());
    let msg = create_test_message(MessageRole::User, "Hello".to_string());
    let new_session = add_message(&session, msg);
    assert_eq!(new_session.message_count(), 1);
}

#[test]
fn test_message_metadata_default() {
    let meta = MessageMetadata::default();
    assert!(meta.model.is_none());
    assert!(meta.tokens_used.is_none());
    assert!(meta.tool_calls.is_none());
}

#[test]
fn test_tool_call_serialization() {
    let tool = ToolCall {
        name: "test".to_string(),
        arguments: serde_json::json!({"key": "value"}),
        result: Some("success".to_string()),
    };
    let json = serde_json::to_string(&tool).unwrap();
    let parsed: ToolCall = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "test");
    assert_eq!(parsed.result.as_deref(), Some("success"));
}

#[test]
fn test_session_id_eq() {
    let id1 = SessionId::from_string("same".to_string());
    let id2 = SessionId::from_string("same".to_string());
    let id3 = SessionId::from_string("different".to_string());
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_session_id_clone() {
    let id1 = SessionId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}

#[test]
fn test_message_clone() {
    let msg1 = create_test_message(MessageRole::User, "test".to_string());
    let msg2 = msg1.clone();
    assert_eq!(msg1.id, msg2.id);
    assert_eq!(msg1.content, msg2.content);
}

#[test]
fn test_session_clone() {
    let session1 = create_test_session("Test".to_string());
    let session2 = session1.clone();
    assert_eq!(session1.id, session2.id);
    assert_eq!(session1.name, session2.name);
}

#[test]
fn test_session_validation_error_display() {
    assert_eq!(
        format!("{}", SessionValidationError::EmptyName),
        "Session name cannot be empty"
    );
    assert_eq!(
        format!("{}", SessionValidationError::NameTooLong),
        "Session name cannot exceed 255 characters"
    );
    assert_eq!(
        format!("{}", SessionValidationError::InvalidCharacters),
        "Session name contains invalid characters"
    );
}
