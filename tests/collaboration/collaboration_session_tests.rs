//! Collaboration Session tests

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
fn test_collaboration_status_variants() {
    assert!(matches!(CollaborationStatus::Active, CollaborationStatus::Active));
    assert!(matches!(CollaborationStatus::Paused, CollaborationStatus::Paused));
    assert!(matches!(CollaborationStatus::Ended, CollaborationStatus::Ended));
}

#[test]
fn test_collaboration_session_new() {
    let owner = create_test_owner();
    let session = create_test_session(
        "Test Session".to_string(),
        owner.clone(),
        "session-123".to_string(),
    );
    
    assert_eq!(session.name, "Test Session");
    assert_eq!(session.participants.len(), 1);
    assert_eq!(session.status, CollaborationStatus::Active);
}

#[test]
fn test_collaboration_session_add_participant() {
    let owner = create_test_owner();
    let mut session = create_test_session(
        "Test".to_string(),
        owner,
        "session-1".to_string(),
    );
    
    let new_participant = Participant {
        id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
        name: "Editor".to_string(),
        role: ParticipantRole::Editor,
        joined_at: Utc::now(),
        is_online: true,
        cursor_position: None,
    };
    
    session.add_participant(new_participant);
    assert_eq!(session.participants.len(), 2);
}

#[test]
fn test_collaboration_session_remove_participant() {
    let owner = create_test_owner();
    let mut session = create_test_session(
        "Test".to_string(),
        owner,
        "session-1".to_string(),
    );
    
    let to_remove = Participant {
        id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
        name: "Editor".to_string(),
        role: ParticipantRole::Editor,
        joined_at: Utc::now(),
        is_online: true,
        cursor_position: None,
    };
    
    let removed_id = to_remove.id.clone();
    session.add_participant(to_remove);
    assert_eq!(session.participants.len(), 2);
    
    session.remove_participant(&removed_id);
    assert_eq!(session.participants.len(), 1);
}

#[test]
fn test_collaboration_session_get_online_participants() {
    let owner = create_test_owner();
    let mut session = create_test_session(
        "Test".to_string(),
        owner,
        "session-1".to_string(),
    );
    
    let offline = Participant {
        id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
        name: "Offline".to_string(),
        role: ParticipantRole::Viewer,
        joined_at: Utc::now(),
        is_online: false,
        cursor_position: None,
    };
    
    session.add_participant(offline);
    
    let online = session.get_online_participants();
    assert_eq!(online.len(), 1);
    assert_eq!(online[0].name, "Owner");
}

#[test]
fn test_collaboration_session_update_cursor() {
    let owner = create_test_owner();
    let mut session = create_test_session(
        "Test".to_string(),
        owner,
        "session-1".to_string(),
    );
    
    let position = CursorPosition {
        file_path: Some("test.rs".to_string()),
        line: 10,
        column: 5,
    };
    
    let owner_id = session.participants[0].id.clone();
    session.update_cursor(&owner_id, position.clone());
    
    assert_eq!(session.participants[0].cursor_position, Some(position));
}

#[test]
fn test_collaboration_session_remove_nonexistent() {
    let owner = create_test_owner();
    let mut session = create_test_session(
        "Test".to_string(),
        owner,
        "session-1".to_string(),
    );
    
    // Try to remove a non-existent participant
    let non_existent = ParticipantId::from_string(uuid::Uuid::new_v4().to_string());
    session.remove_participant(&non_existent);
    
    // Should still have the owner
    assert_eq!(session.participants.len(), 1);
}

#[test]
fn test_update_cursor_nonexistent_participant() {
    let owner = create_test_owner();
    let mut session = create_test_session(
        "Test".to_string(),
        owner,
        "session-1".to_string(),
    );
    
    // Update cursor for non-existent participant - should not panic
    let non_existent = ParticipantId::from_string(uuid::Uuid::new_v4().to_string());
    let position = CursorPosition {
        file_path: Some("test.rs".to_string()),
        line: 10,
        column: 5,
    };
    session.update_cursor(&non_existent, position);
    
    // Owner's cursor should still be None
    assert!(session.participants[0].cursor_position.is_none());
}
