use crate::modules::session::domain::events::SessionEvent;
use crate::modules::session::domain::operations::{self};
use crate::modules::session::ports::{SessionEventPublisher, SessionRepository};
use crate::modules::session::types::*;
use crate::shared::kernel::result::{AppError, AppResult};

/// Use case: Create new session
pub(crate) async fn create_session<R, E>(
    repo: &R,
    publisher: &E,
    name: String,
) -> AppResult<Session>
where
    R: SessionRepository,
    E: SessionEventPublisher,
{
    let session = operations::create_session(name).map_err(|e| AppError::State(e.to_string()))?;
    repo.save(&session).await?;
    publisher
        .publish(SessionEvent::Created {
            session_id: session.id.0.clone(),
        })
        .await?;
    Ok(session)
}

/// Use case: Get session by ID
pub(crate) async fn get_session<R>(repo: &R, id: &SessionId) -> AppResult<Option<Session>>
where
    R: SessionRepository,
{
    repo.find_by_id(id).await
}

/// Use case: List all sessions
pub(crate) async fn list_sessions<R>(repo: &R) -> AppResult<Vec<Session>>
where
    R: SessionRepository,
{
    repo.find_all().await
}

/// Use case: Delete session
pub(crate) async fn delete_session<R, E>(repo: &R, publisher: &E, id: &SessionId) -> AppResult<()>
where
    R: SessionRepository,
    E: SessionEventPublisher,
{
    repo.delete(id).await?;
    publisher
        .publish(SessionEvent::Closed {
            session_id: id.0.clone(),
        })
        .await?;
    Ok(())
}

/// Use case: Add message to session
pub(crate) async fn add_message<R, E>(
    repo: &R,
    publisher: &E,
    session_id: &SessionId,
    message: Message,
) -> AppResult<Session>
where
    R: SessionRepository,
    E: SessionEventPublisher,
{
    let mut session = repo.find_by_id(session_id).await?.ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(format!("Session not found: {}", session_id))
    })?;

    session.add_message(message.clone());
    repo.save(&session).await?;
    publisher
        .publish(SessionEvent::MessageAdded {
            session_id: session.id.0.clone(),
            message_index: session.messages.len().saturating_sub(1),
        })
        .await?;
    Ok(session)
}

/// Use case: Get session statistics
pub(crate) fn get_session_stats(session: &Session) -> operations::SessionStats {
    operations::calculate_session_stats(session)
}
