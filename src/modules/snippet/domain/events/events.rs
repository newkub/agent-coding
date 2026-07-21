use serde::{Deserialize, Serialize};

/// Snippet Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnippetEvent {
    Created { snippet_id: String },
    Updated { snippet_id: String },
    Deleted { snippet_id: String },
    Executed { snippet_id: String },
}
