use crate::modules::audit::ports::AuditRepository;
use crate::modules::audit::domain::models::{AuditEntry, AuditAction, Actor, Resource, AuditId};
use crate::shared::kernel::result::AppResult;
use chrono::Utc;

/// Use case: Log an audit entry
pub(crate) async fn log_entry<R>(
    repo: &R,
    action: AuditAction,
    actor: Actor,
    resource: Resource,
) -> AppResult<AuditEntry>
where
    R: AuditRepository,
{
    // Side effects (ID generation, timestamp) in application layer
    let id = AuditId::from_string(uuid::Uuid::new_v4().to_string());
    let timestamp = Utc::now();
    
    let entry = AuditEntry::create(id, timestamp, action, actor, resource);
    repo.save(&entry).await?;
    Ok(entry)
}

/// Use case: Query audit logs
pub(crate) async fn query_logs<R>(
    repo: &R,
    filters: AuditQuery,
) -> AppResult<Vec<AuditEntry>>
where
    R: AuditRepository,
{
    repo.query(filters).await
}

/// Use case: Get audit statistics
pub(crate) async fn get_statistics<R>(
    repo: &R,
) -> AppResult<crate::modules::audit::domain::operations::AuditSummary>
where
    R: AuditRepository,
{
    let entries = repo.query(AuditQuery::default()).await?;
    Ok(crate::modules::audit::domain::operations::generate_audit_summary(&entries))
}

#[derive(Debug, Clone, Default)]
pub(crate) struct AuditQuery {
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub actor_id: Option<String>,
    pub category: Option<String>,
    pub resource_id: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}