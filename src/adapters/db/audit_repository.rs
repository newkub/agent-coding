use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::audit::application::usecases::AuditQuery;
use crate::modules::audit::domain::models::{AuditEntry, AuditId};
use crate::modules::audit::ports::AuditRepository;
use crate::shared::kernel::result::{AppError, AppResult};

/// SQLite-backed audit repository.
///
/// The full `AuditEntry` is stored as a JSON blob in `data`; `timestamp`,
/// `category`, `actor_id` and `resource_id` are derived columns so the
/// `AuditQuery` filters can run in SQL instead of loading every entry.
/// Timestamps are RFC 3339 strings, which compare correctly as text.
pub(crate) struct SqliteAuditRepository {
    pool: SqlitePool,
}

impl SqliteAuditRepository {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn row_to_entry(row: &sqlx::sqlite::SqliteRow) -> AppResult<AuditEntry> {
        let data: String = row.get("data");
        serde_json::from_str::<AuditEntry>(&data)
            .map_err(|e| AppError::Database(format!("failed to decode audit entry row: {e}")))
    }

    /// WHERE clause fragment for the supported `AuditQuery` filters. The
    /// conditions are appended in a fixed order; [`Self::bind_filters`] binds
    /// values in the same order.
    fn where_clause(filters: &AuditQuery) -> String {
        let mut sql = String::new();
        if filters.start_time.is_some() {
            sql.push_str(" AND timestamp >= ?");
        }
        if filters.end_time.is_some() {
            sql.push_str(" AND timestamp <= ?");
        }
        if filters.actor_id.is_some() {
            sql.push_str(" AND actor_id = ?");
        }
        if filters.category.is_some() {
            sql.push_str(" AND category = ?");
        }
        if filters.resource_id.is_some() {
            sql.push_str(" AND resource_id = ?");
        }
        sql
    }

    /// Bind the filter values to a query built with [`Self::where_clause`].
    fn bind_filters<'q>(
        mut query: sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments>,
        filters: &'q AuditQuery,
    ) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments> {
        if let Some(start) = &filters.start_time {
            query = query.bind(start.to_rfc3339());
        }
        if let Some(end) = &filters.end_time {
            query = query.bind(end.to_rfc3339());
        }
        if let Some(actor_id) = &filters.actor_id {
            query = query.bind(actor_id.as_str());
        }
        if let Some(category) = &filters.category {
            query = query.bind(category.as_str());
        }
        if let Some(resource_id) = &filters.resource_id {
            query = query.bind(resource_id.as_str());
        }
        query
    }

    fn apply_window(mut entries: Vec<AuditEntry>, filters: &AuditQuery) -> Vec<AuditEntry> {
        if let Some(offset) = filters.offset {
            entries = entries.into_iter().skip(offset).collect();
        }
        if let Some(limit) = filters.limit {
            entries.truncate(limit);
        }
        entries
    }
}

#[async_trait]
impl AuditRepository for SqliteAuditRepository {
    async fn save(&self, entry: &AuditEntry) -> AppResult<()> {
        let data = serde_json::to_string(entry)
            .map_err(|e| AppError::Database(format!("failed to encode audit entry: {e}")))?;

        sqlx::query(
            r#"
            INSERT INTO audit_entries
                (id, timestamp, category, actor_id, resource_id, data)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(entry.id.as_str())
        .bind(entry.timestamp.to_rfc3339())
        .bind(entry.action.category())
        .bind(&entry.actor.id)
        .bind(&entry.resource.id)
        .bind(&data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("audit_entries.save", e))?;

        tracing::debug!(
            audit_id = entry.id.as_str(),
            category = entry.action.category(),
            "audit entry persisted"
        );
        Ok(())
    }

    async fn find_by_id(&self, id: &AuditId) -> AppResult<Option<AuditEntry>> {
        let row = sqlx::query("SELECT data FROM audit_entries WHERE id = ?")
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("audit_entries.find_by_id", e))?;

        row.as_ref().map(Self::row_to_entry).transpose()
    }

    async fn query(&self, filters: AuditQuery) -> AppResult<Vec<AuditEntry>> {
        let sql = format!(
            "SELECT data FROM audit_entries WHERE 1 = 1{} ORDER BY rowid",
            Self::where_clause(&filters)
        );
        let rows = Self::bind_filters(sqlx::query(sqlx::AssertSqlSafe(sql)), &filters)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("audit_entries.query", e))?;

        let entries = rows
            .iter()
            .map(Self::row_to_entry)
            .collect::<AppResult<Vec<_>>>()?;
        Ok(Self::apply_window(entries, &filters))
    }

    async fn count(&self, filters: AuditQuery) -> AppResult<usize> {
        let sql = format!(
            "SELECT COUNT(*) AS n FROM audit_entries WHERE 1 = 1{}",
            Self::where_clause(&filters)
        );
        let row = Self::bind_filters(sqlx::query(sqlx::AssertSqlSafe(sql)), &filters)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| db_err("audit_entries.count", e))?;
        let n: i64 = row.get("n");
        usize::try_from(n).map_err(|e| AppError::Database(format!("invalid count: {e}")))
    }

    async fn delete_older_than(
        &self,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<usize> {
        let result = sqlx::query("DELETE FROM audit_entries WHERE timestamp < ?")
            .bind(timestamp.to_rfc3339())
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("audit_entries.delete_older_than", e))?;
        usize::try_from(result.rows_affected())
            .map_err(|e| AppError::Database(format!("invalid rows_affected: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;
    use crate::modules::audit::domain::models::{
        Actor, ActorType, AuditAction, AuditResult, Resource,
    };
    use chrono::Utc;

    async fn repo() -> SqliteAuditRepository {
        SqliteAuditRepository::new(test_pool().await)
    }

    fn sample(id: &str, category: &str) -> AuditEntry {
        let action = match category {
            "command" => AuditAction::CommandExecute {
                command: category.to_string(),
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
        let repo = repo().await;
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
        let repo = repo().await;
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
        let repo = repo().await;
        let mut old = sample("1", "command");
        old.timestamp = Utc::now() - chrono::Duration::days(10);
        let recent = sample("2", "command");

        repo.save(&old).await.unwrap();
        repo.save(&recent).await.unwrap();

        assert_eq!(repo.count(AuditQuery::default()).await.unwrap(), 2);

        let cutoff = Utc::now() - chrono::Duration::days(5);
        let deleted = repo.delete_older_than(cutoff).await.unwrap();
        assert_eq!(deleted, 1);

        let remaining = repo.query(AuditQuery::default()).await.unwrap();
        assert_eq!(remaining.len(), 1);
    }

    #[tokio::test]
    async fn test_query_limit_and_offset() {
        let repo = repo().await;
        for i in 0..5 {
            repo.save(&sample(&format!("e{i}"), "command"))
                .await
                .unwrap();
        }

        let filters = AuditQuery {
            limit: Some(2),
            offset: Some(1),
            ..AuditQuery::default()
        };
        let results = repo.query(filters).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id.as_str(), "e1");
    }
}
