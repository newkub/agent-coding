//! Audit Metadata tests

use agent_tui::modules::audit::domain::models::AuditMetadata;

#[test]
fn test_audit_metadata_default() {
    let meta = AuditMetadata::default();
    assert!(meta.ip_address.is_none());
    assert!(meta.session_id.is_none());
}

#[test]
fn test_audit_metadata_new() {
    let meta = AuditMetadata::new();
    assert!(meta.ip_address.is_none());
}

#[test]
fn test_audit_metadata_with_session() {
    let meta = AuditMetadata::new().with_session("session-123".to_string());
    assert_eq!(meta.session_id, Some("session-123".to_string()));
}

#[test]
fn test_audit_metadata_with_duration() {
    let meta = AuditMetadata::new().with_duration(100);
    assert_eq!(meta.duration_ms, Some(100));
}
