use agent_tui::modules::onboarding::domain::validators::analysis_validators;
use agent_tui::modules::onboarding::domain::models::codebase_analysis::CodebaseAnalysis;
use std::path::PathBuf;

#[test]
fn test_validate_project_path_not_exists() {
    let result = analysis_validators::validate_project_path(&PathBuf::from("/nonexistent"));
    assert!(result.is_err());
}

#[test]
fn test_validate_project_path_exists() {
    let result = analysis_validators::validate_project_path(&PathBuf::from("."));
    assert!(result.is_ok());
}

#[test]
fn test_validate_analysis_completeness_incomplete() {
    let analysis = CodebaseAnalysis::new(PathBuf::from("/test"));
    let result = analysis_validators::validate_analysis_completeness(&analysis);
    assert!(result.is_err());
}

#[test]
fn test_validate_dependencies_empty() {
    let mut deps = agent_tui::modules::onboarding::domain::models::codebase_analysis::Dependencies::default();
    deps.package_manager = "cargo".to_string();
    deps.dependencies.insert("serde".to_string(), agent_tui::modules::onboarding::domain::models::codebase_analysis::DependencyInfo {
        version: "1.0".to_string(),
        description: Some("Serialization framework".to_string()),
        category: "serialization".to_string(),
    });
    let result = analysis_validators::validate_dependencies(&deps);
    assert!(result.is_ok());
}
