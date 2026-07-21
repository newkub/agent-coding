//! Audit Entry tests

use agent_tui::modules::audit::domain::models::*;
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
fn test_audit_entry_new() {
    let action = AuditAction::FileRead {
        path: "test.txt".to_string(),
    };
    let actor = Actor {
        type_: ActorType::User,
        id: "user-123".to_string(),
        name: "Test User".to_string(),
    };
    let resource = Resource {
        type_: "file".to_string(),
        id: "res-456".to_string(),
        path: Some("test.txt".to_string()),
    };

    let entry = create_test_audit_entry(action.clone(), actor.clone(), resource.clone());
    assert!(!entry.id.0.is_empty());
    assert!(matches!(entry.result, AuditResult::Success));
}

#[test]
fn test_audit_entry_with_result() {
    let entry = create_test_audit_entry(
        AuditAction::CommandExecute {
            command: "ls".to_string(),
        },
        Actor {
            type_: ActorType::User,
            id: "1".to_string(),
            name: "Test".to_string(),
        },
        Resource {
            type_: "command".to_string(),
            id: "1".to_string(),
            path: None,
        },
    )
    .with_result(AuditResult::Failure {
        error: "failed".to_string(),
    });

    assert!(matches!(entry.result, AuditResult::Failure { .. }));
}

#[test]
fn test_audit_entry_with_metadata() {
    let entry = create_test_audit_entry(
        AuditAction::FileRead {
            path: "test.txt".to_string(),
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
    .with_metadata(AuditMetadata::new().with_session("session-1".to_string()));

    assert_eq!(entry.metadata.session_id, Some("session-1".to_string()));
}
