use crate::modules::collaboration::domain::models::{
    CollaborationId, CollaborationSession, CursorPosition, Participant, ParticipantId,
    ParticipantRole, SharedMessage,
};
use crate::modules::collaboration::ports::CollaborationRepository;
use crate::shared::kernel::result::AppResult;
use chrono::Utc;

/// Use case: Create collaboration session
pub(crate) async fn create_session<R>(
    repo: &R,
    name: String,
    owner_name: String,
    session_id: String,
) -> AppResult<CollaborationSession>
where
    R: CollaborationRepository + ?Sized,
{
    // Side effects (ID generation, timestamp) in application layer
    let owner_id = ParticipantId::from_string(uuid::Uuid::new_v4().to_string());
    let collaboration_id = CollaborationId::from_string(uuid::Uuid::new_v4().to_string());
    let now = Utc::now();

    let owner = Participant {
        id: owner_id,
        name: owner_name,
        role: ParticipantRole::Owner,
        joined_at: now,
        is_online: true,
        cursor_position: None,
    };

    let collaboration =
        CollaborationSession::create(collaboration_id, name, owner, session_id, now);
    repo.save(&collaboration).await?;

    Ok(collaboration)
}

/// Use case: Join collaboration session
pub(crate) async fn join_session<R>(
    repo: &R,
    collaboration_id: &CollaborationId,
    participant: Participant,
) -> AppResult<CollaborationSession>
where
    R: CollaborationRepository + ?Sized,
{
    let mut session = repo.find_by_id(collaboration_id).await?.ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(
            "Collaboration session not found".to_string(),
        )
    })?;

    session.add_participant(participant);
    repo.save(&session).await?;

    Ok(session)
}

/// Use case: Update cursor position
pub(crate) async fn update_cursor<R>(
    repo: &R,
    collaboration_id: &CollaborationId,
    participant_id: &ParticipantId,
    position: CursorPosition,
) -> AppResult<()>
where
    R: CollaborationRepository + ?Sized,
{
    let mut session = repo.find_by_id(collaboration_id).await?.ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(
            "Collaboration session not found".to_string(),
        )
    })?;

    session.update_cursor(participant_id, position);
    repo.save(&session).await?;

    Ok(())
}

/// Use case: Leave session
pub(crate) async fn leave_session<R>(
    repo: &R,
    collaboration_id: &CollaborationId,
    participant_id: &ParticipantId,
) -> AppResult<CollaborationSession>
where
    R: CollaborationRepository + ?Sized,
{
    let mut session = repo.find_by_id(collaboration_id).await?.ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(
            "Collaboration session not found".to_string(),
        )
    })?;

    session.remove_participant(participant_id);
    repo.save(&session).await?;

    Ok(session)
}

/// Use case: Send shared message
pub(crate) async fn send_message<R>(
    repo: &R,
    collaboration_id: &CollaborationId,
    sender_id: ParticipantId,
    content: String,
    message_type: crate::modules::collaboration::domain::models::SharedMessageType,
) -> AppResult<SharedMessage>
where
    R: CollaborationRepository + ?Sized,
{
    let message = SharedMessage {
        id: uuid::Uuid::new_v4().to_string(),
        collaboration_id: collaboration_id.clone(),
        sender_id,
        content,
        timestamp: chrono::Utc::now(),
        message_type,
    };

    repo.save_message(&message).await?;
    Ok(message)
}
