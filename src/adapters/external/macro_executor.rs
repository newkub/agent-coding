use async_trait::async_trait;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::modules::macros::application::usecases::StepResult;
use crate::modules::macros::domain::models::MacroStep;
use crate::modules::macros::ports::MacroExecutor;
use crate::shared::kernel::result::AppResult;

/// In-memory macro executor.
///
/// `Input`, `KeyCombo` and `Wait` steps are simulated locally; `Command`
/// steps run through the host shell, matching how the Terminal/CLI tabs
/// execute commands.
pub(crate) struct InMemoryMacroExecutor {
    step_counter: AtomicUsize,
}

impl InMemoryMacroExecutor {
    pub(crate) const fn new() -> Self {
        Self {
            step_counter: AtomicUsize::new(0),
        }
    }

    fn next_index(&self) -> usize {
        self.step_counter.fetch_add(1, Ordering::Relaxed)
    }

    fn ok(&self, output: String) -> AppResult<StepResult> {
        Ok(StepResult {
            step_index: self.next_index(),
            success: true,
            output: Some(output),
            error: None,
        })
    }
}

impl Default for InMemoryMacroExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MacroExecutor for InMemoryMacroExecutor {
    async fn execute_step(&self, step: &MacroStep) -> AppResult<StepResult> {
        match step {
            MacroStep::Input { text } => self.ok(format!("input: {text}")),
            MacroStep::KeyCombo { keys } => self.ok(format!("keys: {}", keys.join("+"))),
            MacroStep::Wait { millis } => {
                // Bound the wait so playback cannot stall the UI.
                tokio::time::sleep(std::time::Duration::from_millis((*millis).min(500))).await;
                self.ok(format!("waited {millis}ms"))
            }
            MacroStep::Command { cmd, cwd } => {
                let step_index = self.next_index();
                let mut command = if cfg!(windows) {
                    let mut c = tokio::process::Command::new("cmd");
                    c.args(["/C", cmd]);
                    c
                } else {
                    let mut c = tokio::process::Command::new("sh");
                    c.args(["-c", cmd]);
                    c
                };
                if let Some(dir) = cwd {
                    command.current_dir(dir);
                }

                match command.output().await {
                    Ok(out) => Ok(StepResult {
                        step_index,
                        success: out.status.success(),
                        output: Some(String::from_utf8_lossy(&out.stdout).to_string()),
                        error: if out.status.success() {
                            None
                        } else {
                            Some(String::from_utf8_lossy(&out.stderr).to_string())
                        },
                    }),
                    Err(e) => Ok(StepResult {
                        step_index,
                        success: false,
                        output: None,
                        error: Some(e.to_string()),
                    }),
                }
            }
        }
    }

    async fn can_execute(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_input_step() {
        let executor = InMemoryMacroExecutor::new();
        let result = executor
            .execute_step(&MacroStep::Input {
                text: "hello".to_string(),
            })
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.output.as_deref(), Some("input: hello"));
    }

    #[tokio::test]
    async fn test_execute_wait_step() {
        let executor = InMemoryMacroExecutor::new();
        let result = executor
            .execute_step(&MacroStep::Wait { millis: 1 })
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_execute_command_step() {
        let executor = InMemoryMacroExecutor::new();
        let result = executor
            .execute_step(&MacroStep::Command {
                cmd: "echo hi".to_string(),
                cwd: None,
            })
            .await
            .unwrap();
        assert!(result.success);
    }
}
