use crate::modules::snippet::domain::events::SnippetEvent;
use crate::modules::snippet::domain::models::{Snippet, SnippetId};
use crate::shared::kernel::result::AppResult;
use async_trait::async_trait;

/// Port: Snippet Repository
#[async_trait]
pub(crate) trait SnippetRepository: Send + Sync {
    async fn save(&self, snippet: &Snippet) -> AppResult<()>;
    async fn find_by_id(&self, id: &SnippetId) -> AppResult<Option<Snippet>>;
    async fn find_all(&self) -> AppResult<Vec<Snippet>>;
    async fn delete(&self, id: &SnippetId) -> AppResult<()>;
    async fn search(&self, query: &str) -> AppResult<Vec<Snippet>>;
    async fn find_by_language(&self, language: &str) -> AppResult<Vec<Snippet>>;
    async fn find_by_tag(&self, tag: &str) -> AppResult<Vec<Snippet>>;
}

/// Port: Snippet Event Publisher
#[async_trait]
pub(crate) trait SnippetEventPublisher: Send + Sync {
    async fn publish(&self, event: SnippetEvent) -> AppResult<()>;
}
