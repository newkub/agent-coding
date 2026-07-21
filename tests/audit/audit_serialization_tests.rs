//! Audit serialization tests

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
fn test_audit_entry_serialization() {
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
    );

    let json = serde_json::to_string(&entry).unwrap();
    let parsed: AuditEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id.0, entry.id.0);
}

#[test]
fn test_audit_metadata_serialization() {
    let mut meta = AuditMetadata::default();
    meta.ip_address = Some("127.0.0.1".to_string());
    meta.extra.insert("key".to_string(), "value".to_string());

    let json = serde_json::to_string(&meta).unwrap();
    let parsed: AuditMetadata = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.ip_address, Some("127.0.0.1".to_string()));
}

#[test]
fn test_resource_serialization() {
    let resource = Resource {
        type_: "file".to_string(),
        id: "123".to_string(),
        path: Some("test.txt".to_string()),
    };

    let json = serde_json::to_string(&resource).unwrap();
    let parsed: Resource = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.path, Some("test.txt".to_string()));
}

#[test]
fn test_actor_serialization() {
    let actor = Actor {
        type_: ActorType::User,
        id: "user-123".to_string(),
        name: "Test User".to_string(),
    };

    let json = serde_json::to_string(&actor).unwrap();
    let parsed: Actor = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "Test User");
}
