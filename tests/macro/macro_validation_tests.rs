//! Macro validation tests

use agent_tui::modules::macros::domain::models::{Macro, MacroId, MacroStep};
use agent_tui::modules::macros::domain::operations::{
    calculate_macro_complexity, validate_macro_completeness, validate_macro_name,
    MacroValidationError,
};

#[test]
fn test_validate_macro_name_valid() {
    assert!(validate_macro_name("Valid Macro").is_ok());
    assert!(validate_macro_name("macro123").is_ok());
}

#[test]
fn test_validate_macro_name_empty() {
    assert!(matches!(
        validate_macro_name(""),
        Err(MacroValidationError::EmptyName)
    ));
}

#[test]
fn test_validate_macro_name_too_long() {
    let long_name = "a".repeat(101);
    assert!(matches!(
        validate_macro_name(&long_name),
        Err(MacroValidationError::NameTooLong)
    ));
}

#[test]
fn test_validate_macro_name_invalid_vars() {
    assert!(matches!(
        validate_macro_name("test${{invalid}}"),
        Err(MacroValidationError::InvalidVariables)
    ));
}

#[test]
fn test_validate_macro_completeness_valid() {
    let mut macro_def = Macro::create(
        MacroId::from_string(uuid::Uuid::new_v4().to_string()),
        "Test".to_string(),
        "".to_string(),
        chrono::Utc::now(),
    );
    macro_def.add_step(MacroStep::Input {
        text: "test".to_string(),
    });

    assert!(validate_macro_completeness(&macro_def).is_ok());
}

#[test]
fn test_validate_macro_completeness_empty_steps() {
    let macro_def = Macro::create(
        MacroId::from_string(uuid::Uuid::new_v4().to_string()),
        "Test".to_string(),
        "".to_string(),
        chrono::Utc::now(),
    );
    assert!(matches!(
        validate_macro_completeness(&macro_def),
        Err(MacroValidationError::EmptySteps)
    ));
}

#[test]
fn test_calculate_macro_complexity() {
    let mut macro_def = Macro::create(
        MacroId::from_string(uuid::Uuid::new_v4().to_string()),
        "Test".to_string(),
        "".to_string(),
        chrono::Utc::now(),
    );
    macro_def.add_step(MacroStep::Input {
        text: "test".to_string(),
    });
    macro_def.add_step(MacroStep::Command {
        cmd: "ls".to_string(),
        cwd: None,
    });

    let complexity = calculate_macro_complexity(&macro_def);
    assert_eq!(complexity.step_count, 2);
    assert!(complexity.has_variables);
}

#[test]
fn test_macro_validation_error_display() {
    assert_eq!(
        format!("{}", MacroValidationError::EmptyName),
        "Macro name cannot be empty"
    );
    assert_eq!(
        format!("{}", MacroValidationError::NameTooLong),
        "Macro name cannot exceed 100 characters"
    );
    assert_eq!(
        format!("{}", MacroValidationError::EmptySteps),
        "Macro must have at least one step"
    );
}
