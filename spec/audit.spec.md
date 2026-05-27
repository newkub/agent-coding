# Audit

## AuditId

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_audit_id_new | Generate non-empty ID. ID string is not empty | `AuditId::from_string(uuid::Uuid::new_v4().to_string())` |
| ✅ | test_audit_id_default | Default generates non-empty ID. ID string is not empty | `AuditId::from_string(uuid::Uuid::new_v4().to_string())` |

## AuditAction

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_audit_action_category_file | File actions return "file" category. Category is "file" for FileRead, FileWrite, FileDelete | `AuditAction::FileRead { path: "test.txt".to_string() }.category()` |
| ✅ | test_audit_action_category_command | Command actions return "command" category. Category is "command" for CommandExecute, CommandApprove, CommandReject | `AuditAction::CommandExecute { command: "ls".to_string() }.category()` |
| ✅ | test_audit_action_category_git | Git actions return "git" category. Category is "git" for GitCommit, GitPush, GitBranch | `AuditAction::GitCommit { message: "test".to_string(), files: vec![] }.category()` |
| ✅ | test_audit_action_category_session | Session actions return "session" category. Category is "session" for SessionCreate, SessionDelete, MessageSend | `AuditAction::SessionCreate { name: "test".to_string() }.category()` |
| ✅ | test_audit_action_category_ai | AI actions return "ai" category. Category is "ai" for AiRequest | `AuditAction::AiRequest { model: "gpt-4".to_string(), tokens: 100 }.category()` |
| ✅ | test_audit_action_category_system | System actions return "system" category. Category is "system" for ConfigChange, PluginLoad | `AuditAction::ConfigChange { key: "test".to_string() }.category()` |

## Actor

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_actor_type_variants | All actor type variants match. User, Ai, and System variants exist | `assert!(matches!(ActorType::User, ActorType::User))` |

## AuditEntry

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_audit_entry_new | Creates entry with non-empty ID and Success result. Entry has non-empty ID and Success result | `AuditEntry::create(AuditId, Utc::now(), action, actor, resource)` |
| ✅ | test_audit_entry_with_result | Entry can have custom result. Entry can have Failure result with error message | `entry.with_result(AuditResult::Failure { error: "failed".to_string() })` |
| ✅ | test_audit_entry_with_metadata | Entry can have metadata. Entry can have metadata with session ID | `entry.with_metadata(AuditMetadata::new().with_session("session-1".to_string()))` |

## AuditMetadata

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_audit_metadata_default | Default has no IP or session. IP address and session ID are None | `AuditMetadata::default()` |
| ✅ | test_audit_metadata_new | New has no IP address. IP address is None | `AuditMetadata::new()` |
| ✅ | test_audit_metadata_with_session | Can set session ID. Session ID is set to provided value | `AuditMetadata::new().with_session("session-123".to_string())` |
| ✅ | test_audit_metadata_with_duration | Can set duration. Duration is set to provided value | `AuditMetadata::new().with_duration(100)` |

## AuditResult

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_audit_result_variants | All result variants match. Success and Failure variants exist | `assert!(matches!(entry.result, AuditResult::Success))` |

## Resource

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_resource_serialization | Resource serializes/deserializes correctly. Resource can be serialized and deserialized | `Resource { type_: "file".to_string(), id: "res-456".to_string(), path: Some("test.txt".to_string()) }` |
| ✅ | test_actor_serialization | Actor serializes/deserializes correctly. Actor can be serialized and deserialized | `Actor { type_: ActorType::User, id: "user-123".to_string(), name: "Test User".to_string() }` |

## Audit Operations

| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_generate_audit_summary_empty | Empty entries return zero summary. Summary has total=0, success=0, failure=0 | `generate_audit_summary(&[])` |
| ✅ | test_generate_audit_summary_with_entries | Summary counts entries correctly. Summary counts total, success, failure, and by_category | `generate_audit_summary(&[entry1, entry2])` |
| ✅ | test_audit_summary_default | Default summary is empty. Summary has total=0 and empty by_category | `AuditSummary::default()` |
| ✅ | test_audit_rule_new | Rule created with pattern and allow flag. Rule has action_pattern, allow flag, and description | `AuditRule::new("git .*", true, "Approve git commands")` |
| ✅ | test_audit_rule_with_actor | Rule can have actor pattern. Rule has actor_id_pattern set | `AuditRule::new(".*", true, "Test").with_actor("user-.*")` |
| ✅ | test_audit_rule_matches_action | Rule matches action pattern. Rule returns true when action matches pattern | `rule.matches(&action, &actor)` |
| ✅ | test_audit_rule_matches_actor | Rule matches actor pattern. Rule returns true when actor matches pattern | `rule.matches(&action, &matching_actor)` |
| ✅ | test_is_action_allowed_no_rules | No rules means action allowed. Returns true when rules array is empty | `is_action_allowed(&action, &actor, &[])` |
| ✅ | test_filter_by_action_file | Filters entries by file category. Returns only entries with file category | `filter_by_action(&entries, "file")` |
| ✅ | test_filter_by_action_command | Filters entries by command category. Returns only entries with command category | `filter_by_action(&entries, "command")` |
| ✅ | test_filter_by_actor | Filters entries by actor ID. Returns only entries matching actor ID | `filter_by_actor(&entries, "user-1")` |
| ✅ | test_filter_by_time_range | Filters entries by time range. Returns entries within time range | `filter_by_time_range(&entries, start, end)` |
| ✅ | test_filter_by_time_range_outside | Filters out entries outside range. Returns empty when entries are outside time range | `filter_by_time_range(&entries, now - days(1), now - hours(12))` |
