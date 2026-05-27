//! Macro model tests

use agent_tui::modules::macros::domain::models::{Macro, MacroStep, MacroId};

#[test]
fn test_macro_new() {
    let macro_def = Macro::create(MacroId::from_string(uuid::Uuid::new_v4().to_string()), "Test Macro".to_string(), "A test macro".to_string(), chrono::Utc::now());
    assert_eq!(macro_def.name, "Test Macro");
    assert_eq!(macro_def.description, "A test macro");
    assert!(macro_def.steps.is_empty());
    assert!(!macro_def.id.to_string().is_empty());
}

#[test]
fn test_macro_add_step() {
    let mut macro_def = Macro::create(MacroId::from_string(uuid::Uuid::new_v4().to_string()), "Test".to_string(), "".to_string(), chrono::Utc::now());
    macro_def.add_step(MacroStep::Input { text: "hello".to_string() });
    assert_eq!(macro_def.step_count(), 1);
}

#[test]
fn test_macro_increment_usage() {
    let mut macro_def = Macro::create(MacroId::from_string(uuid::Uuid::new_v4().to_string()), "Test".to_string(), "".to_string(), chrono::Utc::now());
    assert_eq!(macro_def.usage_count, 0);
    macro_def.increment_usage();
    assert_eq!(macro_def.usage_count, 1);
}

#[test]
fn test_macro_estimated_duration_ms_input() {
    let step = MacroStep::Input { text: "hello".to_string() };
    assert_eq!(step.estimated_duration_ms(), 250); // 5 chars * 50ms
}

#[test]
fn test_macro_estimated_duration_ms_wait() {
    let step = MacroStep::Wait { millis: 1000 };
    assert_eq!(step.estimated_duration_ms(), 1000);
}

#[test]
fn test_macro_estimated_duration_ms_key_combo() {
    let step = MacroStep::KeyCombo { keys: vec!["ctrl".to_string(), "c".to_string()] };
    assert_eq!(step.estimated_duration_ms(), 100);
}

#[test]
fn test_macro_estimated_duration_ms_command() {
    let step = MacroStep::Command { cmd: "ls".to_string(), cwd: None };
    assert_eq!(step.estimated_duration_ms(), 1000);
}

#[test]
fn test_macro_estimated_duration_ms_total() {
    let mut macro_def = Macro::create(MacroId::from_string(uuid::Uuid::new_v4().to_string()), "Test".to_string(), "".to_string(), chrono::Utc::now());
    macro_def.add_step(MacroStep::Input { text: "hello".to_string() }); // 5 * 50 = 250
    macro_def.add_step(MacroStep::Wait { millis: 500 }); // 500
    macro_def.add_step(MacroStep::KeyCombo { keys: vec![] }); // 100
    
    assert_eq!(macro_def.estimated_duration_ms(), 850);
}

#[test]
fn test_macro_step_clone() {
    let step1 = MacroStep::Input { text: "test".to_string() };
    let step2 = step1.clone();
    match (step1, step2) {
        (MacroStep::Input { text: t1 }, MacroStep::Input { text: t2 }) => assert_eq!(t1, t2),
        _ => panic!("Clone mismatch"),
    }
}

#[test]
fn test_macro_clone() {
    let mut m1 = Macro::create(MacroId::from_string(uuid::Uuid::new_v4().to_string()), "Test".to_string(), "desc".to_string(), chrono::Utc::now());
    m1.add_step(MacroStep::Input { text: "test".to_string() });
    
    let m2 = m1.clone();
    assert_eq!(m1.id, m2.id);
    assert_eq!(m1.name, m2.name);
    assert_eq!(m1.step_count(), m2.step_count());
}
