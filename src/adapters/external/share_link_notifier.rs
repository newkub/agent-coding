use async_trait::async_trait;
use reqwest::Client;
use tracing::{info, warn};

use crate::adapters::external::http_retry::send_with_retry;
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
    async fn notify_created(&self, link: &ShareLink, _url: &str) -> Result<(), AppError> {
        info!(
            share_link_id = %link.id,
            session_id = %link.session_id,
            "share link created"
        );
        Ok(())
    }

    async fn notify_accessed(&self, link: &ShareLink) -> Result<(), AppError> {
        info!(
            share_link_id = %link.id,
            access_count = link.access_count,
            max_access = link.max_access,
            "share link accessed"
        );
        Ok(())
    }

    async fn notify_deactivated(&self, link: &ShareLink) -> Result<(), AppError> {
        info!(share_link_id = %link.id, "share link deactivated");
        Ok(())
    }
}

/// Share-link notifier backed by a user-configured webhook.
///
/// `AGENT_TUI_SHARE_WEBHOOK_URL` enables real notifications. If it is absent,
/// notifications are explicitly disabled and a warning is emitted instead of
/// pretending that a notification was delivered.
pub(crate) struct ConfiguredShareLinkNotifier {
    client: Client,
    webhook_url: Option<String>,
}

impl ConfiguredShareLinkNotifier {
    pub(crate) fn from_env() -> Result<Self, AppError> {
        let webhook_url = std::env::var("AGENT_TUI_SHARE_WEBHOOK_URL")
            .ok()
            .filter(|url| !url.trim().is_empty());

        if let Some(url) = &webhook_url {
            let parsed = reqwest::Url::parse(url).map_err(|e| {
                AppError::ValidationError(format!(
                    "AGENT_TUI_SHARE_WEBHOOK_URL is not a valid URL: {e}"
                ))
            })?;
            if !matches!(parsed.scheme(), "http" | "https") {
                return Err(AppError::ValidationError(
                    "AGENT_TUI_SHARE_WEBHOOK_URL must use http or https".to_string(),
                ));
            }
        } else {
            warn!("AGENT_TUI_SHARE_WEBHOOK_URL is not set; share-link notifications are disabled");
        }

        Ok(Self {
            client: Client::new(),
            webhook_url,
        })
    }

    async fn send(&self, event: &str, link: &ShareLink, url: Option<&str>) -> Result<(), AppError> {
        let Some(webhook_url) = &self.webhook_url else {
            warn!(event, share_link_id = %link.id, "share-link notification skipped");
            return Ok(());
        };

        let payload = serde_json::json!({
            "event": event,
            "share_link_id": link.id,
            "session_id": link.session_id,
            "url": url,
            "access_count": link.access_count,
            "max_access": link.max_access,
        });
        let response = send_with_retry(|| self.client.post(webhook_url).json(&payload)).await?;
        if !response.status().is_success() {
            return Err(AppError::State(format!(
                "share webhook returned status {}",
                response.status()
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl ShareLinkNotifier for ConfiguredShareLinkNotifier {
    async fn notify_created(&self, link: &ShareLink, url: &str) -> Result<(), AppError> {
        self.send("share_link.created", link, Some(url)).await
    }

    async fn notify_accessed(&self, link: &ShareLink) -> Result<(), AppError> {
        self.send("share_link.accessed", link, None).await
    }

    async fn notify_deactivated(&self, link: &ShareLink) -> Result<(), AppError> {
        self.send("share_link.deactivated", link, None).await
    }
}

/// No-op implementation for tests or explicitly disabled notifications
#[allow(dead_code)]
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
