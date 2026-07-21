use agent_tui::modules::onboarding::domain::models::codebase_analysis::{
    CodebaseAnalysis, EntryPoint, EntryPointType, ProjectStructure, TechStack, TestSetup,
};

#[test]
fn test_codebase_analysis_creation() {
    let analysis = CodebaseAnalysis::new(std::path::PathBuf::from("/test"));
    assert_eq!(analysis.project_path, std::path::PathBuf::from("/test"));
    assert!(analysis.structure.total_files == 0);
}

#[test]
fn test_project_structure_default() {
    let structure = ProjectStructure::default();
    assert_eq!(structure.total_files, 0);
    assert!(structure.root_files.is_empty());
}

#[test]
fn test_entry_point_creation() {
    let entry = EntryPoint {
        path: std::path::PathBuf::from("/test/main.rs"),
        name: "main.rs".to_string(),
        type_: EntryPointType::Main,
        description: "Main entry point".to_string(),
    };
    assert_eq!(entry.name, "main.rs");
}

#[test]
fn test_tech_stack_default() {
    let stack = TechStack::default();
    assert!(stack.languages.is_empty());
    assert!(stack.frameworks.is_empty());
}

#[test]
fn test_test_setup_default() {
    let setup = TestSetup::default();
    assert!(!setup.has_tests);
    assert!(setup.test_directories.is_empty());
}
