//! Collaboration Message tests

use agent_tui::modules::collaboration::domain::models::*;
use chrono::Utc;

#[test]
fn test_cursor_position_new() {
    let pos = CursorPosition {
        file_path: Some("test.rs".to_string()),
        line: 10,
        column: 5,
    };

    assert_eq!(pos.line, 10);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_shared_message_new() {
    let msg = SharedMessage {
        id: "msg-1".to_string(),
        collaboration_id: CollaborationId::from_string(uuid::Uuid::new_v4().to_string()),
        sender_id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
        content: "Hello".to_string(),
        timestamp: Utc::now(),
        message_type: SharedMessageType::Chat,
    };

    assert_eq!(msg.content, "Hello");
    assert!(matches!(msg.message_type, SharedMessageType::Chat));
}

#[test]
fn test_shared_message_type_variants() {
    assert!(matches!(SharedMessageType::Chat, SharedMessageType::Chat));
    assert!(matches!(
        SharedMessageType::Suggestion,
        SharedMessageType::Suggestion
    ));
    assert!(matches!(
        SharedMessageType::Action,
        SharedMessageType::Action
    ));
}
