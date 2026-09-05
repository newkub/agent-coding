mod prompts;

#[cfg(test)]
mod tests;

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

use crate::modules::subagents::domain::models::subagent::{SubagentTask, TaskStatus};
use crate::modules::subagents::ports::SubagentTaskExecutor;
use crate::shared::kernel::result::AppError;

/// Production subagent task executor backed by the OpenAI Chat Completions API.
///
/// The executor reads `OPENAI_API_KEY` from the environment lazily on first
/// use and caches the resulting [`Client`] in an [`Arc`]. Tasks that fail
/// because the key is missing or the API returns an error are marked as
/// `Failed` with the underlying message stored on the task, so callers can
/// surface the failure to the user instead of silently producing synthetic
/// data.
pub(crate) struct DefaultSubagentTaskExecutor {
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
    /// instead of silently producing synthetic output.
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

        let system_prompt = prompts::system_prompt_for(&task.task_type);
        let model = prompts::model_for(&task.task_type);

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

        let response = client.chat().create(request).await.map_err(|e| {
            tracing::error!(task_id = %task.id, error = %e, "openai chat completion failed");
            AppError::State(format!("openai chat completion error: {e}"))
        })?;

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

        for task in tasks.iter_mut() {
            let task_id = task.id.clone();
            let mut task = task.clone();
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
