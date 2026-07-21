use crate::modules::snippet::domain::models::{Snippet, SnippetId};
use crate::modules::snippet::ports::{SnippetEventPublisher, SnippetRepository};
use crate::shared::kernel::result::AppResult;

/// Use case: Create new snippet
pub(crate) async fn create_snippet<R, E>(
    repo: &R,
    publisher: &E,
    name: String,
    description: String,
    code: String,
    language: String,
) -> AppResult<Snippet>
where
    R: SnippetRepository,
    E: SnippetEventPublisher,
{
    let snippet = crate::modules::snippet::application::services::create_snippet(
        name,
        description,
        code,
        language,
    )?;

    repo.save(&snippet).await?;
    publisher
        .publish(
            crate::modules::snippet::domain::events::SnippetEvent::Created {
                snippet_id: snippet.id.0.clone(),
            },
        )
        .await?;

    Ok(snippet)
}

/// Use case: Search snippets
pub(crate) async fn search_snippets<R>(repo: &R, query: &str) -> AppResult<Vec<Snippet>>
where
    R: SnippetRepository,
{
    repo.search(query).await
}

/// Use case: Delete snippet
pub(crate) async fn delete_snippet<R, E>(repo: &R, publisher: &E, id: &SnippetId) -> AppResult<()>
where
    R: SnippetRepository,
    E: SnippetEventPublisher,
{
    repo.delete(id).await?;
    publisher
        .publish(
            crate::modules::snippet::domain::events::SnippetEvent::Deleted {
                snippet_id: id.0.clone(),
            },
        )
        .await?;
    Ok(())
}

/// Use case: Update snippet
pub(crate) async fn update_snippet<R, E>(
    repo: &R,
    publisher: &E,
    snippet: Snippet,
) -> AppResult<Snippet>
where
    R: SnippetRepository,
    E: SnippetEventPublisher,
{
    repo.save(&snippet).await?;
    publisher
        .publish(
            crate::modules::snippet::domain::events::SnippetEvent::Updated {
                snippet_id: snippet.id.0.clone(),
            },
        )
        .await?;
    Ok(snippet)
}
