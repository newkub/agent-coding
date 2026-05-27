use serde::{Deserialize, Serialize};

/// Diff Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum DiffEvent {
    FileDiffGenerated { file_path: String },
    HunkNavigated { file_index: usize, hunk_index: usize },
    HunkApproved { file_path: String, hunk_id: String },
    HunkRejected { file_path: String, hunk_id: String },
    DiffAccepted { file_path: String },
    DiffRejected { file_path: String },
}