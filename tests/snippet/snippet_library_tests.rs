//! Snippet Library tests

use agent_tui::modules::snippet::domain::models::{Snippet, SnippetLibrary, SnippetId};

fn create_test_snippet(name: String, description: String, code: String, language: String) -> Snippet {
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
fn test_snippet_library_new() {
    let lib = SnippetLibrary::new();
    assert!(lib.snippets.is_empty());
}

#[test]
fn test_snippet_library_add() {
    let mut lib = SnippetLibrary::new();
    let snippet = create_test_snippet(
        "Test".to_string(),
        "".to_string(),
        "code".to_string(),
        "rust".to_string(),
    );
    lib.add(snippet);
    assert_eq!(lib.snippets.len(), 1);
}

#[test]
fn test_snippet_library_search() {
    let mut lib = SnippetLibrary::new();
    lib.add(create_test_snippet(
        "Hello World".to_string(),
        "A greeting".to_string(),
        "echo hello".to_string(),
        "bash".to_string(),
    ));
    lib.add(create_test_snippet(
        "Goodbye".to_string(),
        "A farewell".to_string(),
        "echo bye".to_string(),
        "bash".to_string(),
    ));
    
    let results = lib.search("hello");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Hello World");
}

#[test]
fn test_snippet_library_search_case_insensitive() {
    let mut lib = SnippetLibrary::new();
    lib.add(create_test_snippet(
        "Test".to_string(),
        "desc".to_string(),
        "code".to_string(),
        "rust".to_string(),
    ));
    
    let results = lib.search("TEST");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_snippet_library_search_no_results() {
    let lib = SnippetLibrary::new();
    let results = lib.search("nonexistent");
    assert!(results.is_empty());
}

#[test]
fn test_snippet_library_by_language() {
    let mut lib = SnippetLibrary::new();
    lib.add(create_test_snippet("R1".to_string(), "".to_string(), "code".to_string(), "rust".to_string()));
    lib.add(create_test_snippet("R2".to_string(), "".to_string(), "code".to_string(), "rust".to_string()));
    lib.add(create_test_snippet("PY1".to_string(), "".to_string(), "code".to_string(), "python".to_string()));
    
    let rust_snippets = lib.by_language("rust");
    assert_eq!(rust_snippets.len(), 2);
    
    let py_snippets = lib.by_language("python");
    assert_eq!(py_snippets.len(), 1);
}

#[test]
fn test_snippet_library_by_tag() {
    let mut lib = SnippetLibrary::new();
    let mut s1 = create_test_snippet("S1".to_string(), "".to_string(), "code".to_string(), "rust".to_string());
    s1.tags.push("testing".to_string());
    
    let mut s2 = create_test_snippet("S2".to_string(), "".to_string(), "code".to_string(), "rust".to_string());
    s2.tags.push("production".to_string());
    
    lib.add(s1);
    lib.add(s2);
    
    let testing = lib.by_tag("testing");
    assert_eq!(testing.len(), 1);
}
