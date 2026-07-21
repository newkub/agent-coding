// Subagent handler - lists or executes subagent tasks
// Production wiring goes through the ExecuteSubagentTaskUseCase via DI.
// Listing is rendered from the static catalog so it works without booting the container.

use crate::adapters::external::subagent_manager::InMemorySubagentManager;
use crate::adapters::external::subagent_task_executor::DefaultSubagentTaskExecutor;
use crate::modules::subagents::application::usecases::execute_subagent_task::ExecuteSubagentTaskUseCase;
use crate::modules::subagents::domain::models::subagent::TaskType;
use crate::modules::subagents::ports::SubagentManager;
use crate::presentation::cli::commands::SubagentCommands;
use crate::presentation::cli::output;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run(command: SubagentCommands) -> AppResult<()> {
    match command {
        SubagentCommands::List => {
            let manager = InMemorySubagentManager::new();
            // Initialize default subagents so listing reflects the real catalog.
            if let Err(e) = manager.initialize_default_subagents().await {
                output::print_error(&format!("Failed to load subagent catalog: {}", e));
            }
            let subagents = manager.get_available_subagents().await.unwrap_or_default();
            output::print_subagent_list(&subagents);
            Ok(())
        }
        SubagentCommands::Execute { agent, input } => execute_subagent(&agent, &input).await,
    }
}

async fn execute_subagent(agent: &str, input: &str) -> AppResult<()> {
    output::print_section(&format!(
        "Executing subagent: {} with input: {}",
        agent, input
    ));

    let manager = InMemorySubagentManager::new();
    manager.initialize_default_subagents().await?;

    let executor = DefaultSubagentTaskExecutor::new();
    let use_case = ExecuteSubagentTaskUseCase::new(manager, executor);

    let task_type = parse_task_type(agent);
    let context = crate::modules::subagents::domain::models::subagent::TaskContext::new();

    match use_case
        .create_and_execute(task_type, input.to_string(), context)
        .await
    {
        Ok(task) => {
            if let Some(result) = task.output {
                output::print_subagent_result(&result);
            } else {
                output::print_info("Subagent completed without output");
            }
        }
        Err(e) => {
            output::print_error(&format!("Error executing subagent: {}", e));
        }
    }

    Ok(())
}

fn parse_task_type(agent: &str) -> TaskType {
    match agent {
        "code-reviewer" => TaskType::CodeReview,
        "bug-hunter" => TaskType::BugDetection,
        "refactorer" => TaskType::Refactoring,
        "documenter" => TaskType::Documentation,
        "tester" => TaskType::TestGeneration,
        "security-auditor" => TaskType::SecurityAudit,
        "performance-optimizer" => TaskType::PerformanceAnalysis,
        other => TaskType::Custom(other.to_string()),
    }
}
