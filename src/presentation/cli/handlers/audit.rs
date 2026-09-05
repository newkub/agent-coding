// Audit handler - logs and queries audit entries

use crate::modules::audit::application::usecases::{log_entry, query_logs, AuditQuery};
use crate::modules::audit::domain::models::{Actor, ActorType, AuditAction, Resource};
use crate::presentation::cli::commands::AuditCommands;
use crate::presentation::cli::output;
use crate::presentation::tui::di::DIContainer;
use crate::shared::kernel::result::{AppError, AppResult};

pub(crate) async fn run(command: AuditCommands) -> AppResult<()> {
    match command {
        AuditCommands::Log { message, category } => log_audit_entry(message, category).await,
        AuditCommands::Query { from, level } => query_audit_entries(from, level).await,
    }
}

fn build_action(category: &str, message: &str) -> AuditAction {
    match category {
        "command" => AuditAction::CommandExecute {
            command: message.to_string(),
        },
        "file" => AuditAction::FileWrite {
            path: message.to_string(),
        },
        "git" => AuditAction::GitBranch {
            name: message.to_string(),
        },
        "session" => AuditAction::SessionCreate {
            name: message.to_string(),
        },
        "ai" => AuditAction::AiRequest {
            model: message.to_string(),
            tokens: 0,
        },
        "config" => AuditAction::ConfigChange {
            key: message.to_string(),
        },
        _ => AuditAction::ConfigChange {
            key: message.to_string(),
        },
    }
}

async fn log_audit_entry(message: String, category: String) -> AppResult<()> {
    output::print_section(&format!("Audit log: [{}] {}", category, message));

    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;
    let repo = container
        .audit_repo()
        .ok_or_else(|| AppError::State("Audit repository not available".to_string()))?;

    let action = build_action(&category, &message);
    let actor = Actor {
        type_: ActorType::User,
        id: "cli".to_string(),
        name: "cli".to_string(),
    };
    let resource = Resource {
        type_: category.clone(),
        id: "-".to_string(),
        path: None,
    };

    let entry = log_entry(repo, action, actor, resource).await?;
    output::print_info(&format!("Logged audit entry: {}", entry.id.as_str()));
    Ok(())
}

async fn query_audit_entries(from: Option<String>, level: Option<String>) -> AppResult<()> {
    output::print_section("Audit log query");

    let mut container = DIContainer::new().build().await?;
    container.init_db().await?;
    let repo = container
        .audit_repo()
        .ok_or_else(|| AppError::State("Audit repository not available".to_string()))?;

    let start_time = from
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|d| d.with_timezone(&chrono::Utc));

    let filters = AuditQuery {
        start_time,
        category: level,
        limit: Some(100),
        ..AuditQuery::default()
    };

    let entries = query_logs(repo, filters).await?;
    output::print_audit_entries(&entries);
    Ok(())
}
