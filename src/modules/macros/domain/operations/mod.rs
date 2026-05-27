use super::models::Macro;

/// Pure domain operation: Validate macro name
pub fn validate_macro_name(name: &str) -> Result<(), MacroValidationError> {
    if name.trim().is_empty() {
        return Err(MacroValidationError::EmptyName);
    }
    if name.len() > 100 {
        return Err(MacroValidationError::NameTooLong);
    }
    if name.contains('$') && name.contains("{{") {
        return Err(MacroValidationError::InvalidVariables);
    }
    Ok(())
}

/// Validation errors
#[derive(Debug, Clone)]
pub enum MacroValidationError {
    EmptyName,
    NameTooLong,
    EmptySteps,
    InvalidVariables,
}

impl std::fmt::Display for MacroValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(f, "Macro name cannot be empty"),
            Self::NameTooLong => write!(f, "Macro name cannot exceed 100 characters"),
            Self::EmptySteps => write!(f, "Macro must have at least one step"),
            Self::InvalidVariables => write!(f, "Macro contains invalid variable syntax"),
        }
    }
}

/// Pure domain operation: Validate macro completeness
pub fn validate_macro_completeness(macro_def: &Macro) -> Result<(), MacroValidationError> {
    if macro_def.steps.is_empty() {
        return Err(MacroValidationError::EmptySteps);
    }
    validate_macro_name(&macro_def.name)?;
    Ok(())
}

/// Pure domain operation: Calculate macro complexity
pub fn calculate_macro_complexity(macro_def: &Macro) -> MacroComplexity {
    MacroComplexity {
        step_count: macro_def.step_count(),
        estimated_duration_ms: macro_def.estimated_duration_ms(),
        has_variables: macro_def.steps.iter().any(|s| matches!(s, super::models::MacroStep::Command { .. })),
    }
}

#[derive(Debug, Clone)]
pub struct MacroComplexity {
    pub step_count: usize,
    pub estimated_duration_ms: u64,
    pub has_variables: bool,
}