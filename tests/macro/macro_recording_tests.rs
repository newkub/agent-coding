//! Recording State tests

use agent_tui::modules::macros::domain::models::{RecordingState, MacroId};

#[test]
fn test_recording_state_idle() {
    let state = RecordingState::Idle;
    assert!(!state.is_recording());
    assert!(!state.is_paused());
    assert!(state.macro_id().is_none());
}

#[test]
fn test_recording_state_recording() {
    let id = MacroId::from_string(uuid::Uuid::new_v4().to_string());
    let state = RecordingState::Recording(id.clone());
    assert!(state.is_recording());
    assert!(!state.is_paused());
    assert_eq!(state.macro_id(), Some(&id));
}

#[test]
fn test_recording_state_paused() {
    let id = MacroId::from_string(uuid::Uuid::new_v4().to_string());
    let state = RecordingState::Paused(id.clone());
    assert!(!state.is_recording());
    assert!(state.is_paused());
    assert_eq!(state.macro_id(), Some(&id));
}

#[test]
fn test_recording_state_default() {
    let state = RecordingState::default();
    assert!(matches!(state, RecordingState::Idle));
}
