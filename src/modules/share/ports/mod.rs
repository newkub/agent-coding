use crate::modules::session::domain::models::Session;
use crate::modules::share::domain::models::share_link::ShareLink;
use crate::shared::kernel::result::AppError;
use crate::shared::kernel::result::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

/// Port: Share Repository (for session export/import)
#[async_trait]
pub(crate) trait ShareRepository: Send + Sync {
    async fn save(&self, session: &Session) -> AppResult<()>;
    async fn find_by_name(&self, name: &str) -> AppResult<Option<Session>>;
    async fn export_to_json(&self, session_id: &str) -> AppResult<String>;
    async fn import_from_json(&self, json: &str) -> AppResult<Session>;
}

/// Port for share link repository operations
#[async_trait]
pub trait ShareLinkRepository: Send + Sync {
    /// Save a new share link
    async fn save(&self, link: &ShareLink) -> Result<(), AppError>;

    /// Find share link by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ShareLink>, AppError>;

    /// Find share link by token
    async fn find_by_token(&self, token: &str) -> Result<Option<ShareLink>, AppError>;

    /// Find all share links for a session
    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<ShareLink>, AppError>;

    /// Update share link
    async fn update(&self, link: &ShareLink) -> Result<(), AppError>;

    /// Delete share link
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;

    /// Deactivate all share links for a session
    async fn deactivate_by_session_id(&self, session_id: Uuid) -> Result<(), AppError>;
}

/// Port for share link URL generation
#[async_trait]
pub trait ShareLinkUrlGenerator: Send + Sync {
    /// Generate a shareable URL for a share link
    async fn generate_url(&self, token: &str) -> Result<String, AppError>;

    /// Validate a share URL and extract token
    async fn validate_url(&self, url: &str) -> Result<Option<String>, AppError>;
}

/// Port for share link notification
#[async_trait]
pub trait ShareLinkNotifier: Send + Sync {
    /// Notify when a share link is created
    async fn notify_created(&self, link: &ShareLink, url: &str) -> Result<(), AppError>;

    /// Notify when a share link is accessed
    async fn notify_accessed(&self, link: &ShareLink) -> Result<(), AppError>;

    /// Notify when a share link is deactivated
    async fn notify_deactivated(&self, link: &ShareLink) -> Result<(), AppError>;
}
