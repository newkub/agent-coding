use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Headless command entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeadlessCommand {
    pub id: String,
    pub command_type: CommandType,
    pub input: String,
    pub context: CommandContext,
    pub output: Option<String>,
    pub status: CommandStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommandType {
    Chat,
    FileRead,
    FileWrite,
    CommandExecute,
    SessionList,
    SessionCreate,
    SessionLoad,
    Help,
    Exit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandContext {
    pub session_id: Option<String>,
    pub working_directory: String,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommandStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Headless session configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeadlessConfig {
    pub output_format: OutputFormat,
    pub stream_responses: bool,
    pub include_metadata: bool,
    pub max_output_length: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

impl Default for HeadlessConfig {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Text,
            stream_responses: false,
            include_metadata: false,
            max_output_length: None,
        }
    }
}

impl HeadlessCommand {
    pub fn new(command_type: CommandType, input: String, context: CommandContext) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            command_type,
            input,
            context,
            output: None,
            status: CommandStatus::Pending,
            started_at: Utc::now(),
            completed_at: None,
            error: None,
        }
    }

    pub fn complete(&mut self, output: String) {
        self.output = Some(output);
        self.status = CommandStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn fail(&mut self, error: String) {
        self.error = Some(error);
        self.status = CommandStatus::Failed;
        self.completed_at = Some(Utc::now());
    }

    pub const fn is_interactive(&self) -> bool {
        matches!(self.command_type, CommandType::Chat)
    }
}

impl CommandContext {
    pub fn new(working_directory: String) -> Self {
        Self {
            session_id: None,
            working_directory,
            environment: HashMap::new(),
        }
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headless_command_creation() {
        let context = CommandContext::new("/test".to_string());
        let command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);

        assert_eq!(command.status, CommandStatus::Pending);
        assert!(command.is_interactive());
    }

    #[test]
    fn test_headless_command_complete() {
        let context = CommandContext::new("/test".to_string());
        let mut command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);

        command.complete("Response".to_string());

        assert_eq!(command.status, CommandStatus::Completed);
        assert_eq!(command.output, Some("Response".to_string()));
        assert!(command.completed_at.is_some());
    }

    #[test]
    fn test_headless_command_fail() {
        let context = CommandContext::new("/test".to_string());
        let mut command = HeadlessCommand::new(CommandType::Chat, "test".to_string(), context);

        command.fail("Error".to_string());

        assert_eq!(command.status, CommandStatus::Failed);
        assert_eq!(command.error, Some("Error".to_string()));
    }

    #[test]
    fn test_command_context_with_session() {
        let context =
            CommandContext::new("/test".to_string()).with_session("session-123".to_string());

        assert_eq!(context.session_id, Some("session-123".to_string()));
    }
}
