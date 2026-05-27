# Collaboration Domain Tests

## ParticipantId
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_participant_id_new | Generate non-empty ID | `ParticipantId::from_string(uuid::Uuid::new_v4().to_string())` |
| ✅ | test_participant_id_default | Default generates non-empty ID | `ParticipantId::from_string(uuid::Uuid::new_v4().to_string())` |

## CollaborationId
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_collaboration_id_new | Generate non-empty ID | `CollaborationId::from_string(uuid::Uuid::new_v4().to_string())` |
| ✅ | test_collaboration_id_display | ID can be displayed | `format!("{}", id)` |

## ParticipantRole
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_participant_role_variants | All role variants match | `assert!(matches!(ParticipantRole::Owner, ParticipantRole::Owner))` |

## CollaborationStatus
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_collaboration_status_variants | All status variants match | `assert!(matches!(CollaborationStatus::Active, CollaborationStatus::Active))` |

## CollaborationSession
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_collaboration_session_new | Creates session with owner | `CollaborationSession::new(owner_id, name)` |
| ✅ | test_collaboration_session_add_participant | Can add participants | `session.add_participant(participant)` |
| ✅ | test_collaboration_session_remove_participant | Can remove participants | `session.remove_participant(participant_id)` |
| ✅ | test_collaboration_session_get_online_participants | Filters online participants | `session.get_online_participants()` |
| ✅ | test_collaboration_session_update_cursor | Can update cursor position | `session.update_cursor(participant_id, cursor)` |
| ✅ | test_collaboration_session_serialization | Session serializes/deserializes correctly | `serde_json::to_string(&session)` |
| ✅ | test_collaboration_session_remove_nonexistent | Removing non-existent does nothing | `session.remove_participant(nonexistent_id)` |

## CursorPosition
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_cursor_position_new | Creates position with line and column | `CursorPosition { file_path: Some("test.rs".to_string()), line: 10, column: 5 }` |
| ✅ | test_cursor_position_serialization | Position serializes/deserializes correctly | `serde_json::to_string(&position)` |

## SharedMessage
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_shared_message_new | Creates message with content | `SharedMessage { id, collaboration_id, sender_id, content, timestamp, message_type }` |
| ✅ | test_shared_message_type_variants | All message type variants match | `assert!(matches!(SharedMessageType::Chat, SharedMessageType::Chat))` |
| ✅ | test_shared_message_serialization | Message serializes/deserializes correctly | `serde_json::to_string(&message)` |

## Participant
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_participant_serialization | Participant serializes/deserializes correctly | `serde_json::to_string(&participant)` |

## IDs
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_collaboration_id_clone | ID clones correctly | `id1.clone()` |
| ✅ | test_participant_id_clone | ID clones correctly | `id1.clone()` |

## Edge Cases
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_update_cursor_nonexistent_participant | Updating non-existent participant does nothing | `session.update_cursor(nonexistent_id, cursor)` |
