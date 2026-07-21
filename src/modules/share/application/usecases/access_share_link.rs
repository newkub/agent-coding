use crate::modules::share::domain::models::share_link::ShareLink;
use crate::modules::share::domain::operations::share_link_operations::{
    validate_permissions, ShareAction,
};
use crate::modules::share::domain::validators::share_link_validators;
use crate::modules::share::ports::{ShareLinkNotifier, ShareLinkRepository};
use crate::shared::kernel::result::AppError;

/// Use case for accessing a shared session via share link
pub struct AccessShareLinkUseCase<R, N>
where
    R: ShareLinkRepository,
    N: ShareLinkNotifier,
{
    repository: R,
    notifier: N,
}

impl<R, N> AccessShareLinkUseCase<R, N>
where
    R: ShareLinkRepository,
    N: ShareLinkNotifier,
{
    pub(crate) const fn new(repository: R, notifier: N) -> Self {
        Self {
            repository,
            notifier,
        }
    }

    /// Execute the use case to access a shared session
    pub(crate) async fn execute(
        &self,
        token: &str,
        action: ShareAction,
    ) -> Result<ShareLink, AppError> {
        // Find share link by token
        let mut link = self
            .repository
            .find_by_token(token)
            .await?
            .ok_or_else(|| AppError::NotFound("Share link not found".to_string()))?;

        // Validate share link is accessible
        share_link_validators::validate_share_link_access(&link)?;

        // Validate permissions for the requested action
        if !validate_permissions(&link.permissions, action) {
            return Err(AppError::PermissionDenied(
                "Insufficient permissions for this action".to_string(),
            ));
        }

        // Increment access count
        link.increment_access();
        self.repository.update(&link).await?;

        // Notify access
        self.notifier.notify_accessed(&link).await?;

        Ok(link)
    }

    /// Execute the use case to get share link info without incrementing access
    pub(crate) async fn get_info(&self, token: &str) -> Result<ShareLink, AppError> {
        let link = self
            .repository
            .find_by_token(token)
            .await?
            .ok_or_else(|| AppError::NotFound("Share link not found".to_string()))?;

        // Validate share link is accessible
        share_link_validators::validate_share_link_access(&link)?;

        Ok(link)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::share::domain::models::share_link::SharePermissions;
    use std::sync::Arc;
    use uuid::Uuid;

    struct MockRepository;
    struct MockNotifier;

    #[async_trait::async_trait]
    impl ShareLinkRepository for MockRepository {
        async fn save(&self, _link: &ShareLink) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_by_id(&self, _id: Uuid) -> Result<Option<ShareLink>, AppError> {
            Ok(None)
        }
        async fn find_by_token(&self, token: &str) -> Result<Option<ShareLink>, AppError> {
            if token == "valid_token" {
                let mut link = ShareLink::new(Uuid::new_v4(), Some(24), Some(10));
                link.token = token.to_string();
                Ok(Some(link))
            } else {
                Ok(None)
            }
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
    async fn test_access_share_link_success() {
        let repository = MockRepository;
        let notifier = MockNotifier;

        let use_case = AccessShareLinkUseCase::new(repository, notifier);
        let result = use_case.execute("valid_token", ShareAction::Read).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_access_share_link_not_found() {
        let repository = MockRepository;
        let notifier = MockNotifier;

        let use_case = AccessShareLinkUseCase::new(repository, notifier);
        let result = use_case.execute("invalid_token", ShareAction::Read).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_access_share_link_permission_denied() {
        let repository = MockRepository;
        let notifier = MockNotifier;

        let use_case = AccessShareLinkUseCase::new(repository, notifier);
        // Default permissions are read-only, so write should fail
        let result = use_case.execute("valid_token", ShareAction::Write).await;

        assert!(result.is_err());
    }
}
