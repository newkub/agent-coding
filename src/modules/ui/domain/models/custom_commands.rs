use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomCommands {
    pub commands: HashMap<String, CustomCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCommand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub template: String,
    pub variables: Vec<CommandVariable>,
    pub shortcut: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandVariable {
    pub name: String,
    pub default_value: String,
    pub description: String,
}

impl CustomCommands {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_command(&mut self, command: CustomCommand) {
        self.commands.insert(command.id.clone(), command);
    }

    pub fn remove_command(&mut self, id: &str) {
        self.commands.remove(id);
    }

    pub fn get_command(&self, id: &str) -> Option<&CustomCommand> {
        self.commands.get(id)
    }

    pub fn list_commands(&self) -> Vec<&CustomCommand> {
        self.commands.values().collect()
    }

    pub fn execute_command(&self, id: &str, variables: &HashMap<String, String>) -> Result<String, String> {
        let command = self.commands.get(id)
            .ok_or_else(|| format!("Command '{}' not found", id))?;

        let mut result = command.template.clone();
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }

        // Replace remaining variables with defaults
        for var in &command.variables {
            let placeholder = format!("{{{}}}", var.name);
            if result.contains(&placeholder) {
                result = result.replace(&placeholder, &var.default_value);
            }
        }

        Ok(result)
    }

    pub fn get_default_commands() -> Self {
        let mut commands = Self::new();

        commands.add_command(CustomCommand {
            id: "deploy-prod".to_string(),
            name: "Deploy to Production".to_string(),
            description: "Deploy current branch to production".to_string(),
            template: "git push origin {branch} && kubectl apply -f k8s/".to_string(),
            variables: vec![
                CommandVariable {
                    name: "branch".to_string(),
                    default_value: "main".to_string(),
                    description: "Branch to deploy".to_string(),
                },
            ],
            shortcut: Some("Ctrl+D".to_string()),
        });

        commands.add_command(CustomCommand {
            id: "run-tests".to_string(),
            name: "Run All Tests".to_string(),
            description: "Run test suite".to_string(),
            template: "cargo test --all".to_string(),
            variables: vec![],
            shortcut: Some("Ctrl+T".to_string()),
        });

        commands.add_command(CustomCommand {
            id: "clean-build".to_string(),
            name: "Clean Build".to_string(),
            description: "Clean and rebuild project".to_string(),
            template: "cargo clean && cargo build".to_string(),
            variables: vec![],
            shortcut: None,
        });

        commands
    }
}
