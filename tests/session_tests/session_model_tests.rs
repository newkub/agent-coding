use super::*;

#[test]
fn test_session_new() {
    let session = create_test_session("Test Session".to_string());
    assert_eq!(session.name, "Test Session");
    assert!(session.messages.is_empty());
    assert!(!session.id.as_str().is_empty());
}

#[test]
fn test_session_add_message() {
    let mut session = create_test_session("Test".to_string());
    let msg = create_test_message(MessageRole::User, "Hello".to_string());
    let count = session.message_count();
    session.add_message(msg);
    assert_eq!(session.message_count(), count + 1);
}

#[test]
fn test_session_total_tokens() {
    let mut session = create_test_session("Test".to_string());
    let mut msg1 = create_test_message(MessageRole::User, "Hello".to_string());
    msg1.metadata = Some(MessageMetadata {
        tokens_used: Some(10),
        ..Default::default()
    });
    let mut msg2 = create_test_message(MessageRole::Assistant, "Hi".to_string());
    msg2.metadata = Some(MessageMetadata {
        tokens_used: Some(20),
        ..Default::default()
    });
    session.add_message(msg1);
    session.add_message(msg2);
    assert_eq!(session.total_tokens(), 30);
}

#[test]
fn test_calculate_session_stats() {
    let mut session = create_test_session("Test".to_string());
    let mut msg = create_test_message(MessageRole::User, "Hello".to_string());
    msg.metadata = Some(MessageMetadata {
        tokens_used: Some(50),
        ..Default::default()
    });
    session.add_message(msg);

    let stats = calculate_session_stats(&session);
    assert_eq!(stats.message_count, 1);
    assert_eq!(stats.total_tokens, 50);
}

#[test]
fn test_add_message_to_session() {
    let session = create_test_session("Test".to_string());
    let msg = create_test_message(MessageRole::User, "Hello".to_string());
    let new_session = add_message(&session, msg);
    assert_eq!(new_session.message_count(), 1);
}

#[test]
fn test_session_clone() {
    let session1 = create_test_session("Test".to_string());
    let session2 = session1.clone();
    assert_eq!(session1.id, session2.id);
    assert_eq!(session1.name, session2.name);
}
