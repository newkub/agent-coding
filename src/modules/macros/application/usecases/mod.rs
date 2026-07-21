use crate::modules::macros::domain::models::{Macro, MacroContext, MacroId, MacroStep};
use crate::modules::macros::ports::{MacroExecutor, MacroRepository};
use crate::shared::kernel::result::AppResult;

/// Use case: Start recording a macro
pub(crate) async fn start_recording<R>(
    repo: &R,
    name: String,
    description: String,
) -> AppResult<MacroId>
where
    R: MacroRepository,
{
    let id = crate::modules::macros::domain::models::MacroId::from_string(
        uuid::Uuid::new_v4().to_string(),
    );
    let now = chrono::Utc::now();
    let macro_def = Macro::create(id, name, description, now);
    repo.save_recording(&macro_def).await?;
    Ok(macro_def.id)
}

/// Use case: Stop recording and save macro
pub(crate) async fn stop_recording<R>(repo: &R, id: &MacroId) -> AppResult<Option<Macro>>
where
    R: MacroRepository,
{
    repo.finish_recording(id).await
}

/// Use case: Playback a macro
pub(crate) async fn playback_macro<E>(
    executor: &E,
    macro_def: &Macro,
    context: Option<MacroContext>,
) -> AppResult<PlaybackResult>
where
    E: MacroExecutor,
{
    let ctx = context.unwrap_or_default();
    let mut results = Vec::new();

    for step in &macro_def.steps {
        let resolved_step = resolve_step(step, &ctx);
        let result = executor.execute_step(&resolved_step).await?;
        results.push(result);
    }

    let success = results.iter().all(|r| r.success);

    Ok(PlaybackResult {
        macro_id: macro_def.id.clone(),
        step_results: results,
        success,
    })
}

/// Use case: Delete a macro
pub(crate) async fn delete_macro<R>(repo: &R, id: &MacroId) -> AppResult<()>
where
    R: MacroRepository,
{
    repo.delete(id).await
}

/// Use case: List all macros
pub(crate) async fn list_macros<R>(repo: &R) -> AppResult<Vec<Macro>>
where
    R: MacroRepository,
{
    repo.find_all().await
}

fn resolve_step(step: &MacroStep, ctx: &MacroContext) -> MacroStep {
    match step {
        MacroStep::Command { cmd, cwd } => MacroStep::Command {
            cmd: ctx.resolve(cmd),
            cwd: cwd.clone(),
        },
        other => other.clone(),
    }
}

#[derive(Debug)]
pub(crate) struct PlaybackResult {
    pub macro_id: MacroId,
    pub step_results: Vec<StepResult>,
    pub success: bool,
}

#[derive(Debug)]
pub(crate) struct StepResult {
    pub step_index: usize,
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}
