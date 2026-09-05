use uuid::Uuid;

use crate::modules::share::domain::models::share_link::{ShareLink, SharePermissions};
use crate::modules::share::domain::operations::share_link_operations;
use crate::modules::share::domain::validators::share_link_validators;
use crate::modules::share::ports::{ShareLinkNotifier, ShareLinkRepository, ShareLinkUrlGenerator};
use crate::shared::kernel::result::AppError;

/// Use case for creating a share link
pub(crate) struct CreateShareLinkUseCase<R, U, N>
where
    R: ShareLinkRepository,
    U: ShareLinkUrlGenerator,
    N: ShareLinkNotifier,
{
    repository: R,
    url_generator: U,
    notifier: N,
}

impl<R, U, N> CreateShareLinkUseCase<R, U, N>
where
    R: ShareLinkRepository,
    U: ShareLinkUrlGenerator,
    N: ShareLinkNotifier,
{
    pub(crate) const fn new(repository: R, url_generator: U, notifier: N) -> Self {
        Self {
            repository,
            url_generator,
            notifier,
        }
    }

    /// Execute the use case to create a share link
    pub(crate) async fn execute(
        &self,
        session_id: Uuid,
        expires_in_hours: Option<u64>,
        max_access: Option<u32>,
        permissions: Option<SharePermissions>,
    ) -> Result<(ShareLink, String), AppError> {
        // Validate creation parameters
        share_link_validators::validate_share_link_creation(expires_in_hours, max_access)?;

        // Create share link with default or custom permissions
        let permissions = permissions.unwrap_or_else(share_link_operations::read_only_permissions);
        let mut link = ShareLink::new(session_id, expires_in_hours, max_access);
        link.permissions = permissions;

        // Save to repository
        self.repository.save(&link).await?;

        // Generate shareable URL
        let url = self.url_generator.generate_url(&link.token).await?;

        // Notify creation
        self.notifier.notify_created(&link, &url).await?;

        Ok((link, url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations for testing
    struct MockRepository;
    struct MockUrlGenerator;
    struct MockNotifier;

    #[async_trait::async_trait]
    impl ShareLinkRepository for MockRepository {
        async fn save(&self, _link: &ShareLink) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_by_id(&self, _id: Uuid) -> Result<Option<ShareLink>, AppError> {
            Ok(None)
        }
        async fn find_by_token(&self, _token: &str) -> Result<Option<ShareLink>, AppError> {
            Ok(None)
        }
        async fn find_by_session_id(&self, _session_id: Uuid) -> Result<Vec<ShareLink>, AppError> {
            Ok(vec![])
        }
        async fn update(&self, _link: &ShareLink) -> Result<(), AppError> {
            Ok(())
        }
        async fn delete(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn deactivate_by_session_id(&self, _session_id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl ShareLinkUrlGenerator for MockUrlGenerator {
        async fn generate_url(&self, token: &str) -> Result<String, AppError> {
            Ok(format!("https://share.agent-tui.com/{}", token))
        }
        async fn validate_url(&self, _url: &str) -> Result<Option<String>, AppError> {
            Ok(None)
        }
    }

    #[async_trait::async_trait]
    impl ShareLinkNotifier for MockNotifier {
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

    #[tokio::test]
    async fn test_create_share_link_success() {
        let repository = MockRepository;
        let url_generator = MockUrlGenerator;
        let notifier = MockNotifier;

        let use_case = CreateShareLinkUseCase::new(repository, url_generator, notifier);
        let session_id = Uuid::new_v4();

        let result = use_case.execute(session_id, Some(24), Some(10), None).await;

        assert!(result.is_ok());
        let (link, url) = result.unwrap();
        assert_eq!(link.session_id, session_id);
        assert!(url.contains("https://share.agent-tui.com/"));
    }

    #[tokio::test]
    async fn test_create_share_link_invalid_expiration() {
        let repository = MockRepository;
        let url_generator = MockUrlGenerator;
        let notifier = MockNotifier;

        let use_case = CreateShareLinkUseCase::new(repository, url_generator, notifier);
        let session_id = Uuid::new_v4();

        let result = use_case.execute(session_id, Some(0), Some(10), None).await;

        assert!(result.is_err());
    }
}
