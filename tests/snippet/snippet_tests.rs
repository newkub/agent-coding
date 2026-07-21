//! Snippet model tests

use agent_tui::modules::snippet::domain::models::{Snippet, SnippetId};

fn create_test_snippet(
    name: String,
    description: String,
    code: String,
    language: String,
) -> Snippet {
    let now = chrono::Utc::now();
    Snippet::create(
        SnippetId::from_string(uuid::Uuid::new_v4().to_string()),
        name,
        description,
        code,
        language,
        now,
        now,
    )
}

#[test]
fn test_snippet_new() {
    let snippet = create_test_snippet(
        "Test Snippet".to_string(),
        "A test snippet".to_string(),
        "fn main() {}".to_string(),
        "rust".to_string(),
    );
    assert_eq!(snippet.name, "Test Snippet");
    assert_eq!(snippet.language, "rust");
    assert!(!snippet.id.as_str().is_empty());
}

#[test]
fn test_snippet_with_tags() {
    let snippet = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "code".to_string(),
        "rust".to_string(),
    )
    .with_tags(vec!["tag1".to_string(), "tag2".to_string()]);
    assert_eq!(snippet.tags.len(), 2);
}

#[test]
fn test_snippet_add_variable() {
    let mut snippet = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "code".to_string(),
        "rust".to_string(),
    );
    snippet.add_variable("name".to_string(), Some("default".to_string()));
    assert_eq!(snippet.variables.len(), 1);
    assert_eq!(snippet.variables[0].name, "name");
    assert_eq!(
        snippet.variables[0].default_value.as_deref(),
        Some("default")
    );
}

#[test]
fn test_snippet_render() {
    let mut snippet = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "Hello {{name}}!".to_string(),
        "text".to_string(),
    );
    snippet.add_variable("name".to_string(), None);

    let result = snippet.render(&[("name".to_string(), "World".to_string())]);
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_snippet_render_multiple_variables() {
    let mut snippet = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "{{greeting}} {{name}}!".to_string(),
        "text".to_string(),
    );
    snippet.add_variable("greeting".to_string(), None);
    snippet.add_variable("name".to_string(), None);

    let result = snippet.render(&[
        ("greeting".to_string(), "Hi".to_string()),
        ("name".to_string(), "Rust".to_string()),
    ]);
    assert_eq!(result, "Hi Rust!");
}

#[test]
fn test_snippet_extract_variables() {
    let snippet = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "{{var1}} and {{var2}}".to_string(),
        "text".to_string(),
    );
    let vars = snippet.extract_variables();
    assert_eq!(vars.len(), 2);
    assert!(vars.contains(&"var1".to_string()));
    assert!(vars.contains(&"var2".to_string()));
}

#[test]
fn test_snippet_extract_variables_no_vars() {
    let snippet = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "No variables here".to_string(),
        "text".to_string(),
    );
    let vars = snippet.extract_variables();
    assert!(vars.is_empty());
}

#[test]
fn test_snippet_clone() {
    let s1 = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "code".to_string(),
        "rust".to_string(),
    );
    let s2 = s1.clone();
    assert_eq!(s1.id, s2.id);
    assert_eq!(s1.name, s2.name);
}
