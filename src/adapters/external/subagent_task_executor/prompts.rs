use std::env;

use crate::modules::subagents::domain::models::subagent::TaskType;

/// Build the system prompt that scopes the model to a specific task type.
pub(super) fn system_prompt_for(task_type: &TaskType) -> String {
    let base = "You are a senior software engineer working as a specialized subagent. \
        Respond with concrete, actionable findings only — no preamble, no hedging.";
    let role = match task_type {
        TaskType::CodeReview => {
            "Focus on correctness, maintainability, and test coverage. \
            List concrete issues with file/line references and suggested fixes."
        }
        TaskType::BugDetection => {
            "Identify likely bugs, race conditions, and edge cases. \
            For each finding, give the location, the failure mode, and a minimal fix."
        }
        TaskType::Refactoring => {
            "Propose refactorings that preserve behaviour. \
            Prefer small, mechanical steps and cite the design pattern or principle applied."
        }
        TaskType::Documentation => {
            "Produce documentation matching the surrounding style. \
            Include purpose, parameters, return values, errors, and at least one example."
        }
        TaskType::TestGeneration => {
            "Generate tests covering happy paths, edge cases, and error paths. \
            Use the project's existing test framework and naming conventions."
        }
        TaskType::SecurityAudit => {
            "Enumerate security risks with severity, attack vector, and remediation. \
            Reference OWASP categories where applicable."
        }
        TaskType::PerformanceAnalysis => {
            "Identify hot paths and complexity bottlenecks. \
            Quantify the expected improvement and propose a measurement plan."
        }
        TaskType::DependencyUpdate => {
            "List outdated or vulnerable dependencies, the target version, \
            and any breaking changes that affect this codebase."
        }
        TaskType::Custom(_) => "Complete the requested task precisely and concisely.",
    };
    format!("{base}\n\n{role}")
}

/// Map a `TaskType` to the model name used for execution.
///
/// Defaults to `gpt-4o-mini` for cost efficiency; callers can override via the
/// `SUBAGENT_MODEL` environment variable.
pub(super) fn model_for(_task_type: &TaskType) -> String {
    env::var("SUBAGENT_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string())
}
