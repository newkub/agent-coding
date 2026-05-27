use serde::{Deserialize, Serialize};

/// Diff review state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffReviewState {
    pub is_active: bool,
    pub diff_text: String,
    pub current_file_index: usize,
    pub current_hunk_index: usize,
}

/// Snippet manager state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnippetState {
    pub is_active: bool,
    pub selected_snippet_id: Option<String>,
    pub search_query: String,
}

/// Macro recording state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MacroState {
    pub is_recording: bool,
    pub current_macro_id: Option<String>,
    pub steps: Vec<crate::modules::macros::domain::models::MacroStep>,
}

/// Sandbox preview state (deprecated - not used in new tabs)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SandboxState {
    pub is_preview_active: bool,
    pub command_preview: Option<crate::modules::sandbox::application::usecases::CommandPreview>,
}

/// Metrics dashboard state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricsState {
    pub is_active: bool,
    pub summary: Option<crate::modules::metrics::domain::models::MetricsSummary>,
}

/// Collaboration state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationState {
    pub is_active: bool,
    pub session_id: Option<String>,
    pub participants: Vec<crate::modules::collaboration::domain::models::Participant>,
}

/// Timeline state for activity view
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimelineState {
    pub entries: Vec<crate::modules::metrics::domain::models::TimelineEntry>,
}
