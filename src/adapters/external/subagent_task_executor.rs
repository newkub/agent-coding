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
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::subagents::domain::models::subagent::{SubagentTask, TaskStatus, TaskType};
use crate::modules::subagents::ports::SubagentTaskExecutor;
use crate::shared::kernel::result::AppError;

/// Build the system prompt that scopes the model to a specific task type.
fn system_prompt_for(task_type: &TaskType) -> String {
    let base = "You are a senior software engineer working as a specialized subagent. \
        Respond with concrete, actionable findings only — no preamble, no hedging.";
    let role = match task_type {
        TaskType::CodeReview => {
            "Focus on correctness, maintainability, and test coverage. \
            List concrete issues with file/line references and suggested fixes."
        }
        TaskType::BugDetection => {
            "Identify likely bugs, race conditions, and edge cases. \
            For each finding, give the location, the failure mode, and a minimal fix."
        }
        TaskType::Refactoring => {
            "Propose refactorings that preserve behaviour. \
            Prefer small, mechanical steps and cite the design pattern or principle applied."
        }
        TaskType::Documentation => {
            "Produce documentation matching the surrounding style. \
            Include purpose, parameters, return values, errors, and at least one example."
        }
        TaskType::TestGeneration => {
            "Generate tests covering happy paths, edge cases, and error paths. \
            Use the project's existing test framework and naming conventions."
        }
        TaskType::SecurityAudit => {
            "Enumerate security risks with severity, attack vector, and remediation. \
            Reference OWASP categories where applicable."
        }
        TaskType::PerformanceAnalysis => {
            "Identify hot paths and complexity bottlenecks. \
            Quantify the expected improvement and propose a measurement plan."
        }
        TaskType::DependencyUpdate => {
            "List outdated or vulnerable dependencies, the target version, \
            and any breaking changes that affect this codebase."
        }
        TaskType::Custom(_) => "Complete the requested task precisely and concisely.",
    };
    format!("{base}\n\n{role}")
}

/// Map a `TaskType` to the model name used for execution.
///
/// Defaults to `gpt-4o-mini` for cost efficiency; callers can override via the
/// `SUBAGENT_MODEL` environment variable.
fn model_for(_task_type: &TaskType) -> String {
    env::var("SUBAGENT_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string())
}

/// Production subagent task executor backed by the OpenAI Chat Completions API.
///
/// The executor reads `OPENAI_API_KEY` from the environment lazily on first
/// use and caches the resulting [`Client`] in an [`Arc`]. Tasks that fail
/// because the key is missing or the API returns an error are marked as
/// `Failed` with the underlying message stored on the task, so callers can
/// surface the failure to the user instead of silently falling back to mock
/// data.
pub struct DefaultSubagentTaskExecutor {
    tasks: Arc<RwLock<HashMap<String, SubagentTask>>>,
    client: Arc<RwLock<Option<Client<OpenAIConfig>>>>,
}

impl DefaultSubagentTaskExecutor {
    pub(crate) fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            client: Arc::new(RwLock::new(None)),
        }
    }

    /// Lazily build the OpenAI client from `OPENAI_API_KEY`.
    ///
    /// Returns an error if the key is not configured so callers can fail fast
    /// instead of silently producing placeholder output.
    async fn client(&self) -> Result<Client<OpenAIConfig>, AppError> {
        // Fast path: cached client.
        {
            let cache = self.client.read().await;
            if let Some(client) = cache.as_ref() {
                return Ok(client.clone());
            }
        }

        let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
            AppError::State(
                "OPENAI_API_KEY is not set; cannot execute subagent tasks against the OpenAI API"
                    .to_string(),
            )
        })?;

        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);

        let mut cache = self.client.write().await;
        *cache = Some(client.clone());
        Ok(client)
    }

    /// Call the OpenAI Chat Completions API for a single task.
    async fn generate(&self, task: &SubagentTask) -> Result<String, AppError> {
        let client = self.client().await?;

        let system_prompt = system_prompt_for(&task.task_type);
        let model = model_for(&task.task_type);

        let request = CreateChatCompletionRequestArgs::default()
            .model(model.as_str())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()
                    .map_err(|e| {
                        AppError::State(format!("openai system message build error: {e}"))
                    })?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(task.input.as_str())
                    .build()
                    .map_err(|e| AppError::State(format!("openai user message build error: {e}")))?
                    .into(),
            ])
            .build()
            .map_err(|e| AppError::State(format!("openai request build error: {e}")))?;

        let response = client
            .chat()
            .create(request)
            .await
            .map_err(|e| AppError::State(format!("openai chat completion error: {e}")))?;

        let content = response
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .ok_or_else(|| AppError::State("openai returned an empty completion".to_string()))?;

        Ok(content)
    }
}

