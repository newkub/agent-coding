# Configuration Tests

## App Settings
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_app_settings_default | Default theme is "default", default model is "gpt-4", default branch is "main", shell is "bash" | `AppSettings::default()` |
| ✅ | test_serialize_deserialize_settings | Settings serialize/deserialize correctly | `serde_json::to_string(&settings)` |

## AI Settings
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_ai_settings_default | Default model is "gpt-4", endpoint is OpenAI API, max tokens is 4096, temperature is 0.7, timeout is 30s, streaming and caching enabled, cache TTL is 3600s | `AISettings::default()` |

## UI Settings
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_ui_settings_default | Default theme is "default", font size is 12, line numbers and mouse enabled, screen reader disabled, focus announcement enabled | `UISettings::default()` |

## Accessibility Settings
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_accessibility_settings_default | Screen reader, high contrast, and reduced motion disabled, focus and content announcement enabled | `AccessibilitySettings::default()` |

## Memory Settings
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_memory_settings_default | Max cache size is 50MB, memory pooling enabled, session history limit is 100, lazy loading enabled | `MemorySettings::default()`
