# Snippet Domain Tests

## SnippetId
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_snippet_id_new | Generate non-empty ID | `SnippetId::from_string(uuid::Uuid::new_v4().to_string())` |
| ✅ | test_snippet_id_from_string | Creates ID from string | `SnippetId::from_string("snippet-1")` |
| ✅ | test_snippet_id_display | ID displays correctly | `format!("{}", id)` |
| ✅ | test_snippet_id_default | Default generates non-empty ID | `SnippetId::default()` |
| ✅ | test_snippet_id_eq | ID equality works | `assert_eq!(id1, id2)` |
| ✅ | test_snippet_id_clone | ID clones correctly | `id1.clone()` |

## Snippet
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_snippet_new | Creates snippet with metadata | `Snippet::new("name", "code", "language")` |
| ✅ | test_snippet_with_tags | Can add tags | `snippet.with_tags(vec!["tag1".to_string()])` |
| ✅ | test_snippet_add_variable | Can add variables | `snippet.add_variable("var", "value")` |
| ✅ | test_snippet_render | Renders with variable substitution | `snippet.render(&vars)` |
| ✅ | test_snippet_render_multiple_variables | Renders with multiple variables | `snippet.render(&multiple_vars)` |
| ✅ | test_snippet_extract_variables | Extracts variables from code | `snippet.extract_variables()` |
| ✅ | test_snippet_extract_variables_no_vars | Returns empty for no variables | `snippet.extract_variables()` |
| ✅ | test_snippet_clone | Snippet clones correctly | `snippet.clone()` |

## SnippetLibrary
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_snippet_library_new | Creates empty library | `SnippetLibrary::new()` |
| ✅ | test_snippet_library_add | Can add snippets | `library.add(snippet)` |
| ✅ | test_snippet_library_search | Searches snippets | `library.search("query")` |
| ✅ | test_snippet_library_search_case_insensitive | Search is case insensitive | `library.search("QUERY")` |
| ✅ | test_snippet_library_search_no_results | Returns empty for no matches | `library.search("nomatch")` |
| ✅ | test_snippet_library_by_language | Filters by language | `library.by_language("rust")` |
| ✅ | test_snippet_library_by_tag | Filters by tag | `library.by_tag("tag")` |
| ✅ | test_snippet_library_serialization | Serializes/deserializes correctly | `serde_json::to_string(&library)` |

## SnippetVariable
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_snippet_variable_default | Creates variable with defaults | `SnippetVariable::default()` |

## Snippet Validation
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_validate_snippet_name_valid | Valid names pass validation | `validate_snippet_name("valid_name")` |
| ✅ | test_validate_snippet_name_empty | Empty name returns error | `validate_snippet_name("")` |
| ✅ | test_validate_snippet_name_too_long | Long name returns error | `validate_snippet_name("a".repeat(101))` |
| ✅ | test_validate_snippet_name_invalid | Invalid chars return error | `validate_snippet_name("invalid name")` |
| ✅ | test_validate_snippet_code_valid | Valid code passes validation | `validate_snippet_code("code")` |
| ✅ | test_validate_snippet_code_empty | Empty code returns error | `validate_snippet_code("")` |
| ✅ | test_validate_snippet_code_whitespace | Whitespace is valid | `validate_snippet_code("   ")` |
| ✅ | test_validate_snippet_code_too_long | Long code returns error | `validate_snippet_code("a".repeat(10001))` |
| ✅ | test_calculate_snippet_stats | Calculates snippet statistics | `calculate_snippet_stats(&snippet)` |
| ✅ | test_snippet_validation_error_display | Errors display correctly | `format!("{}", error)` |
