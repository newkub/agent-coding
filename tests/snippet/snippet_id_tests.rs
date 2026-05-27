//! Snippet ID tests

use agent_tui::modules::snippet::domain::models::SnippetId;

#[test]
fn test_snippet_id_new() {
    let id = SnippetId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.as_str().is_empty());
}

#[test]
fn test_snippet_id_from_string() {
    let id = SnippetId::from_string("custom-id".to_string());
    assert_eq!(id.as_str(), "custom-id");
}

#[test]
fn test_snippet_id_display() {
    let id = SnippetId::from_string("display-id".to_string());
    assert_eq!(format!("{}", id), "display-id");
}

#[test]
fn test_snippet_id_default() {
    let id = SnippetId::from_string("default".to_string());
    assert!(!id.as_str().is_empty());
}

#[test]
fn test_snippet_id_eq() {
    let id1 = SnippetId::from_string("same".to_string());
    let id2 = SnippetId::from_string("same".to_string());
    let id3 = SnippetId::from_string("different".to_string());
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_snippet_id_clone() {
    let id1 = SnippetId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}
