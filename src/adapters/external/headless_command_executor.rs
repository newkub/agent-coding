use async_openai::{
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::time::timeout;

use crate::adapters::config::loader::ConfigLoader;
use crate::modules::headless::domain::models::command::{
    CommandStatus, CommandType, HeadlessCommand, HeadlessConfig,
};
use crate::modules::headless::domain::operations::command_operations::{
    extract_arguments, format_output, truncate_output,
};
use crate::modules::headless::domain::validators::command_validators;
use crate::modules::headless::ports::{HeadlessCommandExecutor, OutputFormatter};
use crate::shared::kernel::result::AppError;

const SHELL_COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

/// Production implementation for headless command execution.
///
/// File commands operate inside the configured working directory, shell
/// commands are executed by the platform shell with a timeout, and chat commands
/// call the configured OpenAI-compatible chat completions endpoint.
pub(crate) struct DefaultHeadlessCommandExecutor {
    formatter: DefaultOutputFormatter,
    commands: Arc<RwLock<HashMap<String, HeadlessCommand>>>,
}

impl DefaultHeadlessCommandExecutor {
    pub(crate) fn new() -> Self {
        Self {
            formatter: DefaultOutputFormatter::new(),
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn execute_output(&self, command: &HeadlessCommand) -> Result<String, AppError> {
        match command.command_type {
            CommandType::Chat => self.execute_chat(command).await,
            CommandType::FileRead => self.read_file(command).await,
            CommandType::FileWrite => self.write_file(command).await,
            CommandType::CommandExecute => self.execute_shell(command).await,
            CommandType::SessionList | CommandType::SessionCreate | CommandType::SessionLoad => {
                Err(AppError::State(
                    "session commands must be handled by the headless session manager".to_string(),
                ))
            }
            CommandType::Help => Ok(self.generate_help_text()),
            CommandType::Exit => Ok("Exiting...".to_string()),
        }
    }

    async fn execute_chat(&self, command: &HeadlessCommand) -> Result<String, AppError> {
        let prompt = command_payload(&command.input);
        if prompt.is_empty() {
            return Err(AppError::ValidationError(
                "chat command requires a message".to_string(),
            ));
        }

        let settings = ConfigLoader::new()
            .load()
            .map_err(|e| AppError::State(format!("failed to load AI settings: {e}")))?;
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
            AppError::State("OPENAI_API_KEY is not set; cannot execute headless chat".to_string())
        })?;
        let model = env::var("AGENT_TUI_OPENAI_MODEL")
            .unwrap_or_else(|_| settings.ai.default_model.clone());
        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(settings.ai.api_endpoint);
        let client = Client::with_config(config);

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(
                        "You are a concise terminal AI assistant. Answer directly and clearly.",
                    )
                    .build()
                    .map_err(|e| {
                        AppError::State(format!("openai system message build error: {e}"))
                    })?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt)
                    .build()
                    .map_err(|e| AppError::State(format!("openai user message build error: {e}")))?
                    .into(),
            ])
            .build()
            .map_err(|e| AppError::State(format!("openai request build error: {e}")))?;

        let response = client.chat().create(request).await.map_err(|e| {
            tracing::error!(command_id = %command.id, error = %e, "headless chat completion failed");
            AppError::State(format!("openai chat completion error: {e}"))
        })?;

        response
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .ok_or_else(|| AppError::State("openai returned an empty completion".to_string()))
    }

    async fn read_file(&self, command: &HeadlessCommand) -> Result<String, AppError> {
        let args = extract_arguments(&command.input);
        let Some(path_arg) = args.first() else {
            return Err(AppError::ValidationError(
                "read command requires a file path".to_string(),
            ));
        };
        let path = resolve_existing_file(&command.context.working_directory, path_arg)?;
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| AppError::Io(format!("failed to read {}: {e}", path.display())))?;
        Ok(content)
    }

    async fn write_file(&self, command: &HeadlessCommand) -> Result<String, AppError> {
        let args = extract_arguments(&command.input);
        let Some(path_arg) = args.first() else {
            return Err(AppError::ValidationError(
                "write command requires a file path".to_string(),
            ));
        };
        if args.len() < 2 {
            return Err(AppError::ValidationError(
                "write command requires content".to_string(),
            ));
        }

        let path = resolve_output_file(&command.context.working_directory, path_arg)?;
        let content = args[1..].join(" ");
        tokio::fs::write(&path, content)
            .await
            .map_err(|e| AppError::Io(format!("failed to write {}: {e}", path.display())))?;
        Ok(format!("File written: {}", path.display()))
    }

    async fn execute_shell(&self, command: &HeadlessCommand) -> Result<String, AppError> {
        let shell_command = command_payload(&command.input);
        if shell_command.is_empty() {
            return Err(AppError::ValidationError(
                "exec command requires a shell command".to_string(),
            ));
        }

        let working_dir = working_directory_root(&command.context.working_directory)?;
        let mut process = if cfg!(windows) {
            let mut process = Command::new("cmd");
            process.arg("/C").arg(&shell_command);
            process
        } else {
            let mut process = Command::new("sh");
            process.arg("-c").arg(&shell_command);
            process
        };

        process
            .current_dir(&working_dir)
            .envs(&command.context.environment)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = process
            .spawn()
            .map_err(|e| AppError::Io(format!("failed to spawn command '{shell_command}': {e}")))?;

        let mut stdout_pipe = child
            .stdout
            .take()
            .ok_or_else(|| AppError::Io("failed to capture command stdout".to_string()))?;
        let mut stderr_pipe = child
            .stderr
            .take()
            .ok_or_else(|| AppError::Io("failed to capture command stderr".to_string()))?;

        let stdout_task = tokio::spawn(async move {
            let mut output = Vec::new();
            stdout_pipe.read_to_end(&mut output).await.map(|_| output)
        });
        let stderr_task = tokio::spawn(async move {
            let mut output = Vec::new();
            stderr_pipe.read_to_end(&mut output).await.map(|_| output)
        });

        let status = match timeout(SHELL_COMMAND_TIMEOUT, child.wait()).await {
            Ok(status) => status
                .map_err(|e| AppError::Io(format!("failed while waiting for command: {e}")))?,
            Err(_) => {
                child
                    .kill()
                    .await
                    .map_err(|e| AppError::Io(format!("failed to kill timed-out command: {e}")))?;
                let _ = child.wait().await;
                let _ = stdout_task.await;
                let _ = stderr_task.await;
                return Err(AppError::State(format!(
                    "command timed out after {} seconds",
                    SHELL_COMMAND_TIMEOUT.as_secs()
                )));
            }
        };

        let stdout = stdout_task
            .await
            .map_err(|e| AppError::Io(format!("failed to read command stdout: {e}")))?
            .map_err(|e| AppError::Io(format!("failed to read command stdout: {e}")))?;
        let stderr = stderr_task
            .await
            .map_err(|e| AppError::Io(format!("failed to read command stderr: {e}")))?
            .map_err(|e| AppError::Io(format!("failed to read command stderr: {e}")))?;

        let stdout_text = String::from_utf8_lossy(&stdout).trim().to_string();
        let stderr_text = String::from_utf8_lossy(&stderr).trim().to_string();

        if !status.success() {
            let detail = if stderr_text.is_empty() {
                stdout_text
            } else {
                stderr_text
            };
            return Err(AppError::State(format!(
                "command failed with status {status}: {detail}"
            )));
        }

        if stderr_text.is_empty() {
            Ok(stdout_text)
        } else if stdout_text.is_empty() {
            Ok(stderr_text)
        } else {
            Ok(format!("{stdout_text}\n{stderr_text}"))
        }
    }

    fn generate_help_text(&self) -> String {
        r#"
Agent-TUI Headless Mode Commands:

/chat <message>        - Send a chat message to the AI
/read <file>           - Read a file inside the working directory
/write <file> <text>   - Write text to a file inside the working directory
/exec <command>        - Execute a shell command in the working directory
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
        command_validators::validate_command_for_headless(command)?;

        command.status = CommandStatus::Processing;
        {
            let mut commands = self.commands.write().await;
            commands.insert(command.id.clone(), command.clone());
        }

        let output = match self.execute_output(command).await {
            Ok(output) => output,
            Err(error) => {
                command.fail(error.to_string());
                let mut commands = self.commands.write().await;
                commands.insert(command.id.clone(), command.clone());
                return Err(error);
            }
        };

        let formatted_output = self.formatter.format(&output, config);
        let final_output = if let Some(max_length) = config.max_output_length {
            truncate_output(&formatted_output, max_length)
        } else {
            formatted_output
        };

        command.complete(final_output);
        let mut commands = self.commands.write().await;
        commands.insert(command.id.clone(), command.clone());
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

    async fn get_status(&self, command_id: &str) -> Result<HeadlessCommand, AppError> {
        let commands = self.commands.read().await;
        commands
            .get(command_id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("command {command_id} not found")))
    }
}

