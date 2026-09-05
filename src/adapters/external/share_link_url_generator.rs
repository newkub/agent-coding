#![allow(dead_code)]
use crate::modules::share::ports::ShareLinkUrlGenerator;
use crate::shared::kernel::result::AppError;
use async_trait::async_trait;

/// Default implementation for generating share link URLs
pub(crate) struct DefaultShareLinkUrlGenerator {
    base_url: String,
}

impl DefaultShareLinkUrlGenerator {
    pub(crate) const fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Load the share-link base URL from `AGENT_TUI_SHARE_BASE_URL`.
    ///
    /// There is no built-in share service endpoint in this repository, so a
    /// production URL must be supplied explicitly; no built-in share-service
    /// endpoint is assumed.
    pub(crate) fn from_env() -> Result<Self, AppError> {
        let base_url = std::env::var("AGENT_TUI_SHARE_BASE_URL").map_err(|_| {
            AppError::State(
                "AGENT_TUI_SHARE_BASE_URL is not set; share links cannot be generated".to_string(),
            )
        })?;
        let parsed = reqwest::Url::parse(&base_url).map_err(|e| {
            AppError::ValidationError(format!("AGENT_TUI_SHARE_BASE_URL is not a valid URL: {e}"))
        })?;
        if !matches!(parsed.scheme(), "http" | "https") {
            return Err(AppError::ValidationError(
                "AGENT_TUI_SHARE_BASE_URL must use http or https".to_string(),
            ));
        }
        Ok(Self::new(base_url.trim_end_matches('/').to_string()))
    }
}

#[async_trait]
impl ShareLinkUrlGenerator for DefaultShareLinkUrlGenerator {
    async fn generate_url(&self, token: &str) -> Result<String, AppError> {
        Ok(format!("{}/{}", self.base_url, token))
    }

    async fn validate_url(&self, url: &str) -> Result<Option<String>, AppError> {
        if url.starts_with(&self.base_url) {
            let token = url.trim_start_matches(&self.base_url);
            let token = token.trim_start_matches('/');
            if !token.is_empty() {
                return Ok(Some(token.to_string()));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_url() {
        let generator = DefaultShareLinkUrlGenerator::new("https://share.example.com".to_string());
        let url = generator.generate_url("abc123").await.unwrap();
        assert_eq!(url, "https://share.example.com/abc123");
    }

    #[tokio::test]
    async fn test_validate_url_valid() {
        let generator = DefaultShareLinkUrlGenerator::new("https://share.example.com".to_string());
        let token = generator
            .validate_url("https://share.example.com/abc123")
            .await
            .unwrap();
        assert_eq!(token, Some("abc123".to_string()));
    }

    #[tokio::test]
    async fn test_validate_url_invalid() {
        let generator = DefaultShareLinkUrlGenerator::new("https://share.example.com".to_string());
        let token = generator
            .validate_url("https://other.com/abc123")
            .await
            .unwrap();
        assert_eq!(token, None);
    }
}
