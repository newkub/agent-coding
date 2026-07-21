use crate::modules::share::domain::models::share_link::{ShareLink, SharePermissions};

/// Pure function to validate share link permissions
pub const fn validate_permissions(permissions: &SharePermissions, action: ShareAction) -> bool {
    match action {
        ShareAction::Read => permissions.can_read,
        ShareAction::Write => permissions.can_write,
        ShareAction::Execute => permissions.can_execute,
        ShareAction::Delete => permissions.can_delete,
    }
}

/// Pure function to check if share link is accessible
pub fn is_accessible(link: &ShareLink) -> bool {
    link.is_valid()
}

/// Pure function to create read-only permissions
pub const fn read_only_permissions() -> SharePermissions {
    SharePermissions {
        can_read: true,
        can_write: false,
        can_execute: false,
        can_delete: false,
    }
}

/// Pure function to create full access permissions
pub const fn full_access_permissions() -> SharePermissions {
    SharePermissions {
        can_read: true,
        can_write: true,
        can_execute: true,
        can_delete: true,
    }
}

/// Pure function to create custom permissions
pub const fn custom_permissions(
    can_read: bool,
    can_write: bool,
    can_execute: bool,
    can_delete: bool,
) -> SharePermissions {
    SharePermissions {
        can_read,
        can_write,
        can_execute,
        can_delete,
    }
}

/// Actions that can be performed on a shared session
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareAction {
    Read,
    Write,
    Execute,
    Delete,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_validate_read_permissions() {
        let permissions = read_only_permissions();
        assert!(validate_permissions(&permissions, ShareAction::Read));
        assert!(!validate_permissions(&permissions, ShareAction::Write));
    }

    #[test]
    fn test_validate_full_permissions() {
        let permissions = full_access_permissions();
        assert!(validate_permissions(&permissions, ShareAction::Read));
        assert!(validate_permissions(&permissions, ShareAction::Write));
        assert!(validate_permissions(&permissions, ShareAction::Execute));
        assert!(validate_permissions(&permissions, ShareAction::Delete));
    }

    #[test]
    fn test_custom_permissions() {
        let permissions = custom_permissions(true, true, false, false);
        assert!(validate_permissions(&permissions, ShareAction::Read));
        assert!(validate_permissions(&permissions, ShareAction::Write));
        assert!(!validate_permissions(&permissions, ShareAction::Execute));
    }

    #[test]
    fn test_is_accessible() {
        let session_id = Uuid::new_v4();
        let link = ShareLink::new(session_id, Some(24), Some(10));
        assert!(is_accessible(&link));
    }
}
