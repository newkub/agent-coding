//! Collaboration serialization tests

use agent_tui::modules::collaboration::domain::models::*;
use chrono::Utc;

fn create_test_owner() -> Participant {
    Participant {
        id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
        name: "Owner".to_string(),
        role: ParticipantRole::Owner,
        joined_at: Utc::now(),
        is_online: true,
        cursor_position: None,
    }
}

fn create_test_session(name: String, owner: Participant, session_id: String) -> CollaborationSession {
    CollaborationSession::create(
        CollaborationId::from_string(uuid::Uuid::new_v4().to_string()),
        name,
        owner,
        session_id,
        Utc::now(),
    )
}

#[test]
fn test_collaboration_session_serialization() {
    let owner = create_test_owner();
    let session = create_test_session(
        "Test Session".to_string(),
        owner,
        "session-1".to_string(),
    );
    
    let json = serde_json::to_string(&session).unwrap();
    let parsed: CollaborationSession = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "Test Session");
}

#[test]
fn test_shared_message_serialization() {
    let msg = SharedMessage {
        id: "msg-1".to_string(),
        collaboration_id: CollaborationId::from_string(uuid::Uuid::new_v4().to_string()),
        sender_id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
        content: "Hello".to_string(),
        timestamp: Utc::now(),
        message_type: SharedMessageType::Chat,
    };
    
    let json = serde_json::to_string(&msg).unwrap();
    let parsed: SharedMessage = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.content, "Hello");
}

#[test]
fn test_cursor_position_serialization() {
    let pos = CursorPosition {
        file_path: Some("test.rs".to_string()),
        line: 10,
        column: 5,
    };
    
    let json = serde_json::to_string(&pos).unwrap();
    let parsed: CursorPosition = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.line, 10);
}
