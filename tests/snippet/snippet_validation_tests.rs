//! Snippet validation tests

use agent_tui::modules::snippet::domain::operations::{
    validate_snippet_code, validate_snippet_name, SnippetValidationError,
};

#[test]
fn test_validate_snippet_name_valid() {
    assert!(validate_snippet_name("Valid Name").is_ok());
    assert!(validate_snippet_name("123").is_ok());
}

#[test]
fn test_validate_snippet_name_empty() {
    assert!(matches!(
        validate_snippet_name(""),
        Err(SnippetValidationError::EmptyName)
    ));
}

#[test]
fn test_validate_snippet_name_too_long() {
    let long_name = "a".repeat(101);
    assert!(matches!(
        validate_snippet_name(&long_name),
        Err(SnippetValidationError::NameTooLong)
    ));
}

#[test]
fn test_validate_snippet_name_invalid() {
    assert!(matches!(
        validate_snippet_name("test{{invalid}}"),
        Err(SnippetValidationError::InvalidCharacters)
    ));
}

#[test]
fn test_validate_snippet_code_valid() {
    assert!(validate_snippet_code("fn main() {}").is_ok());
}

#[test]
fn test_validate_snippet_code_empty() {
    assert!(matches!(
        validate_snippet_code(""),
        Err(SnippetValidationError::EmptyCode)
    ));
}

#[test]
fn test_validate_snippet_code_whitespace() {
    assert!(validate_snippet_code("   ").is_err()); // whitespace-only is empty after trim
}

#[test]
fn test_validate_snippet_code_too_long() {
    let long_code = "x".repeat(100001);
    assert!(matches!(
        validate_snippet_code(&long_code),
        Err(SnippetValidationError::CodeTooLong)
    ));
}

#[test]
fn test_snippet_validation_error_display() {
    assert_eq!(
        format!("{}", SnippetValidationError::EmptyName),
        "Snippet name cannot be empty"
    );
    assert_eq!(
        format!("{}", SnippetValidationError::NameTooLong),
        "Snippet name cannot exceed 100 characters"
    );
    assert_eq!(
        format!("{}", SnippetValidationError::EmptyCode),
        "Snippet code cannot be empty"
    );
    assert_eq!(
        format!("{}", SnippetValidationError::CodeTooLong),
        "Snippet code cannot exceed 100KB"
    );
}
