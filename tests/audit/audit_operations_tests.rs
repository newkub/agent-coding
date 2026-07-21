//! Audit operations tests

use agent_tui::modules::audit::domain::models::*;
use agent_tui::modules::audit::domain::operations::*;
use chrono::Utc;

fn create_test_audit_entry(action: AuditAction, actor: Actor, resource: Resource) -> AuditEntry {
    AuditEntry::create(
        AuditId::from_string(uuid::Uuid::new_v4().to_string()),
        Utc::now(),
        action,
        actor,
        resource,
    )
}

#[test]
fn test_generate_audit_summary_empty() {
    let entries: Vec<AuditEntry> = vec![];
    let summary = generate_audit_summary(&entries);
    assert_eq!(summary.total, 0);
    assert_eq!(summary.success, 0);
    assert_eq!(summary.failure, 0);
}

#[test]
fn test_generate_audit_summary_with_entries() {
    let entries = vec![
        create_test_audit_entry(
            AuditAction::FileRead {
                path: "a.txt".to_string(),
            },
            Actor {
                type_: ActorType::User,
                id: "1".to_string(),
                name: "User".to_string(),
            },
            Resource {
                type_: "file".to_string(),
                id: "1".to_string(),
                path: None,
            },
        )
        .with_result(AuditResult::Success),
        create_test_audit_entry(
            AuditAction::FileRead {
                path: "b.txt".to_string(),
            },
            Actor {
                type_: ActorType::User,
                id: "1".to_string(),
                name: "User".to_string(),
            },
            Resource {
                type_: "file".to_string(),
                id: "2".to_string(),
                path: None,
            },
        )
        .with_result(AuditResult::Failure {
            error: "error".to_string(),
        }),
    ];

    let summary = generate_audit_summary(&entries);
    assert_eq!(summary.total, 2);
    assert_eq!(summary.success, 1);
    assert_eq!(summary.failure, 1);
    assert_eq!(summary.by_category.get("file"), Some(&2));
}

#[test]
fn test_audit_summary_default() {
    let summary = agent_tui::modules::audit::domain::operations::AuditSummary::default();
    assert_eq!(summary.total, 0);
    assert!(summary.by_category.is_empty());
}

#[test]
fn test_audit_rule_new() {
    let rule = AuditRule::new("git .*", true, "Approve git commands");
    assert_eq!(rule.action_pattern, "git .*");
    assert!(rule.allow);
    assert!(rule.description.contains("Approve"));
}

#[test]
fn test_audit_rule_with_actor() {
    let rule = AuditRule::new(".*", true, "Test").with_actor("user-.*");
    assert_eq!(rule.actor_id_pattern, Some("user-.*".to_string()));
}

#[test]
fn test_audit_rule_matches_action() {
    let rule = AuditRule::new("CommandExecute", true, "Test");
    let action = AuditAction::CommandExecute {
        command: "ls".to_string(),
    };

    let actor = Actor {
        type_: ActorType::User,
        id: "any".to_string(),
        name: "Test".to_string(),
    };
    assert!(rule.matches(&action, &actor));
}

#[test]
fn test_audit_rule_matches_actor() {
    let rule = AuditRule::new(".*", true, "Test").with_actor("user-1");
    let action = AuditAction::FileRead {
        path: "test".to_string(),
    };

    let matching_actor = Actor {
        type_: ActorType::User,
        id: "user-1".to_string(),
        name: "User".to_string(),
    };
    assert!(rule.matches(&action, &matching_actor));

    let non_matching_actor = Actor {
        type_: ActorType::User,
        id: "user-2".to_string(),
        name: "User".to_string(),
    };
    assert!(!rule.matches(&action, &non_matching_actor));
}

#[test]
fn test_is_action_allowed_no_rules() {
    let action = AuditAction::FileRead {
        path: "test".to_string(),
    };
    let actor = Actor {
        type_: ActorType::User,
        id: "1".to_string(),
        name: "Test".to_string(),
    };
    let rules: [AuditRule; 0] = [];

    assert!(is_action_allowed(&action, &actor, &rules));
}
