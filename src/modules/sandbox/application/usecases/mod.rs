use crate::modules::sandbox::ports::{CommandExecutor, ApprovalEngine};
use crate::modules::sandbox::domain::models::{Command, CommandResult, SecurityLevel};
use crate::modules::sandbox::domain::operations::{determine_security_level, validate_command, calculate_risk_score};
use crate::shared::kernel::result::AppResult;

/// Use case: Execute command with security checks
pub(crate) async fn execute_command<E, A>(
    executor: &E,
    approval_engine: &A,
    command: Command,
) -> AppResult<CommandResult>
where
    E: CommandExecutor,
    A: ApprovalEngine,
{
    // Validate command
    validate_command(&command.command).map_err(|e|
        crate::shared::kernel::result::AppError::State(e.to_string())
    )?;

    // Determine security level
    let security_level = if command.security_level == SecurityLevel::Safe {
        determine_security_level(&command.command)
    } else {
        command.security_level
    };

    // Check approval
    let approved = approval_engine.check_approval(&command).await?;
    
    if !approved {
        return Err(crate::shared::kernel::result::AppError::State(
            "Command requires approval".to_string()
        ));
    }

    // Execute based on security level
    match security_level {
        SecurityLevel::Sandboxed => {
            executor.execute_sandboxed(&command).await
        }
        SecurityLevel::Strict => {
            Err(crate::shared::kernel::result::AppError::State(
                "Command requires explicit approval".to_string()
            ))
        }
        _ => {
            executor.execute(&command).await
        }
    }
}

/// Use case: Preview command without executing
pub(crate) fn preview_command(command: &str) -> CommandPreview {
    let risk_score = calculate_risk_score(command);
    let security_level = determine_security_level(command);
    
    CommandPreview {
        command: command.to_string(),
        risk_score,
        suggested_security_level: security_level,
        warnings: generate_warnings(command, risk_score),
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPreview {
    pub command: String,
    pub risk_score: u32,
    pub suggested_security_level: SecurityLevel,
    pub warnings: Vec<String>,
}

fn generate_warnings(command: &str, risk_score: u32) -> Vec<String> {
    let mut warnings = Vec::new();
    let cmd_lower = command.to_lowercase();
    
    if risk_score >= 50 {
        warnings.push("⚠️ High risk command detected".to_string());
    } else if risk_score >= 25 {
        warnings.push("⚡ Medium risk - proceed with caution".to_string());
    }
    
    if cmd_lower.contains("rm -rf") || cmd_lower.contains("rm /") {
        warnings.push("🚫 Deletion command - cannot be undone".to_string());
    }
    if cmd_lower.contains("| sh") || cmd_lower.contains("curl |") {
        warnings.push("⚠️ Pipe to shell - potential security risk".to_string());
    }
    if cmd_lower.contains("curl") && !cmd_lower.starts_with("curl --help") {
        warnings.push("🌐 Network request - data may leave the system".to_string());
    }
    if cmd_lower.contains("git push") {
        warnings.push("📤 Git push - changes will be uploaded".to_string());
    }
    
    warnings
}