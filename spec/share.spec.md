# Share Domain Tests

## ExportedSession
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_exported_session_new | Creates exported session with version | `ExportedSession::new(session, 1)` |
| ✅ | test_exported_session_to_json | Converts to JSON | `serde_json::to_string(&exported)` |
| ✅ | test_exported_session_from_json | Parses from JSON | `serde_json::from_str::<ExportedSession>(json)` |
| ✅ | test_exported_session_from_json_invalid | Invalid JSON returns error | `serde_json::from_str::<ExportedSession>("invalid")` |
| ✅ | test_exported_session_serialization | Serializes/deserializes correctly | `serde_json::to_string(&exported)` |

## ImportResult
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_import_result_new | Creates result without warnings | `ImportResult::new(session)` |
| ✅ | test_import_result_with_warning | Can add warnings | `result.with_warning("warning")` |
| ✅ | test_import_result_serialization | Warnings work correctly | `serde_json::to_string(&result)` |

## KnowledgeEntry
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_knowledge_entry_new | Creates entry with metadata | `KnowledgeEntry::new("title", "content", metadata)` |
| ✅ | test_knowledge_entry_increment_usage | Can increment usage count | `entry.increment_usage()` |
| ✅ | test_knowledge_entry_serialization | Entry serializes/deserializes correctly | `serde_json::to_string(&entry)` |

## KnowledgeBase
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_knowledge_base_new | Creates empty knowledge base | `KnowledgeBase::new()` |
| ✅ | test_knowledge_base_search | Searches entries | `base.search("query")` |
| ✅ | test_knowledge_base_search_case_insensitive | Search is case insensitive | `base.search("QUERY")` |
| ✅ | test_knowledge_base_search_no_results | Returns empty for no matches | `base.search("nomatch")` |
| ✅ | test_knowledge_base_search_in_tags | Searches in tags | `base.search_in_tags("tag")` |
| ✅ | test_knowledge_base_by_category | Filters by category | `base.by_category("category")` |
| ✅ | test_knowledge_base_by_category_case_insensitive | Category filter is case insensitive | `base.by_category("CATEGORY")` |
| ✅ | test_knowledge_base_serialization | Serializes/deserializes correctly | `serde_json::to_string(&base)` |

## ExportMetadata
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_export_metadata_default | Default metadata has defaults | `ExportMetadata::default()` |
| ✅ | test_export_metadata_serialization | Serializes/deserializes correctly | `serde_json::to_string(&metadata)` |