impl Default for DefaultSubagentTaskExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SubagentTaskExecutor for DefaultSubagentTaskExecutor {
    async fn execute_task(&self, task: &mut SubagentTask) -> Result<(), AppError> {
        // Update status to processing.
        task.status = TaskStatus::Processing;

        // Store the in-flight task so callers can observe it.
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task.id.clone(), task.clone());
        }

        // Execute the task against the real AI provider. On failure we record
        // the error on the task and propagate it so the caller can decide
        // whether to retry.
        match self.generate(task).await {
            Ok(output) => task.complete(output),
            Err(err) => {
                task.fail(err.to_string());
                // Update the stored task with the failure state before returning.
                let mut tasks = self.tasks.write().await;
                tasks.insert(task.id.clone(), task.clone());
                return Err(err);
            }
        }

        // Persist the completed task.
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task.clone());

        Ok(())
    }

    async fn execute_tasks_parallel(&self, tasks: &mut [SubagentTask]) -> Result<(), AppError> {
        let mut handles = Vec::new();

        for i in 0..tasks.len() {
            let task_id = tasks[i].id.clone();
            let mut task = tasks[i].clone();
            let executor = self.clone();

            let handle = tokio::spawn(async move {
                if let Err(e) = executor.execute_task(&mut task).await {
                    Err(e)
                } else {
                    Ok(task)
                }
            });
            handles.push((task_id, handle));
        }

        for (task_id, handle) in handles {
            let task = handle
                .await
                .map_err(|e| AppError::State(format!("task execution error: {e}")))??;
            let mut tasks = self.tasks.write().await;
            tasks.insert(task_id, task);
        }

        Ok(())
    }

    async fn execute_tasks_sequential(&self, tasks: &mut [SubagentTask]) -> Result<(), AppError> {
        for task in tasks {
            self.execute_task(task).await?;
        }
        Ok(())
    }

    async fn cancel_task(&self, task_id: &str) -> Result<(), AppError> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = TaskStatus::Cancelled;
            task.completed_at = Some(chrono::Utc::now());
            Ok(())
        } else {
            Err(AppError::NotFound(format!("task {task_id} not found")))
        }
    }

    async fn get_task_status(&self, task_id: &str) -> Result<SubagentTask, AppError> {
        let tasks = self.tasks.read().await;
        tasks
            .get(task_id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("task {task_id} not found")))
    }
}

impl Clone for DefaultSubagentTaskExecutor {
    fn clone(&self) -> Self {
        Self {
            tasks: Arc::clone(&self.tasks),
            client: Arc::clone(&self.client),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::subagents::domain::models::subagent::{TaskContext, TaskType};

    #[tokio::test]
    async fn test_cancel_task() {
        let executor = DefaultSubagentTaskExecutor::new();
        let context = TaskContext::new();
        let task = SubagentTask::new(
            "agent-1".to_string(),
            TaskType::CodeReview,
            "Review this code".to_string(),
            context,
        );

        let mut tasks = executor.tasks.write().await;
        let task_id = task.id.clone();
        tasks.insert(task_id.clone(), task);
        drop(tasks);

        let result = executor.cancel_task(&task_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_task_without_api_key_fails() {
        // Ensure no key is present so the executor fails fast instead of
        // producing placeholder output.
        env::remove_var("OPENAI_API_KEY");
        let executor = DefaultSubagentTaskExecutor::new();
        let context = TaskContext::new();
        let mut task = SubagentTask::new(
            "agent-1".to_string(),
            TaskType::CodeReview,
            "Review this code".to_string(),
            context,
        );

        let result = executor.execute_task(&mut task).await;
        assert!(result.is_err(), "executor must fail without an API key");
        assert_eq!(task.status, TaskStatus::Failed);
        assert!(task.error.is_some());
    }
}
