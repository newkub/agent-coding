// Share link handler - creates, deactivates, and accesses share links

use crate::adapters::external::share_link_notifier::LogShareLinkNotifier;
use crate::adapters::external::share_link_url_generator::DefaultShareLinkUrlGenerator;
use crate::modules::share::application::usecases::{
    access_share_link::AccessShareLinkUseCase, create_share_link::CreateShareLinkUseCase,
    deactivate_share_link::DeactivateShareLinkUseCase,
};
use crate::modules::share::domain::operations::share_link_operations::ShareAction;
use crate::presentation::cli::commands::ShareCommands;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(command: ShareCommands) -> AppResult<()> {
    match command {
        ShareCommands::Create { session_name } => create_share_link(session_name).await,
        ShareCommands::Deactivate { token } => deactivate_share_link(token).await,
        ShareCommands::Access { token } => access_share_link(token).await,
    }
}

async fn create_share_link(session_name: String) -> AppResult<()> {
    output::print_section(&format!(
        "Creating share link for session: {}",
        session_name
    ));

    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;

    let session_repo = container
        .session_repo()
        .ok_or_else(|| AppError::State("Session repository not available".to_string()))?;
    let session = session_repo
        .find_by_name(&session_name)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Session not found: {}", session_name)))?;

    let session_id = uuid::Uuid::parse_str(session.id.as_str())
        .map_err(|_| AppError::ValidationError(format!("Invalid session id: {}", session.id)))?;

    let share_link_repo = container
        .share_link_repo()
        .ok_or_else(|| AppError::State("Share link repository not available".to_string()))?;

    let use_case = CreateShareLinkUseCase::new(
        share_link_repo.clone(),
        DefaultShareLinkUrlGenerator::default(),
        LogShareLinkNotifier::new(),
    );

    let (link, url) = use_case
        .execute(session_id, Some(24), Some(10), None)
        .await?;
    output::print_share_link_created(&link, &url);
    Ok(())
}

async fn deactivate_share_link(token: String) -> AppResult<()> {
    output::print_section(&format!("Deactivating share link: {}", token));

    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;

    let share_link_repo = container
        .share_link_repo()
        .ok_or_else(|| AppError::State("Share link repository not available".to_string()))?;

    let use_case =
        DeactivateShareLinkUseCase::new(share_link_repo.clone(), LogShareLinkNotifier::new());
    let link = use_case.execute_by_token(&token).await?;
    output::print_share_link_deactivated(&link);
    Ok(())
}

async fn access_share_link(token: String) -> AppResult<()> {
    output::print_section(&format!("Accessing share link: {}", token));

    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;

    let share_link_repo = container
        .share_link_repo()
        .ok_or_else(|| AppError::State("Share link repository not available".to_string()))?;

    let use_case =
        AccessShareLinkUseCase::new(share_link_repo.clone(), LogShareLinkNotifier::new());
    let link = use_case.execute(&token, ShareAction::Read).await?;
    output::print_share_link_accessed(&link);
    Ok(())
}
