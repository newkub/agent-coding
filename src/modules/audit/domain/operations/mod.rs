use super::models::{AuditAction, AuditEntry};

/// Pure domain operation: Filter audit entries by action type
pub fn filter_by_action<'a>(entries: &'a [AuditEntry], action_type: &str) -> Vec<&'a AuditEntry> {
    entries
        .iter()
        .filter(|e| e.action.category() == action_type)
        .collect()
}

/// Pure domain operation: Filter audit entries by actor
pub fn filter_by_actor<'a>(entries: &'a [AuditEntry], actor_id: &str) -> Vec<&'a AuditEntry> {
    entries.iter().filter(|e| e.actor.id == actor_id).collect()
}

/// Pure domain operation: Filter audit entries by time range
pub fn filter_by_time_range(
    entries: &[AuditEntry],
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Vec<&AuditEntry> {
    entries
        .iter()
        .filter(|e| e.timestamp >= start && e.timestamp <= end)
        .collect()
}

/// Pure domain operation: Check if action is allowed
pub fn is_action_allowed(
    action: &AuditAction,
    actor: &crate::modules::audit::domain::models::Actor,
    rules: &[crate::modules::audit::domain::operations::AuditRule],
) -> bool {
    for rule in rules {
        if rule.matches(action, actor) {
            return rule.allow;
        }
    }
    true // Default allow
}

#[derive(Debug, Clone)]
pub struct AuditRule {
    pub action_pattern: String,
    pub actor_id_pattern: Option<String>,
    pub allow: bool,
    pub description: String,
}

impl AuditRule {
    pub fn new(pattern: &str, allow: bool, description: &str) -> Self {
        Self {
            action_pattern: pattern.to_string(),
            actor_id_pattern: None,
            allow,
            description: description.to_string(),
        }
    }

    pub fn with_actor(mut self, actor_pattern: &str) -> Self {
        self.actor_id_pattern = Some(actor_pattern.to_string());
        self
    }

    pub fn matches(
        &self,
        action: &AuditAction,
        actor: &crate::modules::audit::domain::models::Actor,
    ) -> bool {
        let action_str = format!("{:?}", action);

        // Check action pattern
        let action_matches = if let Ok(re) = regex::Regex::new(&self.action_pattern) {
            re.is_match(&action_str)
        } else {
            action_str.contains(&self.action_pattern)
        };

        if !action_matches {
            return false;
        }

        // Check actor pattern if specified
        if let Some(ref actor_pattern) = self.actor_id_pattern {
            if let Ok(re) = regex::Regex::new(actor_pattern) {
                return re.is_match(&actor.id);
            }
            return actor.id.contains(actor_pattern);
        }

        true
    }
}

/// Pure domain operation: Generate audit summary
pub fn generate_audit_summary(entries: &[AuditEntry]) -> AuditSummary {
    let mut summary = AuditSummary::default();

    for entry in entries {
        match &entry.result {
            crate::modules::audit::domain::models::AuditResult::Success => summary.success += 1,
            crate::modules::audit::domain::models::AuditResult::Failure { .. } => {
                summary.failure += 1
            }
            crate::modules::audit::domain::models::AuditResult::Pending => summary.pending += 1,
        }

        summary
            .by_category
            .entry(entry.action.category().to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    summary.total = entries.len();
    summary
}

#[derive(Debug, Default)]
pub struct AuditSummary {
    pub total: usize,
    pub success: usize,
    pub failure: usize,
    pub pending: usize,
    pub by_category: std::collections::HashMap<String, usize>,
}
