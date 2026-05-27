# Sandbox Domain Tests

## CommandId
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_command_id_new | Generate non-empty ID | `CommandId::from_string(uuid::Uuid::new_v4().to_string())` |
| ✅ | test_command_id_display | ID can be displayed | `format!("{}", id)` |
| ✅ | test_command_id_clone | ID clones correctly | `id1.clone()` |

## Command
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_command_new | Creates command with safe security level | `Command::new("ls", SecurityLevel::Safe)` |
| ✅ | test_command_with_working_dir | Can set working directory | `command.with_working_dir("/path")` |
| ✅ | test_command_with_env | Can set environment variables | `command.with_env(vec![("KEY", "value")])` |
| ✅ | test_command_serialization | Command serializes/deserializes correctly | `serde_json::to_string(&command)` |

## CommandResult
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_command_result_success | Exit code 0 is success | `CommandResult::success("output")` |
| ✅ | test_command_result_failure | Non-zero exit code is failure | `CommandResult::failure(1, "error")` |
| ✅ | test_command_result_no_exit_code | No exit code is failure | `CommandResult::no_exit_code("output")` |
| ✅ | test_command_result_serialization | Result serializes/deserializes correctly | `serde_json::to_string(&result)` |

## RuleId
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_rule_id_new | Generate non-empty ID | `RuleId::from_string(uuid::Uuid::new_v4().to_string())` |
| ✅ | test_rule_id_clone | ID clones correctly | `id1.clone()` |

## SandboxConfig
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_sandbox_config_default | Default config has safe defaults | `SandboxConfig::default()` |
| ✅ | test_sandbox_config_serialization | Config serializes/deserializes correctly | `serde_json::to_string(&config)` |

## ApprovalRule
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_rule_serialization | Rule serializes/deserializes correctly | `serde_json::to_string(&rule)` |

## Variants
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_security_level_variants | All security level variants match | `assert!(matches!(SecurityLevel::Safe, SecurityLevel::Safe))` |
| ✅ | test_rule_action_variants | All rule action variants match | `assert!(matches!(RuleAction::Allow, RuleAction::Allow))` |

## Sandbox Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_is_safe_command_safe | Safe commands return true | `is_safe_command("ls")` |
| ✅ | test_is_safe_command_dangerous | Dangerous commands return false | `is_safe_command("rm -rf /")` |
| ✅ | test_determine_security_level_safe | Safe commands get Safe level | `determine_security_level("ls")` |
| ✅ | test_determine_security_level_sandboxed | Dangerous commands get Sandboxed level | `determine_security_level("rm")` |
| ✅ | test_validate_command_valid | Valid commands pass validation | `validate_command("ls")` |
| ✅ | test_validate_command_empty | Empty command returns error | `validate_command("")` |
| ✅ | test_validate_command_too_long | Long command returns error | `validate_command("a".repeat(10001))` |
| ✅ | test_validate_command_invalid_chars | Invalid chars return error | `validate_command("ls; rm")` |
| ✅ | test_match_rule_pattern_regex | Regex pattern matching works | `match_rule_pattern("test.*", "test123")` |
| ✅ | test_match_rule_pattern_fallback | Fallback to contains works | `match_rule_pattern("test", "testing")` |
| ✅ | test_match_rule_pattern_invalid_regex | Invalid regex falls back to contains | `match_rule_pattern("[invalid", "test")` |
| ✅ | test_calculate_risk_score_low | Low risk commands have low score | `calculate_risk_score("ls")` |
| ✅ | test_calculate_risk_score_high | High risk commands have high score | `calculate_risk_score("rm -rf /")` |
| ✅ | test_calculate_risk_score_git | Git commands have medium risk | `calculate_risk_score("git push")` |
| ✅ | test_calculate_risk_score_docker | Docker commands have medium risk | `calculate_risk_score("docker run")` |
| ✅ | test_command_validation_error_display | Errors display correctly | `format!("{}", error)` |
