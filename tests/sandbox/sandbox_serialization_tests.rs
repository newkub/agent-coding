//! Sandbox serialization tests

use agent_tui::modules::sandbox::domain::models::{ApprovalRule, RuleId, RuleAction, SecurityLevel};

#[test]
fn test_rule_serialization() {
    let rule = ApprovalRule {
        id: RuleId::from_string(uuid::Uuid::new_v4().to_string()),
        name: "Test Rule".to_string(),
        pattern: "git .*".to_string(),
        action: RuleAction::AutoApprove,
        security_level: SecurityLevel::Safe,
        description: "Approve git commands".to_string(),
    };
    
    let json = serde_json::to_string(&rule).unwrap();
    let parsed: ApprovalRule = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "Test Rule");
}
