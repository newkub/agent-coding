//! Macro serialization tests

use agent_tui::modules::macros::domain::models::{Macro, MacroContext, MacroId, MacroStep};

#[test]
fn test_macro_serialization() {
    let mut macro_def = Macro::create(
        MacroId::from_string(uuid::Uuid::new_v4().to_string()),
        "Test".to_string(),
        "desc".to_string(),
        chrono::Utc::now(),
    );
    macro_def.add_step(MacroStep::Input {
        text: "hello".to_string(),
    });

    let json = serde_json::to_string(&macro_def).unwrap();
    let parsed: Macro = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "Test");
    assert_eq!(parsed.step_count(), 1);
}

#[test]
fn test_macro_context_serialization() {
    let mut ctx = MacroContext::new();
    ctx.set("key".to_string(), "value".to_string());

    let json = serde_json::to_string(&ctx).unwrap();
    let parsed: MacroContext = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.get("key"), Some(&"value".to_string()));
}
