//! Collaboration Participant tests

use agent_tui::modules::collaboration::domain::models::{Participant, ParticipantId, ParticipantRole};
use chrono::Utc;

#[test]
fn test_participant_role_variants() {
    assert!(matches!(ParticipantRole::Owner, ParticipantRole::Owner));
    assert!(matches!(ParticipantRole::Editor, ParticipantRole::Editor));
    assert!(matches!(ParticipantRole::Viewer, ParticipantRole::Viewer));
}

#[test]
fn test_participant_serialization() {
    let participant = Participant {
        id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
        name: "Test User".to_string(),
        role: ParticipantRole::Editor,
        joined_at: Utc::now(),
        is_online: true,
        cursor_position: None,
    };
    
    let json = serde_json::to_string(&participant).unwrap();
    let parsed: Participant = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "Test User");
}
