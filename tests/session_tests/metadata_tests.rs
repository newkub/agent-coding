#![allow(clippy::field_reassign_with_default)]

use super::*;

#[test]
fn test_session_metadata_default() {
    let meta = SessionMetadata::default();
    assert!(meta.provider.is_none());
    assert!(meta.model.is_none());
    assert!(meta.tags.is_empty());
    assert!(!meta.pinned);
}

#[test]
fn test_session_metadata_with_values() {
    let mut meta = SessionMetadata::default();
    meta.provider = Some("openai".to_string());
    meta.model = Some("gpt-4".to_string());
    meta.tags = vec!["test".to_string(), "demo".to_string()];
    meta.pinned = true;
    assert_eq!(meta.provider.as_deref(), Some("openai"));
    assert_eq!(meta.model.as_deref(), Some("gpt-4"));
    assert_eq!(meta.tags.len(), 2);
    assert!(meta.pinned);
}
