//! Macro ID tests

use agent_tui::modules::macros::domain::models::MacroId;

#[test]
fn test_macro_id_new() {
    let id = MacroId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.to_string().is_empty());
}

#[test]
fn test_macro_id_default() {
    let id = MacroId::from_string("default".to_string());
    assert!(!id.to_string().is_empty());
}

#[test]
fn test_macro_id_display() {
    let id = MacroId::from_string(uuid::Uuid::new_v4().to_string());
    let _ = format!("{}", id); // Should not panic
}

#[test]
fn test_macro_id_eq() {
    let id1 = MacroId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}
