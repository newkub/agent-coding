use super::*;

#[test]
fn test_session_id_new() {
    let id = SessionId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.as_str().is_empty());
}

#[test]
fn test_session_id_from_string() {
    let id = SessionId::from_string("test-id".to_string());
    assert_eq!(id.as_str(), "test-id");
}

#[test]
fn test_session_id_display() {
    let id = SessionId::from_string("display-test".to_string());
    assert_eq!(format!("{}", id), "display-test");
}

#[test]
fn test_session_id_default() {
    let id = SessionId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.as_str().is_empty());
}

#[test]
fn test_session_id_eq() {
    let id1 = SessionId::from_string("same".to_string());
    let id2 = SessionId::from_string("same".to_string());
    let id3 = SessionId::from_string("different".to_string());
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_session_id_clone() {
    let id1 = SessionId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}
