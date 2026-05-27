use std::path::PathBuf;

use crate::modules::onboarding::domain::models::codebase_analysis::{CodebaseAnalysis, EntryPoint, EntryPointType};
use crate::modules::onboarding::domain::operations::analysis_operations::{
    calculate_language_distribution, 
    infer_tech_stack_from_deps
};
use crate::modules::onboarding::domain::validators::analysis_validators;
use crate::modules::onboarding::ports::{CodebaseAnalyzer, DependencyParser, FileScanner};
use crate::shared::kernel::result::AppError;

/// Use case for analyzing a codebase
pub struct AnalyzeCodebaseUseCase<S, P>
where
    S: FileScanner,
    P: DependencyParser,
{
    scanner: S,
    parser: P,
}

impl<S, P> AnalyzeCodebaseUseCase<S, P>
where
    S: FileScanner,
    P: DependencyParser,
{
    pub(crate) const fn new(scanner: S, parser: P) -> Self {
        Self { scanner, parser }
    }

    /// Execute the use case to analyze a codebase
    pub(crate) async fn execute(&self, project_path: PathBuf) -> Result<CodebaseAnalysis, AppError> {
        // Validate project path
        analysis_validators::validate_project_path(&project_path)?;

        // Scan directory structure
        let mut structure = self.scanner.scan_directory(&project_path).await?;

        // Parse dependencies
        let dependencies = self.parser.parse_dependencies(&project_path).await?;

        // Calculate language distribution
        calculate_language_distribution(&mut structure);

        // Infer tech stack from dependencies
        let tech_stack = infer_tech_stack_from_deps(&dependencies);

        // Identify entry points
        let entry_points = self.identify_entry_points(&structure, &project_path);

        // Detect test setup
        let test_setup = self.detect_test_setup(&structure);

        // Create analysis
        let mut analysis = CodebaseAnalysis::new(project_path);
        analysis.structure = structure;
        analysis.dependencies = dependencies;
        analysis.tech_stack = tech_stack;
        analysis.entry_points = entry_points;
        analysis.test_setup = test_setup;

        // Generate summary
        analysis.generate_summary();

        // Validate completeness
        analysis_validators::validate_analysis_completeness(&analysis)?;

        Ok(analysis)
    }

    fn identify_entry_points(
        &self,
        _structure: &crate::modules::onboarding::domain::models::codebase_analysis::ProjectStructure,
        project_path: &PathBuf,
    ) -> Vec<EntryPoint> {
        let mut entry_points = Vec::new();

        // Check for common entry point files
        let common_entries = vec![
            ("main.rs", EntryPointType::Main, "Rust main entry point"),
            ("main.js", EntryPointType::Main, "JavaScript main entry point"),
            ("index.js", EntryPointType::Main, "JavaScript index entry point"),
            ("app.py", EntryPointType::Main, "Python main entry point"),
            ("main.go", EntryPointType::Main, "Go main entry point"),
            ("lib.rs", EntryPointType::Library, "Rust library entry point"),
            ("mod.rs", EntryPointType::Library, "Rust module entry point"),
            ("server.js", EntryPointType::Server, "Node.js server entry point"),
            ("cli.rs", EntryPointType::CLI, "Rust CLI entry point"),
        ];

        for (filename, type_, description) in &common_entries {
            let file_path = project_path.join(filename);
            if file_path.exists() {
                entry_points.push(EntryPoint {
                    path: file_path,
                    name: filename.to_string(),
                    type_: type_.clone(),
                    description: description.to_string(),
                });
            }
        }

        // Check in src directory
        let src_path = project_path.join("src");
        if src_path.exists() {
            for (filename, type_, description) in &common_entries {
                let file_path = src_path.join(filename);
                if file_path.exists() {
                    entry_points.push(EntryPoint {
                        path: file_path,
                        name: format!("src/{}", filename),
                        type_: type_.clone(),
                        description: description.to_string(),
                    });
                }
            }
        }

        entry_points
    }

    fn detect_test_setup(
        &self,
        structure: &crate::modules::onboarding::domain::models::codebase_analysis::ProjectStructure,
    ) -> crate::modules::onboarding::domain::models::codebase_analysis::TestSetup {
        let mut test_setup = crate::modules::onboarding::domain::models::codebase_analysis::TestSetup::default();

        // Check for test directories
        let test_dirs = structure.directories.iter().filter(|d| {
            let name_lower = d.name.to_lowercase();
            name_lower.contains("test") || name_lower.contains("spec")
        });

        for dir in test_dirs {
            test_setup.test_directories.push(dir.path.clone());
        }

        // Check for test files
        if !test_setup.test_directories.is_empty() || structure.total_files > 0 {
            test_setup.has_tests = true;
        }

        // Infer test framework from structure
        if structure.root_files.iter().any(|f| f == "package.json") {
            test_setup.test_framework = Some("jest".to_string());
            test_setup.test_command = Some("npm test".to_string());
        } else if structure.root_files.iter().any(|f| f == "Cargo.toml") {
            test_setup.test_framework = Some("cargo test".to_string());
            test_setup.test_command = Some("cargo test".to_string());
        } else if structure.root_files.iter().any(|f| f == "requirements.txt") {
            test_setup.test_framework = Some("pytest".to_string());
            test_setup.test_command = Some("pytest".to_string());
        }

        test_setup
    }
}

#[async_trait::async_trait]
impl<S, P> CodebaseAnalyzer for AnalyzeCodebaseUseCase<S, P>
where
    S: FileScanner,
    P: DependencyParser,
{
    async fn analyze(&self, project_path: PathBuf) -> Result<CodebaseAnalysis, AppError> {
        self.execute(project_path).await
    }

    async fn generate_summary(&self, analysis: &CodebaseAnalysis) -> Result<String, AppError> {
        Ok(analysis.summary.clone())
    }

    async fn save_analysis(&self, analysis: &CodebaseAnalysis, path: &PathBuf) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(analysis)
            .map_err(|e| AppError::State(format!("Failed to serialize analysis: {}", e)))?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    async fn load_analysis(&self, path: &PathBuf) -> Result<CodebaseAnalysis, AppError> {
        let content = tokio::fs::read_to_string(path).await?;
        let analysis: CodebaseAnalysis = serde_json::from_str(&content)
            .map_err(|e| AppError::State(format!("Failed to deserialize analysis: {}", e)))?;
        Ok(analysis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::external::file_scanner::DefaultFileScanner;
    use crate::adapters::external::dependency_parser::DefaultDependencyParser;

    #[tokio::test]
    async fn test_analyze_codebase_use_case_creation() {
        let scanner = DefaultFileScanner::new();
        let parser = DefaultDependencyParser::new();
        let use_case = AnalyzeCodebaseUseCase::new(scanner, parser);
        
        // Test with a non-existent path should fail
        let result = use_case.execute(PathBuf::from("/nonexistent")).await;
        assert!(result.is_err());
    }
}
