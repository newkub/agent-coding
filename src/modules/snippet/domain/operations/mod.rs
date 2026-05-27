use super::models::Snippet;

/// Pure domain operation: Validate snippet name
pub fn validate_snippet_name(name: &str) -> Result<(), SnippetValidationError> {
    if name.trim().is_empty() {
        return Err(SnippetValidationError::EmptyName);
    }
    if name.len() > 100 {
        return Err(SnippetValidationError::NameTooLong);
    }
    if name.contains("{{") || name.contains("}}") {
        return Err(SnippetValidationError::InvalidCharacters);
    }
    Ok(())
}

/// Pure domain operation: Validate snippet code
pub fn validate_snippet_code(code: &str) -> Result<(), SnippetValidationError> {
    if code.trim().is_empty() {
        return Err(SnippetValidationError::EmptyCode);
    }
    if code.len() > 100_000 {
        return Err(SnippetValidationError::CodeTooLong)
    }
    Ok(())
}

/// Validation errors
#[derive(Debug, Clone)]
pub enum SnippetValidationError {
    EmptyName,
    NameTooLong,
    EmptyCode,
    CodeTooLong,
    InvalidCharacters,
}

impl std::fmt::Display for SnippetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(f, "Snippet name cannot be empty"),
            Self::NameTooLong => write!(f, "Snippet name cannot exceed 100 characters"),
            Self::EmptyCode => write!(f, "Snippet code cannot be empty"),
            Self::CodeTooLong => write!(f, "Snippet code cannot exceed 100KB"),
            Self::InvalidCharacters => write!(f, "Snippet name contains invalid characters"),
        }
    }
}

/// Pure domain operation: Calculate snippet statistics
pub fn calculate_snippet_stats(snippet: &Snippet) -> SnippetStats {
    SnippetStats {
        line_count: snippet.code.lines().count(),
        char_count: snippet.code.len(),
        variable_count: snippet.variables.len(),
        tag_count: snippet.tags.len(),
    }
}

#[derive(Debug, Clone)]
pub struct SnippetStats {
    pub line_count: usize,
    pub char_count: usize,
    pub variable_count: usize,
    pub tag_count: usize,
}