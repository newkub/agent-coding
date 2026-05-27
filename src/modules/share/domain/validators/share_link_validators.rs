use crate::modules::share::domain::models::share_link::ShareLink;
use crate::shared::kernel::result::AppError;

/// Pure function to validate share link creation parameters
pub fn validate_share_link_creation(
    expires_in_hours: Option<u64>,
    max_access: Option<u32>,
) -> Result<(), AppError> {
    if let Some(hours) = expires_in_hours {
        if hours == 0 {
            return Err(AppError::ValidationError(
                "Expiration time must be greater than 0".to_string(),
            ));
        }
        if hours > 8760 {
            // Max 1 year
            return Err(AppError::ValidationError(
                "Expiration time cannot exceed 1 year".to_string(),
            ));
        }
    }

    if let Some(max) = max_access {
        if max == 0 {
            return Err(AppError::ValidationError(
                "Max access must be greater than 0".to_string(),
            ));
        }
        if max > 1000 {
            return Err(AppError::ValidationError(
                "Max access cannot exceed 1000".to_string(),
            ));
        }
    }

    Ok(())
}

/// Pure function to validate share link before access
pub fn validate_share_link_access(link: &ShareLink) -> Result<(), AppError> {
    if !link.is_active {
        return Err(AppError::ValidationError(
            "Share link has been deactivated".to_string(),
        ));
    }

    if link.is_expired() {
        return Err(AppError::ValidationError(
            "Share link has expired".to_string(),
        ));
    }

    if link.is_access_limit_reached() {
        return Err(AppError::ValidationError(
            "Share link has reached maximum access limit".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_validate_share_link_creation_valid() {
        assert!(validate_share_link_creation(Some(24), Some(10)).is_ok());
    }

    #[test]
    fn test_validate_share_link_creation_zero_expiration() {
        assert!(validate_share_link_creation(Some(0), Some(10)).is_err());
    }

    #[test]
    fn test_validate_share_link_creation_excessive_expiration() {
        assert!(validate_share_link_creation(Some(10000), Some(10)).is_err());
    }

    #[test]
    fn test_validate_share_link_creation_zero_max_access() {
        assert!(validate_share_link_creation(Some(24), Some(0)).is_err());
    }

    #[test]
    fn test_validate_share_link_access_valid() {
        let link = ShareLink::new(Uuid::new_v4(), Some(24), Some(10));
        assert!(validate_share_link_access(&link).is_ok());
    }

    #[test]
    fn test_validate_share_link_access_deactivated() {
        let mut link = ShareLink::new(Uuid::new_v4(), Some(24), Some(10));
        link.deactivate();
        assert!(validate_share_link_access(&link).is_err());
    }

    #[test]
    fn test_validate_share_link_access_expired() {
        let link = ShareLink::new(Uuid::new_v4(), Some(u64::MAX), Some(10));
        assert!(validate_share_link_access(&link).is_err());
    }
}
