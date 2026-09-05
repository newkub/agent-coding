use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

use crate::modules::onboarding::domain::models::codebase_analysis::{Dependencies, DependencyInfo};
use crate::modules::onboarding::ports::DependencyParser;
use crate::shared::kernel::result::AppError;

/// Default implementation for dependency parsing
pub(crate) struct DefaultDependencyParser;

impl DefaultDependencyParser {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl Default for DefaultDependencyParser {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DependencyParser for DefaultDependencyParser {
    async fn parse_package_json(&self, path: &Path) -> Result<Dependencies, AppError> {
        let content = fs::read_to_string(path).await?;
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| AppError::State(format!("Invalid package.json: {}", e)))?;

        let mut dependencies = Dependencies {
            package_manager: "npm".to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        };

        if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
            for (name, value) in deps {
                if let Some(version) = value.as_str() {
                    dependencies.dependencies.insert(
                        name.clone(),
                        DependencyInfo {
                            version: version.to_string(),
                            description: None,
                            category: "runtime".to_string(),
                        },
                    );
                }
            }
        }

        if let Some(deps) = json.get("devDependencies").and_then(|d| d.as_object()) {
            for (name, value) in deps {
                if let Some(version) = value.as_str() {
                    dependencies.dev_dependencies.insert(
                        name.clone(),
                        DependencyInfo {
                            version: version.to_string(),
                            description: None,
                            category: "development".to_string(),
                        },
                    );
                }
            }
        }

        Ok(dependencies)
    }

    async fn parse_cargo_toml(&self, path: &Path) -> Result<Dependencies, AppError> {
        let content = fs::read_to_string(path).await?;
        let toml: toml::Value = toml::from_str(&content)
            .map_err(|e| AppError::State(format!("Invalid Cargo.toml: {}", e)))?;

        let mut dependencies = Dependencies {
            package_manager: "cargo".to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        };

        if let Some(deps) = toml.get("dependencies").and_then(|d| d.as_table()) {
            for (name, value) in deps {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(table) => {
                        if let Some(version) = table.get("version").and_then(|v| v.as_str()) {
                            version.to_string()
                        } else {
                            "unknown".to_string()
                        }
                    }
                    _ => "unknown".to_string(),
                };

                dependencies.dependencies.insert(
                    name.clone(),
                    DependencyInfo {
                        version,
                        description: None,
                        category: "runtime".to_string(),
                    },
                );
            }
        }

        if let Some(deps) = toml.get("dev-dependencies").and_then(|d| d.as_table()) {
            for (name, value) in deps {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(table) => {
                        if let Some(version) = table.get("version").and_then(|v| v.as_str()) {
                            version.to_string()
                        } else {
                            "unknown".to_string()
                        }
                    }
                    _ => "unknown".to_string(),
                };

                dependencies.dev_dependencies.insert(
                    name.clone(),
                    DependencyInfo {
                        version,
                        description: None,
                        category: "development".to_string(),
                    },
                );
            }
        }

        Ok(dependencies)
    }

    async fn parse_requirements_txt(&self, path: &Path) -> Result<Dependencies, AppError> {
        let content = fs::read_to_string(path).await?;

        let mut dependencies = Dependencies {
            package_manager: "pip".to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse requirement (simplified)
            let parts: Vec<&str> = line.split(['=', '>', '<', '~']).collect();
            if let Some(name) = parts.first() {
                let name = name.trim();
                if !name.is_empty() {
                    dependencies.dependencies.insert(
                        name.to_string(),
                        DependencyInfo {
                            version: "latest".to_string(),
                            description: None,
                            category: "runtime".to_string(),
                        },
                    );
                }
            }
        }

        Ok(dependencies)
    }

    async fn parse_dependencies(&self, project_path: &Path) -> Result<Dependencies, AppError> {
        // Try package.json first
        let package_json = project_path.join("package.json");
        if package_json.exists() {
            return self.parse_package_json(&package_json).await;
        }

        // Try Cargo.toml
        let cargo_toml = project_path.join("Cargo.toml");
        if cargo_toml.exists() {
            return self.parse_cargo_toml(&cargo_toml).await;
        }

        // Try requirements.txt
        let requirements_txt = project_path.join("requirements.txt");
        if requirements_txt.exists() {
            return self.parse_requirements_txt(&requirements_txt).await;
        }

        // Try go.mod
        let go_mod = project_path.join("go.mod");
        if go_mod.exists() {
            return Ok(Dependencies {
                package_manager: "go".to_string(),
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
            });
        }

        Err(AppError::NotFound(
            "No supported dependency file found".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_requirements_txt() {
        let parser = DefaultDependencyParser::new();
        let content = "requests==2.28.0\nnumpy>=1.20.0\n# comment\nflask";

        let temp_dir = tempfile::tempdir().unwrap();
        let req_file = temp_dir.path().join("requirements.txt");
        tokio::fs::write(&req_file, content).await.unwrap();

        let result = parser.parse_requirements_txt(&req_file).await;
        assert!(result.is_ok());

        let deps = result.unwrap();
        assert!(deps.dependencies.contains_key("requests"));
        assert!(deps.dependencies.contains_key("numpy"));
        assert!(deps.dependencies.contains_key("flask"));
        assert!(!deps.dependencies.contains_key("comment"));
    }
}
