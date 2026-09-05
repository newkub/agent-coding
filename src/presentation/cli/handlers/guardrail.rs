// Guardrail handler - runs a one-shot input check through the GuardrailChecker port

use crate::adapters::external::guardrail_checker::DefaultGuardrailChecker;
use crate::modules::guardrails::ports::GuardrailChecker;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(input: String, guardrail_type: String) -> AppResult<()> {
    output::print_section(&format!(
        "Running guardrail check: {} (type: {})",
        input, guardrail_type
    ));

    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;
    let manager = container
        .guardrail_manager()
        .cloned()
        .ok_or_else(|| AppError::State("Guardrail manager not available".to_string()))?;
    let checker = DefaultGuardrailChecker::new(manager);

    match checker.check_input(&input).await {
        Ok(checks) => output::print_guardrail_report(&checks),
        Err(e) => output::print_error(&format!("Error in guardrail check: {}", e)),
    }

    Ok(())
}
