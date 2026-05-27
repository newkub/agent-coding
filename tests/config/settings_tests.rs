use agent_tui::{AppSettings, AISettings, UISettings, AccessibilitySettings, MemorySettings};

#[test]
fn test_app_settings_default() {
    let settings = AppSettings::default();
    
    assert_eq!(settings.ui.theme, "default");
    assert_eq!(settings.ai.default_model, "gpt-4");
    assert_eq!(settings.git.default_branch, "main");
    assert_eq!(settings.terminal.shell, "bash");
}

#[test]
fn test_ai_settings_default() {
    let settings = AISettings::default();
    
    assert_eq!(settings.default_model, "gpt-4");
    assert_eq!(settings.api_endpoint, "https://api.openai.com/v1");
    assert_eq!(settings.max_tokens, 4096);
    assert_eq!(settings.temperature, 0.7);
    assert_eq!(settings.request_timeout, 30);
    assert!(settings.enable_streaming);
    assert!(settings.enable_caching);
    assert_eq!(settings.cache_ttl, 3600);
}

#[test]
fn test_ui_settings_default() {
    let settings = UISettings::default();
    
    assert_eq!(settings.theme, "default");
    assert_eq!(settings.font_size, 12);
    assert!(settings.show_line_numbers);
    assert!(settings.enable_mouse);
    assert!(!settings.accessibility.enable_screen_reader);
    assert!(settings.accessibility.announce_focus);
}

#[test]
fn test_accessibility_settings_default() {
    let settings = AccessibilitySettings::default();
    
    assert!(!settings.enable_screen_reader);
    assert!(!settings.high_contrast);
    assert!(!settings.reduced_motion);
    assert!(settings.announce_focus);
    assert!(settings.announce_content);
}

#[test]
fn test_memory_settings_default() {
    let settings = MemorySettings::default();
    
    assert_eq!(settings.max_cache_size_mb, 50);
    assert!(settings.enable_memory_pooling);
    assert_eq!(settings.session_history_limit, 100);
    assert!(settings.enable_lazy_loading);
}

#[test]
fn test_serialize_deserialize_settings() {
    let settings = AppSettings::default();
    let serialized = serde_json::to_string(&settings).unwrap();
    let deserialized: AppSettings = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(deserialized.ui.theme, settings.ui.theme);
    assert_eq!(deserialized.ai.default_model, settings.ai.default_model);
}
