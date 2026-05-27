use crate::modules::onboarding::domain::models::codebase_analysis::CodebaseAnalysis;
use crate::shared::kernel::result::AppError;
use std::path::Path;

/// Pure function to validate project path
pub fn validate_project_path(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        return Err(AppError::NotFound(format!(
            "Project path does not exist: {}",
            path.display()
        )));
    }

    if !path.is_dir() {
        return Err(AppError::ValidationError(
            "Project path must be a directory".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate analysis completeness
pub fn validate_analysis_completeness(analysis: &CodebaseAnalysis) -> Result<(), AppError> {
    if analysis.structure.total_files == 0 {
        return Err(AppError::ValidationError(
            "Analysis found no files in project".to_string(),
        ));
    }

    if analysis.entry_points.is_empty() {
        return Err(AppError::ValidationError(
            "Analysis found no entry points".to_string(),
        ));
    }

    if analysis.tech_stack.languages.is_empty() {
        return Err(AppError::ValidationError(
            "Analysis could not detect programming languages".to_string(),
        ));
    }

    Ok(())
}

/// Pure function to validate dependency data
pub fn validate_dependencies(
    dependencies: &crate::modules::onboarding::domain::models::codebase_analysis::Dependencies,
) -> Result<(), AppError> {
    if dependencies.package_manager == "unknown" {
        return Err(AppError::ValidationError(
            "Could not detect package manager".to_string(),
        ));
    }

    if dependencies.dependencies.is_empty() && dependencies.dev_dependencies.is_empty() {
        return Err(AppError::ValidationError(
            "No dependencies found in project".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_project_path_not_exists() {
        let path = PathBuf::from("/nonexistent/path");
        assert!(validate_project_path(&path).is_err());
    }

    #[test]
    fn test_validate_project_path_not_directory() {
        let path = PathBuf::from("/etc/passwd"); // Assume this exists and is a file
        if path.exists() && !path.is_dir() {
            assert!(validate_project_path(&path).is_err());
        }
    }

    #[test]
    fn test_validate_analysis_completeness_no_files() {
        let mut analysis = CodebaseAnalysis::new(PathBuf::from("/test"));
        assert!(validate_analysis_completeness(&analysis).is_err());
    }

    #[test]
    fn test_validate_analysis_completeness_success() {
        let mut analysis = CodebaseAnalysis::new(PathBuf::from("/test"));
        analysis.structure.total_files = 100;
        analysis.entry_points.push(crate::modules::onboarding::domain::models::codebase_analysis::EntryPoint {
            path: PathBuf::from("/src/main.rs"),
            name: "main".to_string(),
            type_: crate::modules::onboarding::domain::models::codebase_analysis::EntryPointType::Main,
            description: "Main entry point".to_string(),
        });
        analysis.tech_stack.languages.push("Rust".to_string());
        
        assert!(validate_analysis_completeness(&analysis).is_ok());
    }
}
