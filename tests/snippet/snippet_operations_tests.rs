//! Snippet operations tests

use agent_tui::modules::snippet::domain::models::Snippet;
use agent_tui::modules::snippet::domain::operations::calculate_snippet_stats;

#[test]
fn test_calculate_snippet_stats() {
    let now = chrono::Utc::now();
    let snippet = Snippet::create(
        agent_tui::modules::snippet::domain::models::SnippetId::from_string(
            uuid::Uuid::new_v4().to_string(),
        ),
        "Test".to_string(),
        "".to_string(),
        "line1\nline2\nline3".to_string(),
        "rust".to_string(),
        now,
        now,
    );
    let stats = calculate_snippet_stats(&snippet);
    assert_eq!(stats.line_count, 3);
    assert!(stats.char_count > 0);
}
