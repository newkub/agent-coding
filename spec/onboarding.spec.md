# Onboarding Domain Tests

## Analysis Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_detect_project_type_rust | Cargo.toml detected as Rust project | `detect_project_type("/path/Cargo.toml")` |
| ✅ | test_detect_project_type_javascript | package.json detected as JavaScript/Node project | `detect_project_type("/path/package.json")` |
| ✅ | test_estimate_complexity_small | Small project (10 files, 100 lines) rated as Simple | `estimate_complexity(10, 100)` |
| ✅ | test_estimate_complexity_large | Large project (1000 files, 100000 lines) rated as VeryComplex | `estimate_complexity(1000, 100000)` |
| ✅ | test_calculate_language_distribution | Language percentages normalized to sum to 100% | `calculate_language_distribution(&stats)` |
| ✅ | test_infer_tech_stack_from_deps | React dependency detected in tech stack | `infer_tech_stack_from_deps(&["react".to_string()])` |

## Analysis Validators
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_validate_project_path_not_exists | Non-existent path fails validation | `validate_project_path("/nonexistent")` |
| ✅ | test_validate_project_path_exists | Existing path passes validation | `validate_project_path("/existing")` |
| ✅ | test_validate_analysis_completeness_incomplete | Incomplete analysis fails validation | `validate_analysis_completeness(&incomplete)` |
| ✅ | test_validate_dependencies_empty | Empty dependencies are valid | `validate_dependencies(&[])` |
