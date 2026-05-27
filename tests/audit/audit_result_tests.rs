//! Audit Result tests

use agent_tui::modules::audit::domain::models::AuditResult;

#[test]
fn test_audit_result_variants() {
    assert!(matches!(AuditResult::Success, AuditResult::Success));
    assert!(matches!(AuditResult::Pending, AuditResult::Pending));
    let failure = AuditResult::Failure { error: "test".to_string() };
    assert!(matches!(failure, AuditResult::Failure { error: _ }));
}
