// Guardrail handler - runs a one-shot input check through the GuardrailChecker port

use crate::adapters::external::guardrail_checker::DefaultGuardrailChecker;
use crate::adapters::external::guardrail_manager::InMemoryGuardrailManager;
use crate::modules::guardrails::ports::GuardrailChecker;
use crate::presentation::cli::output;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run(input: String, guardrail_type: String) -> AppResult<()> {
    output::print_section(&format!(
        "Running guardrail check: {} (type: {})",
        input, guardrail_type
    ));

    let manager = InMemoryGuardrailManager::new();
    if let Err(e) = manager.initialize_default_guardrails().await {
        output::print_error(&format!("Failed to initialize guardrails: {}", e));
    }
    let checker = DefaultGuardrailChecker::new(manager);

    match checker.check_input(&input).await {
        Ok(checks) => output::print_guardrail_report(&checks),
        Err(e) => output::print_error(&format!("Error in guardrail check: {}", e)),
    }

    Ok(())
}
