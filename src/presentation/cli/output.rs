// CLI output rendering helpers
// Pure formatting functions: take domain values, return strings or print to stdout.
// Keeping rendering in one place keeps the handlers thin and makes output
// formatting easy to test in isolation.

use crate::modules::audit::domain::models::AuditEntry;
use crate::modules::automation::domain::models::issue_pr::AutomationWorkflow;
use crate::modules::collaboration::domain::models::CollaborationSession;
use crate::modules::guardrails::domain::models::guardrail::GuardrailCheck;
use crate::modules::macros::domain::models::Macro;
use crate::modules::onboarding::domain::models::codebase_analysis::CodebaseAnalysis;
use crate::modules::performance::application::usecases::analyze_performance::PerformanceAnalysisResult;
use crate::modules::performance::domain::models::metrics::{
    OptimizationSuggestion, PerformanceSnapshot,
};
use crate::modules::session::domain::models::Session;
use crate::modules::share::domain::models::share_link::ShareLink;
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

pub(crate) fn print_subagent_details(agent: &Subagent) {
    println!("  ID:          {}", agent.id);
    println!("  Name:        {}", agent.name);
    println!("  Description: {}", agent.description);
    println!("  Type:        {:?}", agent.agent_type);
    println!("  Capabilities: {:?}", agent.capabilities);
    println!("  Status:      {:?}", agent.status);
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
    println!(
        "  Response Time: {}",
        m.response_time_ms
            .map_or_else(|| "n/a".to_string(), |ms| format!("{ms}ms"))
    );
    println!(
        "  Throughput: {}",
        m.throughput
            .map_or_else(|| "n/a".to_string(), |t| format!("{t:.1} req/s"))
    );
    println!(
        "  Error Rate: {}",
        m.error_rate
            .map_or_else(|| "n/a".to_string(), |e| format!("{:.2}%", e * 100.0))
    );
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

pub(crate) fn print_snapshot_list(snapshots: &[PerformanceSnapshot]) {
    if snapshots.is_empty() {
        println!("No performance snapshots found.");
        return;
    }

    println!("Performance snapshots:");
    for snapshot in snapshots {
        println!(
            "  - {} ({}) @ {}",
            snapshot.name, snapshot.id, snapshot.created_at
        );
    }
}

pub(crate) fn print_suggestions(suggestions: &[OptimizationSuggestion]) {
    if suggestions.is_empty() {
        println!("No optimization suggestions.");
        return;
    }

    println!("Optimization suggestions:");
    for suggestion in suggestions {
        println!(
            "  - [{:?}] {} (improvement: {:.0}%)",
            suggestion.impact,
            suggestion.title,
            suggestion.estimated_improvement * 100.0
        );
    }
}

pub(crate) fn print_collaboration_sessions(sessions: &[CollaborationSession]) {
    if sessions.is_empty() {
        println!("No active collaboration sessions.");
        return;
    }

    println!("Collaboration sessions:");
    for session in sessions {
        println!(
            "  - {} ({}) [{:?}] participants={}",
            session.name,
            session.id.as_str(),
            session.status,
            session.participants.len()
        );
    }
}

pub(crate) fn print_collaboration_session(session: &CollaborationSession) {
    println!("Collaboration session:");
    println!("  ID:           {}", session.id.as_str());
    println!("  Name:         {}", session.name);
    println!("  Status:       {:?}", session.status);
    println!("  AI session:   {}", session.session_id);
    println!("  Participants: {}", session.participants.len());
    for participant in &session.participants {
        println!(
            "    - {} ({}) {:?} online={}",
            participant.name,
            participant.id.as_str(),
            participant.role,
            participant.is_online
        );
    }
}

pub(crate) fn print_macro_list(macros: &[Macro]) {
    if macros.is_empty() {
        println!("No macros found.");
        return;
    }

    println!("Macros:");
    for macro_def in macros {
        println!(
            "  - {} ({}): {} steps, {} uses",
            macro_def.name,
            macro_def.id.as_str(),
            macro_def.step_count(),
            macro_def.usage_count
        );
    }
}

pub(crate) fn print_headless_sessions(sessions: &[String]) {
    if sessions.is_empty() {
        println!("No headless sessions found.");
        return;
    }

    println!("Headless sessions:");
    for session_id in sessions {
        println!("  - {session_id}");
    }
}

pub(crate) fn print_session_list(sessions: &[Session]) {
    if sessions.is_empty() {
        println!("No sessions found.");
        return;
    }

    println!("Sessions:");
    for session in sessions {
        println!(
            "  - {} ({}): {} messages",
            session.name,
            session.id,
            session.messages.len()
        );
    }
}

pub(crate) fn print_audit_entries(entries: &[AuditEntry]) {
    if entries.is_empty() {
        println!("No audit entries found.");
        return;
    }

    println!("Audit entries:");
    for entry in entries {
        println!(
            "  - {} | {} | {:?} | actor={} | resource={} | {:?}",
            entry.timestamp,
            entry.id.as_str(),
            entry.action,
            entry.actor.name,
            entry.resource.type_,
            entry.result
        );
    }
}

pub(crate) fn print_share_link_created(link: &ShareLink, url: &str) {
    println!("Share link created:");
    println!("  ID:      {}", link.id);
    println!("  Token:   {}", link.token);
    println!("  URL:     {}", url);
    println!("  Active:  {}", link.is_active);
    println!(
        "  Expires: {}",
        link.expires_at
            .map_or("never".to_string(), |d| d.to_rfc3339())
    );
}

pub(crate) fn print_share_link_deactivated(link: &ShareLink) {
    println!("Share link deactivated:");
    println!("  ID:     {}", link.id);
    println!("  Token:  {}", link.token);
    println!("  Active: {}", link.is_active);
}

pub(crate) fn print_share_link_accessed(link: &ShareLink) {
    println!("Share link accessed:");
    println!("  ID:           {}", link.id);
    println!("  Token:        {}", link.token);
    println!("  Access count: {}", link.access_count);
    println!("  Active:       {}", link.is_active);
}
