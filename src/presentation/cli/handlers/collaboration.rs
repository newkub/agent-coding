// Collaboration handler - session management via the collaboration repository
// and the collaboration use cases (create/join/leave/send).

use crate::modules::collaboration::application::usecases;
use crate::modules::collaboration::domain::models::{
    CollaborationId, Participant, ParticipantId, ParticipantRole, SharedMessageType,
};
use crate::presentation::cli::commands::CollaborationCommands;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(command: CollaborationCommands) -> AppResult<()> {
    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;
    let repo = container
        .collaboration_repo()
        .ok_or_else(|| AppError::State("Collaboration repository not available".to_string()))?;

    match command {
        CollaborationCommands::List => {
            output::print_section("Collaboration sessions");
            let sessions = repo.find_active().await?;
            output::print_collaboration_sessions(&sessions);
        }
        CollaborationCommands::Create { name } => {
            output::print_section(&format!("Creating collaboration session: {name}"));
            let session =
                usecases::create_session(repo, name, "cli-user".to_string(), "cli".to_string())
                    .await?;
            output::print_collaboration_session(&session);
        }
        CollaborationCommands::Join { id, name } => {
            output::print_section(&format!("Joining collaboration session: {id}"));
            let participant = Participant {
                id: ParticipantId::from_string(uuid::Uuid::new_v4().to_string()),
                name,
                role: ParticipantRole::Editor,
                joined_at: chrono::Utc::now(),
                is_online: true,
                cursor_position: None,
            };
            let session =
                usecases::join_session(repo, &CollaborationId::from_string(id), participant)
                    .await?;
            output::print_collaboration_session(&session);
        }
        CollaborationCommands::Send { id, message } => {
            output::print_section(&format!("Sending message to session: {id}"));
            let cid = CollaborationId::from_string(id);
            let session = repo
                .find_by_id(&cid)
                .await?
                .ok_or_else(|| AppError::NotFound("Collaboration session not found".to_string()))?;
            // Use the most recent participant as the CLI sender.
            let sender = session
                .participants
                .last()
                .ok_or_else(|| AppError::State("Session has no participants".to_string()))?;
            let message = usecases::send_message(
                repo,
                &cid,
                sender.id.clone(),
                message,
                SharedMessageType::Chat,
            )
            .await?;
            output::print_info(&format!(
                "Sent [{}] {}: {}",
                message.timestamp.format("%H:%M:%S"),
                message.sender_id.as_str(),
                message.content
            ));
        }
        CollaborationCommands::Leave { id } => {
            output::print_section(&format!("Leaving collaboration session: {id}"));
            let cid = CollaborationId::from_string(id);
            let session = repo
                .find_by_id(&cid)
                .await?
                .ok_or_else(|| AppError::NotFound("Collaboration session not found".to_string()))?;
            // Leave with the most recently joined non-owner participant.
            let participant = session
                .participants
                .iter()
                .rev()
                .find(|p| p.role != ParticipantRole::Owner)
                .ok_or_else(|| AppError::State("No non-owner participant to remove".to_string()))?;
            let session = usecases::leave_session(repo, &cid, &participant.id).await?;
            output::print_collaboration_session(&session);
        }
    }
    Ok(())
}
