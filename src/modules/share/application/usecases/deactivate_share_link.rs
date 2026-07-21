use uuid::Uuid;

use crate::modules::share::domain::models::share_link::ShareLink;
use crate::modules::share::ports::{ShareLinkNotifier, ShareLinkRepository};
use crate::shared::kernel::result::AppError;

/// Use case for deactivating a share link
pub struct DeactivateShareLinkUseCase<R, N>
where
    R: ShareLinkRepository,
    N: ShareLinkNotifier,
{
    repository: R,
    notifier: N,
}

impl<R, N> DeactivateShareLinkUseCase<R, N>
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

    /// Execute the use case to deactivate a share link by ID
    pub(crate) async fn execute_by_id(&self, id: Uuid) -> Result<ShareLink, AppError> {
        let mut link = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Share link not found".to_string()))?;

        link.deactivate();
        self.repository.update(&link).await?;

        self.notifier.notify_deactivated(&link).await?;

        Ok(link)
    }

    /// Execute the use case to deactivate a share link by token
    pub(crate) async fn execute_by_token(&self, token: &str) -> Result<ShareLink, AppError> {
        let mut link = self
            .repository
            .find_by_token(token)
            .await?
            .ok_or_else(|| AppError::NotFound("Share link not found".to_string()))?;

        link.deactivate();
        self.repository.update(&link).await?;

        self.notifier.notify_deactivated(&link).await?;

        Ok(link)
    }

    /// Execute the use case to deactivate all share links for a session
    pub(crate) async fn execute_by_session_id(&self, session_id: Uuid) -> Result<(), AppError> {
        self.repository.deactivate_by_session_id(session_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockRepository;
    struct MockNotifier;

    #[async_trait::async_trait]
    impl ShareLinkRepository for MockRepository {
        async fn save(&self, _link: &ShareLink) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<ShareLink>, AppError> {
            if id == Uuid::nil() {
                let mut link = ShareLink::new(Uuid::new_v4(), Some(24), Some(10));
                link.id = id;
                Ok(Some(link))
            } else {
                Ok(None)
            }
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
    async fn test_deactivate_share_link_by_id_success() {
        let repository = MockRepository;
        let notifier = MockNotifier;

        let use_case = DeactivateShareLinkUseCase::new(repository, notifier);
        let result = use_case.execute_by_id(Uuid::nil()).await;

        assert!(result.is_ok());
        let link = result.unwrap();
        assert!(!link.is_active);
    }

    #[tokio::test]
    async fn test_deactivate_share_link_by_id_not_found() {
        let repository = MockRepository;
        let notifier = MockNotifier;

        let use_case = DeactivateShareLinkUseCase::new(repository, notifier);
        let result = use_case.execute_by_id(Uuid::new_v4()).await;

        assert!(result.is_err());
    }
}
