use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A recorded macro consisting of a sequence of actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub id: MacroId,
    pub name: String,
    pub description: String,
    pub steps: Vec<MacroStep>,
    pub created_at: DateTime<Utc>,
    pub usage_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MacroId(pub String);

impl MacroId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MacroId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A single step/action in a macro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacroStep {
    /// Type text input
    Input { text: String },
    /// Wait for duration
    Wait { millis: u64 },
    /// Press key combination
    KeyCombo { keys: Vec<String> },
    /// Execute command
    Command { cmd: String, cwd: Option<String> },
}

impl Macro {
    // Pure constructor - timestamp and ID moved to application layer
    pub const fn create(
        id: MacroId,
        name: String,
        description: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            steps: Vec::new(),
            created_at,
            usage_count: 0,
        }
    }

    pub fn add_step(&mut self, step: MacroStep) {
        self.steps.push(step);
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }

    /// Estimate total duration of macro
    pub fn estimated_duration_ms(&self) -> u64 {
        self.steps.iter().map(|s| s.estimated_duration_ms()).sum()
    }
}

impl MacroStep {
    pub fn estimated_duration_ms(&self) -> u64 {
        match self {
            Self::Input { text } => text.len() as u64 * 50, // ~50ms per char
            Self::Wait { millis } => *millis,
            Self::KeyCombo { .. } => 100,
            Self::Command { .. } => 1000, // Command execution, assume 1s minimum
        }
    }
}

/// Macro recording state
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Default)]
pub enum RecordingState {
    #[default]
    Idle,
    Recording(MacroId),
    Paused(MacroId),
}

impl RecordingState {
    pub const fn is_recording(&self) -> bool {
        matches!(self, Self::Recording(_))
    }

    pub const fn is_paused(&self) -> bool {
        matches!(self, Self::Paused(_))
    }

    pub const fn macro_id(&self) -> Option<&MacroId> {
        match self {
            Self::Recording(id) => Some(id),
            Self::Paused(id) => Some(id),
            Self::Idle => None,
        }
    }
}


/// Macro execution context
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MacroContext {
    pub variables: std::collections::HashMap<String, String>,
}

impl MacroContext {
    pub fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    /// Resolve variables in a string
    pub fn resolve(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (name, value) in &self.variables {
            let placeholder = format!("${{{}}}", name);
            result = result.replace(&placeholder, value);
        }
        result
    }
}