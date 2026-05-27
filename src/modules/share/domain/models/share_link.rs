use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use base64::Engine;

/// Share link entity for session collaboration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShareLink {
    pub id: Uuid,
    pub session_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub access_count: u32,
    pub max_access: Option<u32>,
    pub is_active: bool,
    pub permissions: SharePermissions,
}

/// Permissions for shared sessions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SharePermissions {
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub can_delete: bool,
}

impl Default for SharePermissions {
    fn default() -> Self {
        Self {
            can_read: true,
            can_write: false,
            can_execute: false,
            can_delete: false,
        }
    }
}

impl ShareLink {
    /// Create a new share link
    pub fn new(session_id: Uuid, expires_in_hours: Option<u64>, max_access: Option<u32>) -> Self {
        let now = Utc::now();
        let expires_at = expires_in_hours.map(|hours| now + chrono::Duration::hours(hours as i64));
        
        Self {
            id: Uuid::new_v4(),
            session_id,
            token: Self::generate_token(),
            created_at: now,
            expires_at,
            access_count: 0,
            max_access,
            is_active: true,
            permissions: SharePermissions::default(),
        }
    }

    /// Generate a unique share token
    fn generate_token() -> String {
        let bytes = Uuid::new_v4().as_bytes().to_vec();
        base64::engine::general_purpose::URL_SAFE.encode(bytes)
    }

    /// Check if the share link is expired
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires) => Utc::now() > expires,
            None => false,
        }
    }

    /// Check if the share link has reached max access
    pub const fn is_access_limit_reached(&self) -> bool {
        match self.max_access {
            Some(max) => self.access_count >= max,
            None => false,
        }
    }

    /// Check if the share link is valid (active, not expired, not at limit)
    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired() && !self.is_access_limit_reached()
    }

    /// Increment access count
    pub fn increment_access(&mut self) {
        self.access_count += 1;
    }

    /// Deactivate the share link
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Set custom permissions
    pub const fn with_permissions(mut self, permissions: SharePermissions) -> Self {
        self.permissions = permissions;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_share_link_creation() {
        let session_id = Uuid::new_v4();
        let link = ShareLink::new(session_id, Some(24), Some(10));
        
        assert_eq!(link.session_id, session_id);
        assert!(link.is_active);
        assert_eq!(link.access_count, 0);
        assert_eq!(link.max_access, Some(10));
    }

    #[test]
    fn test_share_link_expiration() {
        let link = ShareLink::new(Uuid::new_v4(), Some(u64::MAX), None);
        assert!(link.is_expired());
    }

    #[test]
    fn test_share_link_access_limit() {
        let mut link = ShareLink::new(Uuid::new_v4(), None, Some(2));
        link.access_count = 2;
        assert!(link.is_access_limit_reached());
    }

    #[test]
    fn test_share_link_validity() {
        let link = ShareLink::new(Uuid::new_v4(), Some(24), Some(10));
        assert!(link.is_valid());
    }

    #[test]
    fn test_share_link_increment_access() {
        let mut link = ShareLink::new(Uuid::new_v4(), None, None);
        let initial_count = link.access_count;
        link.increment_access();
        assert_eq!(link.access_count, initial_count + 1);
    }

    #[test]
    fn test_share_link_deactivation() {
        let mut link = ShareLink::new(Uuid::new_v4(), None, None);
        link.deactivate();
        assert!(!link.is_active);
    }
}
