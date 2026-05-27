//! Snippet serialization tests

use agent_tui::modules::snippet::domain::models::{Snippet, SnippetLibrary, SnippetId};

#[test]
fn test_snippet_library_serialization() {
    let mut lib = SnippetLibrary::new();
    lib.add(Snippet::create(SnippetId::from_string(uuid::Uuid::new_v4().to_string()), "Test".to_string(), "desc".to_string(), "code".to_string(), "rust".to_string(), chrono::Utc::now(), chrono::Utc::now()));
    
    let json = serde_json::to_string(&lib).unwrap();
    let parsed: SnippetLibrary = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.snippets.len(), 1);
}
