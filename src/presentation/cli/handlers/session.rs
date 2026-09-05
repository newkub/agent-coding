// Session handler - lists, creates, deletes, and searches sessions

use crate::modules::session::application::services::{
    search_sessions, sort_sessions, SortCriteria,
};
use crate::modules::session::domain::models::SessionId;
use crate::modules::session::domain::operations::create_session;
use crate::presentation::cli::commands::SessionCommands;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(command: SessionCommands) -> AppResult<()> {
    match command {
        SessionCommands::List => list_sessions().await,
        SessionCommands::Create { name } => create_session_cmd(name).await,
        SessionCommands::Delete { id } => delete_session_cmd(id).await,
        SessionCommands::Search { query } => search_session_cmd(query).await,
    }
}

async fn list_sessions() -> AppResult<()> {
    output::print_section("Sessions");

    let container = DIContainer::new().build().await?;
    let repo = container
        .session_repo()
        .ok_or_else(|| AppError::State("Session repository not available".to_string()))?;

    let sessions = repo.find_all().await?;
    let sorted = sort_sessions(sessions, SortCriteria::Name);
    output::print_session_list(&sorted);
    Ok(())
}

async fn create_session_cmd(name: String) -> AppResult<()> {
    output::print_section(&format!("Creating session: {}", name));

    let session = create_session(name).map_err(|e| AppError::ValidationError(e.to_string()))?;

    let container = DIContainer::new().build().await?;
    let repo = container
        .session_repo()
        .ok_or_else(|| AppError::State("Session repository not available".to_string()))?;

    repo.save(&session).await?;
    output::print_info(&format!(
        "Created session: {} ({})",
        session.name, session.id
    ));
    Ok(())
}

async fn delete_session_cmd(id: String) -> AppResult<()> {
    output::print_section(&format!("Deleting session: {}", id));

    let container = DIContainer::new().build().await?;
    let repo = container
        .session_repo()
        .ok_or_else(|| AppError::State("Session repository not available".to_string()))?;

    let session_id = SessionId::from_string(id);
    repo.delete(&session_id).await?;
    output::print_info(&format!("Deleted session: {}", session_id));
    Ok(())
}

async fn search_session_cmd(query: String) -> AppResult<()> {
    output::print_section(&format!("Searching sessions: {}", query));

    let container = DIContainer::new().build().await?;
    let repo = container
        .session_repo()
        .ok_or_else(|| AppError::State("Session repository not available".to_string()))?;

    let sessions = repo.find_all().await?;
    let results = search_sessions(&sessions, &query);
    output::print_session_list(&results);
    Ok(())
}
