use crate::modules::snippet::domain::models::Snippet;
use crate::modules::snippet::domain::operations::{validate_snippet_code, validate_snippet_name};
use crate::shared::kernel::result::AppResult;

/// Service: Create snippet with validation
pub(crate) fn create_snippet(
    name: String,
    description: String,
    code: String,
    language: String,
) -> AppResult<Snippet> {
    validate_snippet_name(&name)
        .map_err(|e| crate::shared::kernel::result::AppError::State(e.to_string()))?;
    validate_snippet_code(&code)
        .map_err(|e| crate::shared::kernel::result::AppError::State(e.to_string()))?;

    // Create snippet with generated ID and timestamps
    let id = crate::modules::snippet::domain::models::SnippetId::from_string(
        uuid::Uuid::new_v4().to_string(),
    );
    let now = chrono::Utc::now();
    Ok(Snippet::create(
        id,
        name,
        description,
        code,
        language,
        now,
        now,
    ))
}

/// Service: Find variable values for prompt
pub(crate) fn prepare_variable_prompts(snippet: &Snippet) -> Vec<(String, Option<String>, String)> {
    snippet
        .variables
        .iter()
        .map(|v| {
            let prompt = format!(
                "Enter value for '{}':{}",
                v.name,
                v.description
                    .as_deref()
                    .map(|d| format!(" ({})", d))
                    .unwrap_or_default()
            );
            (v.name.clone(), v.default_value.clone(), prompt)
        })
        .collect()
}
