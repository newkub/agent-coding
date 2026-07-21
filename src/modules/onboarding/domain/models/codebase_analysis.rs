use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Codebase analysis result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodebaseAnalysis {
    pub id: String,
    pub project_path: PathBuf,
    pub analyzed_at: DateTime<Utc>,
    pub structure: ProjectStructure,
    pub dependencies: Dependencies,
    pub tech_stack: TechStack,
    pub entry_points: Vec<EntryPoint>,
    pub test_setup: TestSetup,
    pub summary: String,
}

/// Project structure information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectStructure {
    pub root_files: Vec<String>,
    pub directories: Vec<DirectoryInfo>,
    pub total_files: usize,
    pub total_lines: usize,
    pub languages: HashMap<String, f64>, // language -> percentage
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub name: String,
    pub file_count: usize,
    pub purpose: String,
}

/// Dependencies information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependencies {
    pub package_manager: String,
    pub dependencies: HashMap<String, DependencyInfo>,
    pub dev_dependencies: HashMap<String, DependencyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DependencyInfo {
    pub version: String,
    pub description: Option<String>,
    pub category: String,
}

/// Technology stack
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TechStack {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub databases: Vec<String>,
    pub build_tools: Vec<String>,
    pub testing_frameworks: Vec<String>,
}

/// Entry points
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntryPoint {
    pub path: PathBuf,
    pub name: String,
    pub type_: EntryPointType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntryPointType {
    Main,
    Library,
    CLI,
    Server,
    Worker,
}

/// Test setup information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TestSetup {
    pub has_tests: bool,
    pub test_framework: Option<String>,
    pub test_command: Option<String>,
    pub coverage_tool: Option<String>,
    pub test_directories: Vec<PathBuf>,
}

impl CodebaseAnalysis {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_path,
            analyzed_at: Utc::now(),
            structure: ProjectStructure::default(),
            dependencies: Dependencies::default(),
            tech_stack: TechStack::default(),
            entry_points: Vec::new(),
            test_setup: TestSetup::default(),
            summary: String::new(),
        }
    }

    pub fn generate_summary(&mut self) {
        let mut summary = format!("Project at {}\n", self.project_path.display());

        summary.push_str(&format!("Total files: {}\n", self.structure.total_files));
        summary.push_str(&format!("Total lines: {}\n", self.structure.total_lines));

        if !self.structure.languages.is_empty() {
            summary.push_str("Languages: ");
            let langs: Vec<_> = self
                .structure
                .languages
                .keys()
                .map(|s| s.as_str())
                .collect();
            summary.push_str(&langs.join(", "));
            summary.push('\n');
        }

        if !self.tech_stack.frameworks.is_empty() {
            summary.push_str("Frameworks: ");
            summary.push_str(&self.tech_stack.frameworks.join(", "));
            summary.push('\n');
        }

        if !self.entry_points.is_empty() {
            summary.push_str("Entry points: ");
            for ep in &self.entry_points {
                summary.push_str(&format!("{} ({}) ", ep.name, ep.path.display()));
            }
            summary.push('\n');
        }

        if self.test_setup.has_tests {
            summary.push_str(&format!(
                "Tests: {} using {}\n",
                if self.test_setup.has_tests {
                    "Yes"
                } else {
                    "No"
                },
                self.test_setup
                    .test_framework
                    .as_deref()
                    .unwrap_or("unknown")
            ));
        }

        self.summary = summary;
    }
}

impl Default for Dependencies {
    fn default() -> Self {
        Self {
            package_manager: "unknown".to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codebase_analysis_creation() {
        let analysis = CodebaseAnalysis::new(PathBuf::from("/test"));
        assert_eq!(analysis.structure.total_files, 0);
        assert_eq!(analysis.entry_points.len(), 0);
    }

    #[test]
    fn test_generate_summary() {
        let mut analysis = CodebaseAnalysis::new(PathBuf::from("/test"));
        analysis.structure.total_files = 100;
        analysis.structure.total_lines = 10000;
        analysis
            .structure
            .languages
            .insert("Rust".to_string(), 100.0);
        analysis.tech_stack.frameworks.push("Actix".to_string());
        analysis.test_setup.has_tests = true;
        analysis.test_setup.test_framework = Some("cargo test".to_string());

        analysis.generate_summary();

        assert!(analysis.summary.contains("Total files: 100"));
        assert!(analysis.summary.contains("Rust"));
        assert!(analysis.summary.contains("Actix"));
    }
}
