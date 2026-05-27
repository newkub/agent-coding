use serde::{Deserialize, Serialize};

/// Command for command palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub shortcut: Option<String>,
    pub description: String,
}

impl Command {
    pub fn new(name: &str, shortcut: Option<&str>, description: &str) -> Self {
        Self {
            name: name.to_string(),
            shortcut: shortcut.map(|s| s.to_string()),
            description: description.to_string(),
        }
    }
}
