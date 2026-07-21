use serde::{Deserialize, Serialize};

/// Macro Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacroEvent {
    RecordingStarted { macro_id: String },
    StepRecorded { macro_id: String, step_index: usize },
    RecordingStopped { macro_id: String },
    MacroExecuted { macro_id: String },
}
