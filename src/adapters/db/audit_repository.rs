use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::audit::application::usecases::AuditQuery;
use crate::modules::audit::domain::models::{AuditEntry, AuditId};
use crate::modules::audit::ports::AuditRepository;
use crate::shared::kernel::result::AppResult;

/// In-memory audit repository for fast startup and CLI wiring.
pub(crate) struct InMemoryAuditRepository {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
}

impl InMemoryAuditRepository {
    pub(crate) fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for InMemoryAuditRepository {
    fn default() -> Self {
        Self::new()
    }
}

fn matches_filters(entry: &AuditEntry, filters: &AuditQuery) -> bool {
    if let Some(start) = filters.start_time {
        if entry.timestamp < start {
            return false;
        }
    }

    if let Some(end) = filters.end_time {
        if entry.timestamp > end {
            return false;
        }
    }

    if let Some(ref actor_id) = filters.actor_id {
        if &entry.actor.id != actor_id {
            return false;
        }
    }

    if let Some(ref category) = filters.category {
        if entry.action.category() != category {
            return false;
        }
    }

    if let Some(ref resource_id) = filters.resource_id {
        if &entry.resource.id != resource_id {
            return false;
        }
    }

    true
}

#[async_trait]
impl AuditRepository for InMemoryAuditRepository {
    async fn save(&self, entry: &AuditEntry) -> AppResult<()> {
        let mut entries = self.entries.write().await;
        entries.push(entry.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &AuditId) -> AppResult<Option<AuditEntry>> {
        let entries = self.entries.read().await;
        Ok(entries
            .iter()
            .find(|e| e.id.as_str() == id.as_str())
            .cloned())
    }

    async fn query(&self, filters: AuditQuery) -> AppResult<Vec<AuditEntry>> {
        let entries = self.entries.read().await;
        let mut filtered: Vec<AuditEntry> = entries
            .iter()
            .filter(|e| matches_filters(e, &filters))
            .cloned()
            .collect();

        if let Some(offset) = filters.offset {
            filtered = filtered.into_iter().skip(offset).collect();
        }

        if let Some(limit) = filters.limit {
            filtered.truncate(limit);
        }

        Ok(filtered)
    }

    async fn count(&self, filters: AuditQuery) -> AppResult<usize> {
        let entries = self.entries.read().await;
        Ok(entries
            .iter()
            .filter(|e| matches_filters(e, &filters))
            .count())
    }

    async fn delete_older_than(
        &self,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<usize> {
        let mut entries = self.entries.write().await;
        let original_len = entries.len();
        entries.retain(|e| e.timestamp >= timestamp);
        Ok(original_len - entries.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::audit::domain::models::{
        Actor, ActorType, AuditAction, AuditResult, Resource,
    };
    use chrono::Utc;

    fn sample(id: &str, category: &str) -> AuditEntry {
        let action = match category {
            "command" => AuditAction::CommandExecute {
                command: category.to_string(),
            },
            "system" => AuditAction::ConfigChange {
                key: category.to_string(),
            },
            _ => AuditAction::ConfigChange {
                key: category.to_string(),
            },
        };

        AuditEntry {
            id: AuditId::from_string(id.to_string()),
            timestamp: Utc::now(),
            action,
            actor: Actor {
                type_: ActorType::User,
                id: "cli".to_string(),
                name: "cli".to_string(),
            },
            resource: Resource {
                type_: category.to_string(),
                id: "-".to_string(),
                path: None,
            },
            metadata: Default::default(),
            result: AuditResult::Success,
        }
    }

    #[tokio::test]
    async fn test_save_and_query() {
        let repo = InMemoryAuditRepository::new();
        let entry = sample("1", "command");
        repo.save(&entry).await.unwrap();

        let found = repo
            .find_by_id(&AuditId::from_string("1".to_string()))
            .await
            .unwrap();
        assert!(found.is_some());

        let results = repo.query(AuditQuery::default()).await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_query_by_category() {
        let repo = InMemoryAuditRepository::new();
        repo.save(&sample("1", "command")).await.unwrap();
        repo.save(&sample("2", "system")).await.unwrap();

        let filters = AuditQuery {
            category: Some("command".to_string()),
            ..AuditQuery::default()
        };

        let results = repo.query(filters).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id.as_str(), "1");
    }

    #[tokio::test]
    async fn test_count_and_delete_older_than() {
        let repo = InMemoryAuditRepository::new();
        let mut old = sample("1", "command");
        old.timestamp = Utc::now() - chrono::Duration::days(10);
        let mut recent = sample("2", "command");
        recent.timestamp = Utc::now();

        repo.save(&old).await.unwrap();
        repo.save(&recent).await.unwrap();

        let cutoff = Utc::now() - chrono::Duration::days(5);
        let deleted = repo.delete_older_than(cutoff).await.unwrap();
        assert_eq!(deleted, 1);

        let remaining = repo.query(AuditQuery::default()).await.unwrap();
        assert_eq!(remaining.len(), 1);
    }
}
