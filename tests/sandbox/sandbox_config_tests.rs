//! Sandbox Config tests

use agent_tui::modules::sandbox::domain::models::{SandboxConfig, RuleId};

#[test]
fn test_rule_id_new() {
    let id = RuleId::from_string(uuid::Uuid::new_v4().to_string());
    assert!(!id.to_string().is_empty());
}

#[test]
fn test_sandbox_config_default() {
    let config = SandboxConfig::default();
    assert!(config.enabled);
    assert_eq!(config.memory_limit_mb, 512);
    assert!(!config.network_enabled);
    assert!(config.read_only_filesystem);
    assert_eq!(config.denied_paths.len(), 3);
}

#[test]
fn test_rule_id_clone() {
    let id1 = RuleId::from_string(uuid::Uuid::new_v4().to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}

#[test]
fn test_sandbox_config_serialization() {
    let config = SandboxConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let parsed: SandboxConfig = serde_json::from_str(&json).unwrap();
    assert!(parsed.enabled);
}
