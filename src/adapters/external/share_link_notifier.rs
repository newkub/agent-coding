use async_trait::async_trait;
use tracing::info;

use crate::modules::share::domain::models::share_link::ShareLink;
use crate::modules::share::ports::ShareLinkNotifier;
use crate::shared::kernel::result::AppError;

/// Default implementation for share link notifications (logs to console)
pub(crate) struct LogShareLinkNotifier;

impl LogShareLinkNotifier {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl Default for LogShareLinkNotifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ShareLinkNotifier for LogShareLinkNotifier {
    async fn notify_created(&self, link: &ShareLink, url: &str) -> Result<(), AppError> {
        info!(
            "Share link created: ID={}, Session={}, Token={}, URL={}",
            link.id, link.session_id, link.token, url
        );
        Ok(())
    }

    async fn notify_accessed(&self, link: &ShareLink) -> Result<(), AppError> {
        info!(
            "Share link accessed: ID={}, Token={}, Access Count={}/{}",
            link.id,
            link.token,
            link.access_count,
            link.max_access.map_or("∞".to_string(), |m| m.to_string())
        );
        Ok(())
    }

    async fn notify_deactivated(&self, link: &ShareLink) -> Result<(), AppError> {
        info!(
            "Share link deactivated: ID={}, Token={}",
            link.id, link.token
        );
        Ok(())
    }
}

/// No-op implementation for testing or when notifications are disabled
pub(crate) struct NoopShareLinkNotifier;

#[async_trait]
impl ShareLinkNotifier for NoopShareLinkNotifier {
    async fn notify_created(&self, _link: &ShareLink, _url: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn notify_accessed(&self, _link: &ShareLink) -> Result<(), AppError> {
        Ok(())
    }

    async fn notify_deactivated(&self, _link: &ShareLink) -> Result<(), AppError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_log_notifier_created() {
        let notifier = LogShareLinkNotifier::new();
        let link = ShareLink::new(uuid::Uuid::new_v4(), Some(24), Some(10));
        let result = notifier
            .notify_created(&link, "https://example.com/test")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_noop_notifier() {
        let notifier = NoopShareLinkNotifier;
        let link = ShareLink::new(uuid::Uuid::new_v4(), Some(24), Some(10));
        assert!(notifier.notify_created(&link, "").await.is_ok());
        assert!(notifier.notify_accessed(&link).await.is_ok());
        assert!(notifier.notify_deactivated(&link).await.is_ok());
    }
}
