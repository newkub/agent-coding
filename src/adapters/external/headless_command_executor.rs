use async_trait::async_trait;

use crate::modules::headless::domain::models::command::{
    CommandStatus, CommandType, HeadlessCommand, HeadlessConfig,
};
use crate::modules::headless::domain::operations::command_operations::{
    format_output, truncate_output,
};
use crate::modules::headless::domain::validators::command_validators;
use crate::modules::headless::ports::{HeadlessCommandExecutor, OutputFormatter};
use crate::shared::kernel::result::AppError;

/// Default implementation for headless command execution
pub struct DefaultHeadlessCommandExecutor {
    formatter: DefaultOutputFormatter,
}

impl DefaultHeadlessCommandExecutor {
    pub(crate) const fn new() -> Self {
        Self {
            formatter: DefaultOutputFormatter::new(),
        }
    }
}

impl Default for DefaultHeadlessCommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HeadlessCommandExecutor for DefaultHeadlessCommandExecutor {
    async fn execute(
        &self,
        command: &mut HeadlessCommand,
        config: &HeadlessConfig,
    ) -> Result<(), AppError> {
        // Validate command
        command_validators::validate_command_for_headless(command)?;

        // Update status
        command.status = CommandStatus::Processing;

        // Execute based on command type
        let output = match command.command_type {
            CommandType::Chat => {
                // In a real implementation, this would call the AI service
                format!("AI Response to: {}", command.input)
            }
            CommandType::FileRead => {
                format!("File read: {}", command.input)
            }
            CommandType::FileWrite => {
                format!("File written: {}", command.input)
            }
            CommandType::CommandExecute => {
                format!("Command executed: {}", command.input)
            }
            CommandType::SessionList => "Available sessions: [session-1, session-2]".to_string(),
            CommandType::SessionCreate => {
                format!("Created session: {}", uuid::Uuid::new_v4())
            }
            CommandType::SessionLoad => {
                format!("Loaded session: {}", command.input)
            }
            CommandType::Help => self.generate_help_text(),
            CommandType::Exit => "Exiting...".to_string(),
        };

        // Format output
        let formatted_output = self.formatter.format(&output, config);

        // Truncate if needed
        let final_output = if let Some(max_length) = config.max_output_length {
            truncate_output(&formatted_output, max_length)
        } else {
            formatted_output
        };

        command.complete(final_output);
        Ok(())
    }

    async fn execute_batch(
        &self,
        commands: &mut [HeadlessCommand],
        config: &HeadlessConfig,
    ) -> Result<(), AppError> {
        for command in commands {
            self.execute(command, config).await?;
        }
        Ok(())
    }

    async fn get_status(&self, _command_id: &str) -> Result<HeadlessCommand, AppError> {
        // In a real implementation, this would load from storage
        Err(AppError::NotFound("Command not found".to_string()))
    }
}

impl DefaultHeadlessCommandExecutor {
    fn generate_help_text(&self) -> String {
        r#"
Agent-TUI Headless Mode Commands:

/chat <message>        - Send a chat message to the AI
/read <file>           - Read a file
/write <file>          - Write to a file
/exec <command>        - Execute a shell command
/list                  - List available sessions
/create                - Create a new session
/load <session_id>     - Load an existing session
/help                  - Show this help message
/exit                  - Exit headless mode

Output formats:
--format=text          - Plain text output (default)
--format=json          - JSON output
--format=markdown      - Markdown output

Options:
--stream               - Stream responses in real-time
--metadata             - Include metadata in output
--max-length=<n>       - Truncate output to n characters
"#
        .to_string()
    }
}

/// Default output formatter
pub struct DefaultOutputFormatter;

impl DefaultOutputFormatter {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl Default for DefaultOutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl OutputFormatter for DefaultOutputFormatter {
    fn format(&self, output: &str, config: &HeadlessConfig) -> String {
        format_output(output, &config.output_format, config.include_metadata)
    }

    async fn stream(&self, output: &str, config: &HeadlessConfig) -> Result<(), AppError> {
        if config.stream_responses {
            // In a real implementation, this would stream character by character
            println!("{}", output);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headless_executor_creation() {
        let executor = DefaultHeadlessCommandExecutor::new();
        // Just test creation
    }

    #[test]
    fn test_output_formatter() {
        let formatter = DefaultOutputFormatter::new();
        let config = HeadlessConfig::default();
        let output = formatter.format("test", &config);
        assert_eq!(output, "test");
    }
}
