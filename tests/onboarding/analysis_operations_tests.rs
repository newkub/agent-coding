use agent_tui::modules::onboarding::domain::models::codebase_analysis::{ProjectStructure, TechStack, Dependencies};
use agent_tui::modules::onboarding::domain::operations::analysis_operations::{
    detect_project_type, estimate_complexity, identify_main_directories, calculate_language_distribution,
    infer_tech_stack_from_deps, ComplexityLevel
};

#[test]
fn test_detect_project_type_rust() {
    let mut structure = ProjectStructure::default();
    structure.root_files.push("Cargo.toml".to_string());
    let project_type = detect_project_type(&structure);
    assert!(project_type.contains("Rust"));
}

#[test]
fn test_detect_project_type_javascript() {
    let mut structure = ProjectStructure::default();
    structure.root_files.push("package.json".to_string());
    let project_type = detect_project_type(&structure);
    assert!(project_type.contains("JavaScript") || project_type.contains("Node"));
}

#[test]
fn test_estimate_complexity_small() {
    let mut structure = ProjectStructure::default();
    structure.total_files = 10;
    structure.total_lines = 100;
    let complexity = estimate_complexity(&structure);
    assert_eq!(complexity, ComplexityLevel::Simple);
}

#[test]
fn test_estimate_complexity_large() {
    let mut structure = ProjectStructure::default();
    structure.total_files = 1000;
    structure.total_lines = 100000;
    let complexity = estimate_complexity(&structure);
    assert_eq!(complexity, ComplexityLevel::VeryComplex);
}

#[test]
fn test_calculate_language_distribution() {
    let mut structure = ProjectStructure::default();
    structure.total_files = 10;
    structure.languages.insert("Rust".to_string(), 70.0);
    structure.languages.insert("JavaScript".to_string(), 30.0);
    
    calculate_language_distribution(&mut structure);
    // Verify normalization happened (percentages should sum to 100)
    let total: f64 = structure.languages.values().sum();
    assert!((total - 100.0).abs() < 0.01);
}

#[test]
fn test_infer_tech_stack_from_deps() {
    let mut deps = Dependencies::default();
    deps.dependencies.insert("react".to_string(), 
        agent_tui::modules::onboarding::domain::models::codebase_analysis::DependencyInfo {
            version: "18.0.0".to_string(),
            description: None,
            category: "runtime".to_string(),
        });
    
    let stack = infer_tech_stack_from_deps(&deps);
    assert!(stack.frameworks.contains(&"React".to_string()));
}
