pub mod share_link;

pub use share_link::*;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Exportable session format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedSession {
    pub version: String,
    pub exported_at: DateTime<Utc>,
    pub session: crate::modules::session::domain::models::Session,
    pub metadata: ExportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub exported_by: Option<String>,
    pub application_version: Option<String>,
    pub include_messages: bool,
    pub include_context: bool,
}

impl Default for ExportMetadata {
    fn default() -> Self {
        Self {
            exported_by: None,
            application_version: None,
            include_messages: true,
            include_context: true,
        }
    }
}

impl ExportedSession {
    pub fn new(
        session: crate::modules::session::domain::models::Session,
        metadata: ExportMetadata,
    ) -> Self {
        Self {
            version: "1.0".to_string(),
            exported_at: Utc::now(),
            session,
            metadata,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Import result
#[derive(Debug)]
pub struct ImportResult {
    pub session: crate::modules::session::domain::models::Session,
    pub warnings: Vec<String>,
    pub conflicts_resolved: Vec<String>,
}

impl ImportResult {
    pub const fn new(session: crate::modules::session::domain::models::Session) -> Self {
        Self {
            session,
            warnings: Vec::new(),
            conflicts_resolved: Vec::new(),
        }
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Team knowledge base entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub usage_count: u32,
}

impl KnowledgeEntry {
    pub fn new(title: String, content: String, category: String, author: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content,
            category,
            tags: Vec::new(),
            author,
            created_at: now,
            updated_at: now,
            usage_count: 0,
        }
    }

    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }
}

/// Team knowledge base
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub entries: Vec<KnowledgeEntry>,
}

impl KnowledgeBase {
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn search(&self, query: &str) -> Vec<&KnowledgeEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.title.to_lowercase().contains(&query_lower)
                    || e.content.to_lowercase().contains(&query_lower)
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn by_category(&self, category: &str) -> Vec<&KnowledgeEntry> {
        self.entries
            .iter()
            .filter(|e| e.category.to_lowercase() == category.to_lowercase())
            .collect()
    }
}