/// Default output formatter
pub(crate) struct DefaultOutputFormatter;

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
            println!("{}", output);
        }
        Ok(())
    }
}

fn command_payload(input: &str) -> String {
    let trimmed = input.trim();
    if let Some(command) = trimmed.strip_prefix('!') {
        return command.trim().to_string();
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or_default();
    let known_command = first.starts_with('/')
        || matches!(
            first.to_ascii_lowercase().as_str(),
            "chat" | "read" | "write" | "exec"
        );

    if known_command {
        parts.next().unwrap_or_default().trim().to_string()
    } else {
        trimmed.to_string()
    }
}

fn working_directory_root(working_directory: &str) -> Result<PathBuf, AppError> {
    std::fs::canonicalize(working_directory).map_err(|e| {
        AppError::Io(format!(
            "failed to resolve working directory '{working_directory}': {e}"
        ))
    })
}

fn resolve_existing_file(working_directory: &str, path_arg: &str) -> Result<PathBuf, AppError> {
    let root = working_directory_root(working_directory)?;
    let candidate = candidate_path(&root, path_arg);
    let path = std::fs::canonicalize(&candidate).map_err(|e| {
        AppError::NotFound(format!(
            "file '{}' does not exist or cannot be resolved: {e}",
            candidate.display()
        ))
    })?;

    if !path.starts_with(&root) {
        return Err(AppError::PermissionDenied(format!(
            "file '{}' is outside the working directory",
            path.display()
        )));
    }

    if !path.is_file() {
        return Err(AppError::ValidationError(format!(
            "'{}' is not a file",
            path.display()
        )));
    }

    Ok(path)
}

fn resolve_output_file(working_directory: &str, path_arg: &str) -> Result<PathBuf, AppError> {
    let root = working_directory_root(working_directory)?;
    let candidate = candidate_path(&root, path_arg);
    let parent = candidate.parent().ok_or_else(|| {
        AppError::ValidationError(format!("invalid output file path '{path_arg}'"))
    })?;
    let canonical_parent = std::fs::canonicalize(parent).map_err(|e| {
        AppError::NotFound(format!(
            "output directory '{}' does not exist or cannot be resolved: {e}",
            parent.display()
        ))
    })?;

    if !canonical_parent.starts_with(&root) {
        return Err(AppError::PermissionDenied(format!(
            "file '{}' is outside the working directory",
            candidate.display()
        )));
    }

    let file_name = candidate.file_name().ok_or_else(|| {
        AppError::ValidationError(format!("invalid output file path '{path_arg}'"))
    })?;
    Ok(canonical_parent.join(file_name))
}

fn candidate_path(root: &Path, path_arg: &str) -> PathBuf {
    let path = Path::new(path_arg);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::headless::domain::models::command::CommandContext;

    fn test_context() -> CommandContext {
        #[cfg(windows)]
        let path = std::env::current_dir().unwrap();
        #[cfg(not(windows))]
        let path = std::env::temp_dir();
        CommandContext::new(path.to_string_lossy().to_string())
    }

    #[test]
    fn test_headless_executor_creation() {
        let _executor = DefaultHeadlessCommandExecutor::new();
    }

    #[test]
    fn test_output_formatter() {
        let formatter = DefaultOutputFormatter::new();
        let config = HeadlessConfig::default();
        let output = formatter.format("test", &config);
        assert_eq!(output, "test");
    }

    #[test]
    fn test_command_payload() {
        assert_eq!(command_payload("/exec echo hello"), "echo hello");
        assert_eq!(command_payload("exec echo hello"), "echo hello");
        assert_eq!(command_payload("!echo hello"), "echo hello");
        assert_eq!(command_payload("plain prompt"), "plain prompt");
    }

    #[tokio::test]
    async fn test_file_write_and_read_inside_working_directory() {
        let executor = DefaultHeadlessCommandExecutor::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("note.txt");
        let context = CommandContext::new(temp_dir.path().to_string_lossy().to_string());
        let config = HeadlessConfig::default();

        let mut write = HeadlessCommand::new(
            CommandType::FileWrite,
            format!("/write {} hello world", file_path.display()),
            context.clone(),
        );
        executor.execute(&mut write, &config).await.unwrap();
        assert_eq!(write.status, CommandStatus::Completed);
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "hello world");

        let mut read = HeadlessCommand::new(
            CommandType::FileRead,
            format!("/read {}", file_path.display()),
            context,
        );
        executor.execute(&mut read, &config).await.unwrap();
        assert_eq!(read.output.as_deref(), Some("hello world"));
    }

    #[tokio::test]
    async fn test_shell_execute() {
        let executor = DefaultHeadlessCommandExecutor::new();
        let context = test_context();
        let config = HeadlessConfig::default();
        let input = if cfg!(windows) {
            "/exec echo hello"
        } else {
            "/exec printf hello"
        };
        let mut command =
            HeadlessCommand::new(CommandType::CommandExecute, input.to_string(), context);
        executor.execute(&mut command, &config).await.unwrap();
        assert_eq!(command.status, CommandStatus::Completed);
        assert_eq!(command.output.as_deref(), Some("hello"));
    }
}
