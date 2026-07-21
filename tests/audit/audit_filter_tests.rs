//! Audit filter tests

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
fn test_filter_by_action_file() {
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
        ),
        create_test_audit_entry(
            AuditAction::CommandExecute {
                command: "ls".to_string(),
            },
            Actor {
                type_: ActorType::User,
                id: "1".to_string(),
                name: "User".to_string(),
            },
            Resource {
                type_: "command".to_string(),
                id: "2".to_string(),
                path: None,
            },
        ),
    ];

    let file_entries = filter_by_action(&entries, "file");
    assert_eq!(file_entries.len(), 1);
}

#[test]
fn test_filter_by_action_command() {
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
        ),
        create_test_audit_entry(
            AuditAction::CommandExecute {
                command: "ls".to_string(),
            },
            Actor {
                type_: ActorType::User,
                id: "1".to_string(),
                name: "User".to_string(),
            },
            Resource {
                type_: "command".to_string(),
                id: "2".to_string(),
                path: None,
            },
        ),
    ];

    let cmd_entries = filter_by_action(&entries, "command");
    assert_eq!(cmd_entries.len(), 1);
}

#[test]
fn test_filter_by_actor() {
    let entries = vec![
        create_test_audit_entry(
            AuditAction::FileRead {
                path: "a.txt".to_string(),
            },
            Actor {
                type_: ActorType::User,
                id: "user-1".to_string(),
                name: "User 1".to_string(),
            },
            Resource {
                type_: "file".to_string(),
                id: "1".to_string(),
                path: None,
            },
        ),
        create_test_audit_entry(
            AuditAction::FileRead {
                path: "b.txt".to_string(),
            },
            Actor {
                type_: ActorType::User,
                id: "user-2".to_string(),
                name: "User 2".to_string(),
            },
            Resource {
                type_: "file".to_string(),
                id: "2".to_string(),
                path: None,
            },
        ),
    ];

    let user_entries = filter_by_actor(&entries, "user-1");
    assert_eq!(user_entries.len(), 1);
}

#[test]
fn test_filter_by_time_range() {
    let now = Utc::now();
    let entries = vec![create_test_audit_entry(
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
    )];

    let filtered = filter_by_time_range(
        &entries,
        now - chrono::Duration::hours(1),
        now + chrono::Duration::hours(1),
    );
    assert_eq!(filtered.len(), 1);
}

#[test]
fn test_filter_by_time_range_outside() {
    let now = Utc::now();
    let entries = vec![create_test_audit_entry(
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
    )];

    // Time range before the entry
    let filtered = filter_by_time_range(
        &entries,
        now - chrono::Duration::days(1),
        now - chrono::Duration::hours(12),
    );
    assert_eq!(filtered.len(), 0);
}
