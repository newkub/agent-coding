//! Collaboration ID tests

use agent_tui::modules::collaboration::domain::models::{CollaborationId, ParticipantId};

#[test]
fn test_participant_id_new() {
    let id = ParticipantId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.0.is_empty());
}

#[test]
fn test_participant_id_default() {
    let id = ParticipantId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.0.is_empty());
}

#[test]
fn test_collaboration_id_new() {
    let id = CollaborationId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.to_string().is_empty());
}

#[test]
fn test_collaboration_id_display() {
    let id = CollaborationId::from_string(uuid::Uuid::new_v4().to_string());
    let _ = format!("{}", id);
}

#[test]
fn test_collaboration_id_clone() {
    let id1 = CollaborationId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}

#[test]
fn test_participant_id_clone() {
    let id1 = ParticipantId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}
