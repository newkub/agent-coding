//! Snippet Variable tests

use agent_tui::modules::snippet::domain::models::SnippetVariable;

#[test]
fn test_snippet_variable_default() {
    let var = SnippetVariable {
        name: "test".to_string(),
        description: Some("A test variable".to_string()),
        default_value: None,
    };
    assert_eq!(var.name, "test");
    assert!(var.default_value.is_none());
}
