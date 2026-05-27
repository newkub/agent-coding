//! Macro Context tests

use agent_tui::modules::macros::domain::models::MacroContext;

#[test]
fn test_macro_context_new() {
    let ctx = MacroContext::new();
    assert!(ctx.variables.is_empty());
}

#[test]
fn test_macro_context_set() {
    let mut ctx = MacroContext::new();
    ctx.set("name".to_string(), "value".to_string());
    assert_eq!(ctx.get("name"), Some(&"value".to_string()));
}

#[test]
fn test_macro_context_get_nonexistent() {
    let ctx = MacroContext::new();
    assert!(ctx.get("nonexistent").is_none());
}

#[test]
fn test_macro_context_resolve() {
    let mut ctx = MacroContext::new();
    ctx.set("name".to_string(), "World".to_string());
    
    let result = ctx.resolve("Hello ${name}!");
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_macro_context_resolve_multiple() {
    let mut ctx = MacroContext::new();
    ctx.set("first".to_string(), "Hello".to_string());
    ctx.set("second".to_string(), "World".to_string());
    
    let result = ctx.resolve("${first} ${second}!");
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_macro_context_resolve_no_vars() {
    let ctx = MacroContext::new();
    let result = ctx.resolve("No variables");
    assert_eq!(result, "No variables");
}

#[test]
fn test_macro_context_resolve_missing_var() {
    let ctx = MacroContext::new();
    let result = ctx.resolve("${missing}");
    assert_eq!(result, "${missing}"); // Unchanged
}
