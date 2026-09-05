use crate::modules::ui::domain::models::{AppState, NoteItem, SnippetItem};
use crate::shared::kernel::result::AppResult;
use async_trait::async_trait;

/// Port: UI Renderer
#[async_trait]
pub(crate) trait UIRenderer: Send + Sync {
    async fn render(&mut self, state: &AppState) -> AppResult<()>;
    async fn clear(&mut self) -> AppResult<()>;
}

/// Port: Input Handler
#[async_trait]
pub(crate) trait InputHandler: Send + Sync {
    async fn read_key(&self) -> AppResult<Option<crossterm::event::KeyEvent>>;
}

/// Port: Event Bus
#[async_trait]
pub(crate) trait EventBus: Send + Sync {
    async fn publish(&self, event: crate::modules::ui::domain::events::UIEvent) -> AppResult<()>;
    async fn subscribe(&self) -> AppResult<()>;
}

/// Port: persistent UI-owned content (notes and snippets).
#[async_trait]
pub(crate) trait UiContentRepository: Send + Sync {
    async fn list_notes(&self) -> AppResult<Vec<NoteItem>>;
    async fn replace_notes(&self, notes: &[NoteItem]) -> AppResult<()>;
    async fn list_snippets(&self) -> AppResult<Vec<SnippetItem>>;
    async fn replace_snippets(&self, snippets: &[SnippetItem]) -> AppResult<()>;
}
