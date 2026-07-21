// Automation handler - executes the ExecuteAutomationUseCase for issue-to-PR workflows

use crate::modules::automation::domain::models::issue_pr::{
    AutomationConfig, AutomationWorkflow, Issue,
};
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run(repository: String, number: u32) -> AppResult<()> {
    output::print_section(&format!("Automating issue #{} in {}", number, repository));

    let container = DIContainer::new().build().await?;

    let use_case = container.execute_automation_use_case().ok_or_else(|| {
        crate::shared::kernel::result::AppError::State(
            "Execute automation use case not available".to_string(),
        )
    })?;

    // In a production system, the issue would be fetched via the GitHubClient port.
    // For the CLI demo we build a minimal issue payload.
    let issue = Issue::new(
        number,
        "Automated Issue".to_string(),
        "Description".to_string(),
        "user".to_string(),
        repository.clone(),
    );

    let mut workflow = AutomationWorkflow::new(issue);
    let config = AutomationConfig::default();

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
