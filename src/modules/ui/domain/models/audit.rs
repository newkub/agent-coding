use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditLog {
    pub entries: VecDeque<AuditEntry>,
    pub max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub operation: String,
    pub details: String,
    pub user: String,
    pub success: bool,
}

impl AuditLog {
    pub const fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries: 1000,
        }
    }

    pub fn log_operation(
        &mut self,
        operation: String,
        details: String,
        user: String,
        success: bool,
    ) {
        let entry = AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            operation,
            details,
            user,
            success,
        };

        self.entries.push_back(entry);

        // Keep only max_entries
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    pub fn get_entries(&self) -> Vec<&AuditEntry> {
        self.entries.iter().collect()
    }

    pub fn filter_by_operation(&self, operation: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.operation == operation)
            .collect()
    }

    pub fn filter_by_user(&self, user: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.user == user).collect()
    }

    pub fn filter_by_date_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    pub fn export(&self) -> String {
        self.entries
            .iter()
            .map(|e| {
                format!(
                    "[{}] {} - {} - {} - {}",
                    e.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    e.operation,
                    e.details,
                    e.user,
                    if e.success { "✓" } else { "✗" }
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
