//! Audit ID tests

use agent_tui::modules::audit::domain::models::AuditId;

#[test]
fn test_audit_id_new() {
    let id = AuditId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.0.is_empty());
}

#[test]
fn test_audit_id_default() {
    let id = AuditId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.0.is_empty());
}
