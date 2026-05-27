use async_trait::async_trait;
use std::path::PathBuf;

use crate::modules::onboarding::domain::models::codebase_analysis::{CodebaseAnalysis, Dependencies, ProjectStructure};
use crate::shared::kernel::result::AppError;

/// Port for file system scanning operations
#[async_trait]
pub trait FileScanner: Send + Sync {
    /// Scan project directory and return structure
    async fn scan_directory(&self, path: &PathBuf) -> Result<ProjectStructure, AppError>;
    
    /// Count lines in a file
    async fn count_lines(&self, path: &PathBuf) -> Result<usize, AppError>;
    
    /// Detect language from file extension
    fn detect_language(&self, file_path: &PathBuf) -> Option<String>;
}

/// Port for dependency parsing operations
#[async_trait]
pub trait DependencyParser: Send + Sync {
    /// Parse package.json and return dependencies
    async fn parse_package_json(&self, path: &PathBuf) -> Result<Dependencies, AppError>;
    
    /// Parse Cargo.toml and return dependencies
    async fn parse_cargo_toml(&self, path: &PathBuf) -> Result<Dependencies, AppError>;
    
    /// Parse requirements.txt and return dependencies
    async fn parse_requirements_txt(&self, path: &PathBuf) -> Result<Dependencies, AppError>;
    
    /// Auto-detect and parse dependencies
    async fn parse_dependencies(&self, project_path: &PathBuf) -> Result<Dependencies, AppError>;
}

/// Port for codebase analysis operations
#[async_trait]
pub trait CodebaseAnalyzer: Send + Sync {
    /// Analyze entire codebase
    async fn analyze(&self, project_path: PathBuf) -> Result<CodebaseAnalysis, AppError>;
    
    /// Generate analysis summary
    async fn generate_summary(&self, analysis: &CodebaseAnalysis) -> Result<String, AppError>;
    
    /// Save analysis to file
    async fn save_analysis(&self, analysis: &CodebaseAnalysis, path: &PathBuf) -> Result<(), AppError>;
    
    /// Load analysis from file
    async fn load_analysis(&self, path: &PathBuf) -> Result<CodebaseAnalysis, AppError>;
}
