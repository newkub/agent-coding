use super::models::SecurityLevel;

/// Pure domain operation: Check if command is safe
pub fn is_safe_command(command: &str) -> bool {
    let cmd_lower = command.to_lowercase();
    
    // Read-only commands that are generally safe
    let safe_commands = [
        "git status", "git diff", "git log", "git show", "git branch",
        "git remote", "git tag", "git stash list", "git reflog",
        "ls", "pwd", "cat", "head", "tail", "grep", "find", "wc",
        "echo", "env", "whoami", "date",
    ];
    
    // Commands that should never be auto-approved
    let dangerous_patterns = [
        "rm -rf", "rm /", "dd if=", "mkfs", "fdisk",
        "curl | sh", "wget | sh", "bash -c", "sh -c",
        "chmod 777", "chown", "sudo", "su ",
        "nc ", "netcat", "ssh", "scp",
        "dropdb", "delete from", "truncate",
    ];
    
    // Check if starts with safe command
    let is_read_only = safe_commands.iter().any(|safe| cmd_lower.starts_with(safe));
    
    // Check for dangerous patterns
    let is_dangerous = dangerous_patterns.iter().any(|pat| cmd_lower.contains(pat));
    
    is_read_only && !is_dangerous
}

/// Pure domain operation: Determine security level for command
pub fn determine_security_level(command: &str) -> SecurityLevel {
    if is_safe_command(command) {
        SecurityLevel::Safe
    } else {
        SecurityLevel::Sandboxed
    }
}

/// Pure domain operation: Validate command
pub fn validate_command(command: &str) -> Result<(), CommandValidationError> {
    if command.trim().is_empty() {
        return Err(CommandValidationError::EmptyCommand);
    }
    if command.len() > 10000 {
        return Err(CommandValidationError::CommandTooLong);
    }
    // Check for null bytes or other dangerous characters
    if command.contains('\0') {
        return Err(CommandValidationError::InvalidCharacters);
    }
    Ok(())
}

/// Validation errors
#[derive(Debug, Clone)]
pub enum CommandValidationError {
    EmptyCommand,
    CommandTooLong,
    InvalidCharacters,
}

impl std::fmt::Display for CommandValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCommand => write!(f, "Command cannot be empty"),
            Self::CommandTooLong => write!(f, "Command exceeds maximum length"),
            Self::InvalidCharacters => write!(f, "Command contains invalid characters"),
        }
    }
}

/// Pure domain operation: Match command against rule pattern
pub fn match_rule_pattern(command: &str, pattern: &str) -> bool {
    if let Ok(re) = regex::Regex::new(pattern) {
        re.is_match(command)
    } else {
        // Fallback to simple contains check
        command.contains(pattern)
    }
}

/// Pure domain operation: Calculate command risk score
pub fn calculate_risk_score(command: &str) -> u32 {
    let mut score = 0u32;
    let cmd_lower = command.to_lowercase();
    
    // High risk indicators
    if cmd_lower.contains("sudo") || cmd_lower.contains("su ") { score += 40; }
    if cmd_lower.contains("rm -") { score += 30; }
    if cmd_lower.contains("chmod 7") { score += 30; }
    if cmd_lower.contains("curl") || cmd_lower.contains("wget") { score += 20; }
    if cmd_lower.contains("| sh") || cmd_lower.contains("> /") { score += 50; }
    if cmd_lower.contains("git push") || cmd_lower.contains("git force") { score += 25; }
    if cmd_lower.contains("npm install") || cmd_lower.contains("pip install") { score += 15; }
    if cmd_lower.contains("docker run") { score += 20; }
    if cmd_lower.contains("exec") { score += 25; }
    
    // File system modifications
    if cmd_lower.contains("mv ") || cmd_lower.contains("cp ") { score += 10; }
    if cmd_lower.contains("mkdir ") { score += 5; }
    
    // Network operations
    if cmd_lower.contains("curl") || cmd_lower.contains("wget") { score += 15; }
    if cmd_lower.contains("ssh") || cmd_lower.contains("scp") { score += 35; }
    
    score.min(100)
}