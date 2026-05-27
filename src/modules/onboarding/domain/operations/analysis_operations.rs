use crate::modules::onboarding::domain::models::codebase_analysis::{ProjectStructure, TechStack};

/// Pure function to detect project type from structure
pub fn detect_project_type(structure: &ProjectStructure) -> String {
    let root_files_lower: Vec<String> = structure
        .root_files
        .iter()
        .map(|f| f.to_lowercase())
        .collect();

    if root_files_lower.iter().any(|f| f == "package.json") {
        return "Node.js/JavaScript".to_string();
    }
    if root_files_lower.iter().any(|f| f == "cargo.toml") {
        return "Rust".to_string();
    }
    if root_files_lower.iter().any(|f| f == "go.mod") {
        return "Go".to_string();
    }
    if root_files_lower.iter().any(|f| f == "pom.xml") {
        return "Java/Maven".to_string();
    }
    if root_files_lower.iter().any(|f| f == "requirements.txt" || f == "pyproject.toml") {
        return "Python".to_string();
    }
    if root_files_lower.iter().any(|f| f == "gemfile") {
        return "Ruby".to_string();
    }
    if root_files_lower.iter().any(|f| f == "composer.json") {
        return "PHP".to_string();
    }

    "Unknown".to_string()
}

/// Pure function to estimate complexity from structure
pub fn estimate_complexity(structure: &ProjectStructure) -> ComplexityLevel {
    let file_count = structure.total_files;
    let dir_count = structure.directories.len();
    let language_count = structure.languages.len();

    match (file_count, dir_count, language_count) {
        (f, d, l) if f < 50 && d < 5 && l <= 1 => ComplexityLevel::Simple,
        (f, d, l) if f < 200 && d < 15 && l <= 3 => ComplexityLevel::Medium,
        (f, d, l) if f < 1000 && d < 50 && l <= 5 => ComplexityLevel::Complex,
        _ => ComplexityLevel::VeryComplex,
    }
}

/// Pure function to identify main directories
pub fn identify_main_directories(structure: &ProjectStructure) -> Vec<String> {
    let mut main_dirs = Vec::new();
    
    for dir in &structure.directories {
        let name_lower = dir.name.to_lowercase();
        if matches!(
            name_lower.as_str(),
            "src" | "lib" | "app" | "components" | "pages" | "api" | "server" | "client"
        ) {
            main_dirs.push(dir.name.clone());
        }
    }
    
    main_dirs
}

/// Pure function to calculate language distribution
pub fn calculate_language_distribution(structure: &mut ProjectStructure) {
    if structure.total_files == 0 {
        return;
    }
    
    // Normalize percentages to sum to 100%
    let total: f64 = structure.languages.values().sum();
    if total > 0.0 {
        for (_, percentage) in structure.languages.iter_mut() {
            *percentage = (*percentage / total) * 100.0;
        }
    }
}

/// Pure function to infer tech stack from dependencies
pub fn infer_tech_stack_from_deps(
    dependencies: &crate::modules::onboarding::domain::models::codebase_analysis::Dependencies,
) -> TechStack {
    let mut tech_stack = TechStack::default();
    
    // Analyze dependencies to infer frameworks and tools
    for name in dependencies.dependencies.keys() {
        let name_lower = name.to_lowercase();
        
        // Frameworks
        if name_lower.contains("react") || name_lower.contains("next") {
            tech_stack.frameworks.push("React".to_string());
        } else if name_lower.contains("vue") {
            tech_stack.frameworks.push("Vue".to_string());
        } else if name_lower.contains("angular") {
            tech_stack.frameworks.push("Angular".to_string());
        } else if name_lower.contains("actix") || name_lower.contains("rocket") {
            tech_stack.frameworks.push("Rust Web".to_string());
        } else if name_lower.contains("express") || name_lower.contains("koa") {
            tech_stack.frameworks.push("Node.js Web".to_string());
        }
        
        // Databases
        if name_lower.contains("postgres") || name_lower.contains("pg") {
            tech_stack.databases.push("PostgreSQL".to_string());
        } else if name_lower.contains("mysql") {
            tech_stack.databases.push("MySQL".to_string());
        } else if name_lower.contains("mongo") {
            tech_stack.databases.push("MongoDB".to_string());
        } else if name_lower.contains("sqlite") {
            tech_stack.databases.push("SQLite".to_string());
        } else if name_lower.contains("redis") {
            tech_stack.databases.push("Redis".to_string());
        }
        
        // Build tools
        if name_lower.contains("webpack") || name_lower.contains("vite") || name_lower.contains("rollup") {
            tech_stack.build_tools.push(name.clone());
        }
        
        // Testing
        if name_lower.contains("jest") || name_lower.contains("mocha") || name_lower.contains("pytest") {
            tech_stack.testing_frameworks.push(name.clone());
        }
    }
    
    // Remove duplicates
    tech_stack.frameworks.sort();
    tech_stack.frameworks.dedup();
    tech_stack.databases.sort();
    tech_stack.databases.dedup();
    tech_stack.build_tools.sort();
    tech_stack.build_tools.dedup();
    tech_stack.testing_frameworks.sort();
    tech_stack.testing_frameworks.dedup();
    
    tech_stack
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityLevel {
    Simple,
    Medium,
    Complex,
    VeryComplex,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_project_type_rust() {
        let mut structure = ProjectStructure::default();
        structure.root_files.push("Cargo.toml".to_string());
        
        assert_eq!(detect_project_type(&structure), "Rust");
    }

    #[test]
    fn test_detect_project_type_nodejs() {
        let mut structure = ProjectStructure::default();
        structure.root_files.push("package.json".to_string());
        
        assert_eq!(detect_project_type(&structure), "Node.js/JavaScript");
    }

    #[test]
    fn test_estimate_complexity_simple() {
        let mut structure = ProjectStructure::default();
        structure.total_files = 30;
        structure.directories.push(crate::modules::onboarding::domain::models::codebase_analysis::DirectoryInfo {
            path: PathBuf::from("/src"),
            name: "src".to_string(),
            file_count: 10,
            purpose: "source".to_string(),
        });
        
        assert_eq!(estimate_complexity(&structure), ComplexityLevel::Simple);
    }

    #[test]
    fn test_estimate_complexity_complex() {
        let mut structure = ProjectStructure::default();
        structure.total_files = 500;
        structure.languages.insert("Rust".to_string(), 50.0);
        structure.languages.insert("JavaScript".to_string(), 30.0);
        structure.languages.insert("TypeScript".to_string(), 20.0);
        
        for i in 0..20 {
            structure.directories.push(crate::modules::onboarding::domain::models::codebase_analysis::DirectoryInfo {
                path: PathBuf::from(format!("/dir{}", i)),
                name: format!("dir{}", i),
                file_count: 25,
                purpose: "test".to_string(),
            });
        }
        
        assert_eq!(estimate_complexity(&structure), ComplexityLevel::Complex);
    }
}
