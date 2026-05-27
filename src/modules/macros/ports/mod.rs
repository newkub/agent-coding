use async_trait::async_trait;
use crate::modules::macros::domain::{models::{Macro, MacroId, MacroStep}, events::MacroEvent};
use crate::shared::kernel::result::AppResult;
use crate::modules::macros::application::usecases::StepResult;

/// Port: Macro Repository
#[async_trait]
pub(crate) trait MacroRepository: Send + Sync {
    async fn save(&self, macro_def: &Macro) -> AppResult<()>;
    async fn save_recording(&self, macro_def: &Macro) -> AppResult<()>;
    async fn finish_recording(&self, id: &MacroId) -> AppResult<Option<Macro>>;
    async fn find_by_id(&self, id: &MacroId) -> AppResult<Option<Macro>>;
    async fn find_all(&self) -> AppResult<Vec<Macro>>;
    async fn delete(&self, id: &MacroId) -> AppResult<()>;
}

/// Port: Macro Executor
#[async_trait]
pub(crate) trait MacroExecutor: Send + Sync {
    async fn execute_step(&self, step: &MacroStep) -> AppResult<StepResult>;
    async fn can_execute(&self) -> bool;
}

/// Port: Macro Event Publisher
#[async_trait]
pub(crate) trait MacroEventPublisher: Send + Sync {
    async fn publish(&self, event: MacroEvent) -> AppResult<()>;
}