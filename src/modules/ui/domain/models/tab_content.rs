use crate::shared::kernel::types::Tab;
use serde::{Deserialize, Serialize};

/// Tab content with 3-column layout
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TabContent {
    pub id: Tab,
    pub label: String,
    pub left: String,
    pub center: String,
    pub right: String,
}

impl TabContent {
    pub fn new(id: Tab) -> Self {
        Self {
            id,
            label: id.label().to_string(),
            left: String::new(),
            center: String::new(),
            right: String::new(),
        }
    }

    pub fn with_content(id: Tab, left: &str, center: &str, right: &str) -> Self {
        Self {
            id,
            label: id.label().to_string(),
            left: left.to_string(),
            center: center.to_string(),
            right: right.to_string(),
        }
    }
}
