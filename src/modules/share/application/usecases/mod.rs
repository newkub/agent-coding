pub mod create_share_link;
pub mod access_share_link;
pub mod deactivate_share_link;

use crate::modules::share::ports::ShareRepository;
use crate::modules::share::domain::models::{ExportedSession, ImportResult, ExportMetadata};
use crate::shared::kernel::result::AppResult;

/// Use case: Export session to JSON
pub(crate) fn export_session(
    session: crate::modules::session::domain::models::Session,
    metadata: ExportMetadata,
) -> AppResult<ExportedSession> {
    Ok(ExportedSession::new(session, metadata))
}

/// Use case: Import session from JSON
pub(crate) async fn import_session<R>(
    repo: &R,
    json: &str,
) -> AppResult<ImportResult>
where
    R: ShareRepository,
{
    let exported: ExportedSession = serde_json::from_str(json)
        .map_err(|e| crate::shared::kernel::result::AppError::State(format!("Invalid JSON: {}", e)))?;
    
    // Check version compatibility
    if exported.version != "1.0" {
        return Err(crate::shared::kernel::result::AppError::State(
            format!("Unsupported export version: {}", exported.version)
        ));
    }
    
    // Check for existing session with same name
    let existing = repo.find_by_name(&exported.session.name).await?;
    
    let mut result = ImportResult::new(exported.session.clone());
    
    let mut session = exported.session;
    
    if let Some(existing) = existing {
        result = result.with_warning(format!(
            "Session '{}' already exists, creating copy",
            existing.name
        ));
        session.name = format!("{} (imported)", session.name);
    }
    
    // Save imported session
    repo.save(&session).await?;
    
    Ok(result)
}

/// Use case: Validate exported session format
pub(crate) fn validate_export(json: &str) -> Result<ExportedSession, String> {
    ExportedSession::from_json(json).map_err(|e| e.to_string())
}