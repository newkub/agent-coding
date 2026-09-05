// Automation handler - executes the ExecuteAutomationUseCase for issue-to-PR workflows

use crate::adapters::config::loader::load_automation_config;
use crate::modules::automation::domain::models::issue_pr::AutomationWorkflow;
use crate::modules::automation::ports::GitHubClient;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run(repository: String, number: u32) -> AppResult<()> {
    output::print_section(&format!("Automating issue #{} in {}", number, repository));

    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;

    let use_case = container.execute_automation_use_case().ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(
            "Execute automation use case not available".to_string(),
        )
    })?;

    let github = container.github_client().ok_or_else(|| {
        crate::shared::kernel::result::AppError::State("GitHub client not available".to_string())
    })?;
    let issue = github.get_issue(&repository, number).await?;

    let mut workflow = AutomationWorkflow::new(issue);
    let config = load_automation_config();

    match use_case.execute(&mut workflow, &config).await {
        Ok(_) => {
            output::print_automation_result(&repository, &workflow);
        }
        Err(e) => {
            output::print_error(&format!("Error in automation: {}", e));
        }
    }

    Ok(())
}
