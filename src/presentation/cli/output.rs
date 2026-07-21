// CLI output rendering helpers
// Pure formatting functions: take domain values, return strings or print to stdout.
// Keeping rendering in one place keeps the handlers thin and makes output
// formatting easy to test in isolation.

use crate::modules::automation::domain::models::issue_pr::AutomationWorkflow;
use crate::modules::guardrails::domain::models::guardrail::GuardrailCheck;
use crate::modules::onboarding::domain::models::codebase_analysis::CodebaseAnalysis;
use crate::modules::performance::application::usecases::analyze_performance::PerformanceAnalysisResult;
use crate::modules::performance::domain::models::metrics::PerformanceSnapshot;
use crate::modules::subagents::domain::models::subagent::Subagent;

/// Version metadata shown by `agent-tui version`
pub(crate) struct VersionInfo {
    pub(crate) name: &'static str,
    pub(crate) version: &'static str,
    pub(crate) description: &'static str,
    pub(crate) architecture: &'static str,
}

impl VersionInfo {
    pub(crate) fn current() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            architecture: "Clean Architecture (FP-style) with Vertical Slice",
        }
    }
}

pub(crate) fn print_version(info: &VersionInfo) {
    println!("{} {}", info.name, info.version);
    println!("{}", info.description);
    println!("Architecture: {}", info.architecture);
}

pub(crate) fn print_section(title: &str) {
    println!();
    println!("== {} ==", title);
}

pub(crate) fn print_info(message: &str) {
    println!("{}", message);
}

pub(crate) fn print_error(message: &str) {
    eprintln!("{}", message);
}

pub(crate) fn print_codebase_analysis(analysis: &CodebaseAnalysis) {
    println!("Codebase Analysis Complete:");
    println!("  Project Type: {}", analysis.summary);
    println!("  Total Files: {}", analysis.structure.total_files);
    println!("  Total Lines: {}", analysis.structure.total_lines);
    println!("  Languages: {:?}", analysis.structure.languages);
    println!("  Tech Stack: {:?}", analysis.tech_stack.frameworks);
    println!("  Entry Points: {:?}", analysis.entry_points);
}

pub(crate) fn print_automation_result(repository: &str, workflow: &AutomationWorkflow) {
    println!("Automation workflow completed successfully");
    if let Some(pr) = &workflow.pr {
        println!("  PR Created: #{}", pr.number);
        println!("  PR URL: {}/{}", repository, pr.number);
    }
}

pub(crate) fn print_subagent_list(subagents: &[Subagent]) {
    println!("Available Subagents:");
    for agent in subagents {
        println!("  - {}: {}", agent.id, agent.description);
    }
}

pub(crate) fn print_subagent_result(output: &str) {
    println!("Output: {}", output);
}

pub(crate) fn print_guardrail_report(checks: &[GuardrailCheck]) {
    let passed = checks.iter().all(|c| c.passed);
    if passed {
        println!("Guardrail check passed");
        return;
    }

    println!("Guardrail check failed:");
    for check in checks {
        if !check.passed {
            println!(
                "  - {}: {} violations",
                check.guardrail_name,
                check.violations.len()
            );
        }
    }
}

pub(crate) fn print_performance_analysis(result: &PerformanceAnalysisResult) {
    let m = &result.metrics;
    println!("Analyzing performance metrics...");
    println!("  CPU Usage: {:.1}%", m.cpu_usage);
    println!(
        "  Memory: {} / {} MB ({:.1}%)",
        m.memory_usage / (1024 * 1024),
        m.memory_total / (1024 * 1024),
        m.memory_usage_percentage()
    );
    println!("  Response Time: {}ms", m.response_time_ms);
    println!("  Throughput: {:.1} req/s", m.throughput);
    println!("  Error Rate: {:.2}%", m.error_rate * 100.0);
    println!("  Score: {:.1}/100", result.score);
    println!("  Healthy: {}", result.is_healthy);
}

pub(crate) fn print_performance_snapshot(snapshot: &PerformanceSnapshot) {
    println!("Performance snapshot created:");
    println!("  ID: {}", snapshot.id);
    println!("  Name: {}", snapshot.name);
    println!("  Created: {}", snapshot.created_at);
}

pub(crate) fn print_performance_report(result: &PerformanceAnalysisResult) {
    println!("Performance Report");
    println!("==================");
    print_performance_analysis(result);
    if !result.suggestions.is_empty() {
        println!();
        println!("Optimization Suggestions:");
        for suggestion in &result.suggestions {
            println!(
                "  - [{}] {} (improvement: {:.0}%)",
                suggestion.title,
                suggestion.description,
                suggestion.estimated_improvement * 100.0
            );
        }
    }
}
