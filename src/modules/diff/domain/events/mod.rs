// Diff domain events

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffEvent {
    DiffGenerated {
        old_content: String,
        new_content: String,
        diff_output: String,
    },
    DiffApplied {
        file_path: String,
    },
    DiffFailed {
        error: String,
    },
    HunkApproved {
        file_path: String,
        hunk_id: String,
    },
    HunkRejected {
        file_path: String,
        hunk_id: String,
    },
}
