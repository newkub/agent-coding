use crate::modules::sandbox::domain::models::{Command, SandboxConfig};
use crate::modules::sandbox::domain::operations::calculate_risk_score;

/// Service: Filter commands by risk level
pub(crate) fn filter_by_risk(commands: &[Command], max_risk: u32) -> Vec<&Command> {
    commands
        .iter()
        .filter(|c| calculate_risk_score(&c.command) <= max_risk)
        .collect()
}

/// Service: Categorize commands
pub(crate) fn categorize_commands(commands: &[Command]) -> CommandCategories<'_> {
    let mut categories = CommandCategories::default();
    
    for cmd in commands {
        let risk = calculate_risk_score(&cmd.command);
        if risk == 0 {
            categories.safe.push(cmd);
        } else if risk < 25 {
            categories.low_risk.push(cmd);
        } else if risk < 50 {
            categories.medium_risk.push(cmd);
        } else {
            categories.high_risk.push(cmd);
        }
    }
    
    categories
}

#[derive(Debug, Default)]
pub(crate) struct CommandCategories<'a> {
    pub safe: Vec<&'a Command>,
    pub low_risk: Vec<&'a Command>,
    pub medium_risk: Vec<&'a Command>,
    pub high_risk: Vec<&'a Command>,
}

/// Service: Generate sandbox command
pub(crate) fn generate_sandbox_command(cmd: &str, config: &SandboxConfig) -> Vec<String> {
    let mut docker_cmd = vec![
        "docker".to_string(),
        "run".to_string(),
        "--rm".to_string(),
        "-i".to_string(),
        "--memory".to_string(),
        format!("{}m", config.memory_limit_mb),
        "--cpus".to_string(),
        config.cpu_limit.to_string(),
    ];
    
    if !config.network_enabled {
        docker_cmd.push("--network".to_string());
        docker_cmd.push("none".to_string());
    }
    
    if config.read_only_filesystem {
        docker_cmd.push("--read-only".to_string());
    }
    
    // Mount allowed paths
    for path in &config.allowed_paths {
        docker_cmd.push("-v".to_string());
        docker_cmd.push(format!("{}:{}:ro", path, path));
    }
    
    docker_cmd.push(config.image.clone());
    docker_cmd.push("sh".to_string());
    docker_cmd.push("-c".to_string());
    docker_cmd.push(cmd.to_string());
    
    docker_cmd
}