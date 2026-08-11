use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;
use walkdir::WalkDir;

use crate::modules::onboarding::domain::models::codebase_analysis::{
    DirectoryInfo, ProjectStructure,
};
use crate::modules::onboarding::ports::FileScanner;
use crate::shared::kernel::result::AppError;

/// Default implementation for file system scanning
pub(crate) struct DefaultFileScanner;

impl DefaultFileScanner {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl Default for DefaultFileScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FileScanner for DefaultFileScanner {
    async fn scan_directory(&self, path: &PathBuf) -> Result<ProjectStructure, AppError> {
        let mut structure = ProjectStructure::default();
        let mut total_lines = 0;
        let mut language_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        let mut total_files = 0;

        // Get root files
        let mut entries = fs::read_dir(path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Some(name) = entry_path.file_name() {
                    structure
                        .root_files
                        .push(name.to_string_lossy().to_string());
                }
            }
        }

        // Walk directory tree
        for entry in WalkDir::new(path)
            .min_depth(1)
            .max_depth(10)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_file() {
                total_files += 1;

                // Count lines
                let path_buf = entry_path.to_path_buf();
                if let Ok(lines) = self.count_lines(&path_buf).await {
                    total_lines += lines;
                }

                // Detect language
                if let Some(lang) = self.detect_language(&path_buf) {
                    *language_counts.entry(lang).or_insert(0) += 1;
                }
            } else if entry_path.is_dir() {
                let dir_name = entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let file_count = WalkDir::new(entry_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                    .count();

                if file_count > 0 {
                    structure.directories.push(DirectoryInfo {
                        path: entry_path.to_path_buf(),
                        name: dir_name.clone(),
                        file_count,
                        purpose: self.infer_directory_purpose(&dir_name),
                    });
                }
            }
        }

        structure.total_files = total_files;
        structure.total_lines = total_lines;

        // Calculate language percentages
        if total_files > 0 {
            for (lang, count) in language_counts {
                structure
                    .languages
                    .insert(lang, (count as f64 / total_files as f64) * 100.0);
            }
        }

        Ok(structure)
    }

    async fn count_lines(&self, path: &PathBuf) -> Result<usize, AppError> {
        let content = fs::read_to_string(path).await?;
        Ok(content.lines().count())
    }

    fn detect_language(&self, file_path: &PathBuf) -> Option<String> {
        let extension = file_path.extension()?.to_str()?;

        match extension.to_lowercase().as_str() {
            "rs" => Some("Rust".to_string()),
            "js" | "jsx" | "mjs" => Some("JavaScript".to_string()),
            "ts" | "tsx" => Some("TypeScript".to_string()),
            "py" => Some("Python".to_string()),
            "go" => Some("Go".to_string()),
            "java" => Some("Java".to_string()),
            "kt" | "kts" => Some("Kotlin".to_string()),
            "rb" => Some("Ruby".to_string()),
            "php" => Some("PHP".to_string()),
            "c" | "h" => Some("C".to_string()),
            "cpp" | "hpp" | "cc" | "cxx" => Some("C++".to_string()),
            "cs" => Some("C#".to_string()),
            "swift" => Some("Swift".to_string()),
            "scala" => Some("Scala".to_string()),
            "sh" | "bash" => Some("Shell".to_string()),
            "sql" => Some("SQL".to_string()),
            "html" | "htm" => Some("HTML".to_string()),
            "css" | "scss" | "sass" => Some("CSS".to_string()),
            "json" => Some("JSON".to_string()),
            "yaml" | "yml" => Some("YAML".to_string()),
            "toml" => Some("TOML".to_string()),
            "xml" => Some("XML".to_string()),
            "md" => Some("Markdown".to_string()),
            _ => None,
        }
    }
}

impl DefaultFileScanner {
    fn infer_directory_purpose(&self, name: &str) -> String {
        let name_lower = name.to_lowercase();

        match name_lower.as_str() {
            "src" => "Source code".to_string(),
            "lib" => "Library code".to_string(),
            "app" => "Application code".to_string(),
            "components" => "UI components".to_string(),
            "pages" => "Page components".to_string(),
            "api" => "API endpoints".to_string(),
            "server" => "Server code".to_string(),
            "client" => "Client code".to_string(),
            "tests" | "test" | "__tests__" => "Test files".to_string(),
            "docs" | "documentation" => "Documentation".to_string(),
            "examples" => "Example code".to_string(),
            "scripts" => "Build scripts".to_string(),
            "config" | "configuration" => "Configuration files".to_string(),
            "assets" | "static" | "public" => "Static assets".to_string(),
            "build" | "dist" | "target" => "Build output".to_string(),
            "node_modules" => "Node dependencies".to_string(),
            "vendor" => "Third-party dependencies".to_string(),
            _ => "Directory".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language_rust() {
        let scanner = DefaultFileScanner::new();
        let path = PathBuf::from("/test/main.rs");
        assert_eq!(scanner.detect_language(&path), Some("Rust".to_string()));
    }

    #[test]
    fn test_detect_language_typescript() {
        let scanner = DefaultFileScanner::new();
        let path = PathBuf::from("/test/app.tsx");
        assert_eq!(
            scanner.detect_language(&path),
            Some("TypeScript".to_string())
        );
    }

    #[test]
    fn test_detect_language_unknown() {
        let scanner = DefaultFileScanner::new();
        let path = PathBuf::from("/test/file.xyz");
        assert_eq!(scanner.detect_language(&path), None);
    }

    #[test]
    fn test_infer_directory_purpose() {
        let scanner = DefaultFileScanner::new();
        assert_eq!(scanner.infer_directory_purpose("src"), "Source code");
        assert_eq!(scanner.infer_directory_purpose("tests"), "Test files");
        assert_eq!(scanner.infer_directory_purpose("unknown"), "Directory");
    }
}
