use super::*;

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
fn test_message_clone() {
    let msg1 = create_test_message(MessageRole::User, "test".to_string());
    let msg2 = msg1.clone();
    assert_eq!(msg1.id, msg2.id);
    assert_eq!(msg1.content, msg2.content);
}
