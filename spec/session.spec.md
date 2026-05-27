# Session Domain Tests

## SessionId
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_session_id_new | Generate non-empty ID | `SessionId::from_string(uuid::Uuid::new_v4().to_string())` |
| ✅ | test_session_id_from_string | Creates ID from string | `SessionId::from_string("session-1")` |
| ✅ | test_session_id_display | ID displays correctly | `format!("{}", id)` |
| ✅ | test_session_id_default | Default generates non-empty ID | `SessionId::default()` |
| ✅ | test_session_id_eq | ID equality works | `assert_eq!(id1, id2)` |
| ✅ | test_session_id_clone | ID clones correctly | `id1.clone()` |

## MessageRole
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_message_role_as_str | All roles return correct string | `MessageRole::User.as_str()` |

## Message
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_message_new | Creates message with role and content | `Message::new(MessageRole::User, "Hello")` |
| ✅ | test_message_system | Creates system message | `Message::system("System prompt")` |
| ✅ | test_message_user | Creates user message | `Message::user("User input")` |
| ✅ | test_message_assistant | Creates assistant message | `Message::assistant("Response")` |
| ✅ | test_message_with_metadata | Message can have metadata | `message.with_metadata(metadata)` |
| ✅ | test_message_clone | Message clones correctly | `message.clone()` |

## Session
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_session_new | Creates session with name | `Session::new("My Session")` |
| ✅ | test_session_add_message | Can add messages | `session.add_message(message)` |
| ✅ | test_session_total_tokens | Calculates total tokens | `session.total_tokens()` |
| ✅ | test_session_clone | Session clones correctly | `session.clone()` |

## SessionMetadata
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_session_metadata_default | Default metadata is empty | `SessionMetadata::default()` |
| ✅ | test_session_metadata_with_values | Can set metadata values | `metadata.with_value("key", "value")` |

## MessageMetadata
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_message_metadata_default | Default metadata is empty | `MessageMetadata::default()` |

## ToolCall
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_tool_call_serialization | Tool call serializes/deserializes correctly | `serde_json::to_string(&tool_call)` |

## Session Validation
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_validate_session_name_valid | Valid names pass validation | `validate_session_name("valid_name")` |
| ✅ | test_validate_session_name_empty | Empty name returns error | `validate_session_name("")` |
| ✅ | test_validate_session_name_too_long | Long name returns error | `validate_session_name("a".repeat(101))` |
| ✅ | test_validate_session_name_invalid_chars | Invalid chars return error | `validate_session_name("invalid name")` |
| ✅ | test_session_validation_error_display | Errors display correctly | `format!("{}", error)` |

## Session Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_calculate_session_stats | Calculates session statistics | `calculate_session_stats(&session)` |
| ✅ | test_create_session_valid | Creates valid session | `create_session("valid_name")` |
| ✅ | test_create_session_invalid | Invalid session returns error | `create_session("")` |
| ✅ | test_add_message_to_session | Adds message to session | `add_message_to_session(&mut session, message)` |
