# Subagents Domain Tests

## Subagent Models
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_subagent_creation | Subagent created with name and is available | `Subagent::new("CodeReviewer", capabilities)` |
| ✅ | test_subagent_can_handle | CodeReviewer can handle CodeReview tasks, not BugDetection | `subagent.can_handle(&task)` |
| ✅ | test_subagent_task_creation | Task created with Pending status | `SubagentTask::new(subagent, task)` |
| ✅ | test_subagent_task_complete | Task marked as Completed and is_completed returns true | `task.complete()` |
| ✅ | test_task_context_with_session | Context can have session ID | `TaskContext::new(Some("session-1"))` |

## Subagent Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_select_subagent_for_task | Idle subagent selected for matching task type | `select_subagent_for_task(&subagents, &task)` |
| ✅ | test_estimate_task_complexity | BugDetection with 5000 tokens rated as High complexity | `estimate_task_complexity(&task)` |
| ✅ | test_calculate_task_priority | SecurityAudit with Medium complexity rated as Critical priority | `calculate_task_priority(&task)` |
| ✅ | test_generate_system_prompt | System prompt for CodeReviewer contains "code reviewer" | `generate_system_prompt(&subagent)` |

## Subagent Validators
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_validate_subagent_config_invalid_temperature | Temperature > 2.0 fails validation | `validate_subagent_config(&config)` |
| ✅ | test_validate_subagent_config_success | Default config passes validation | `validate_subagent_config(&default_config)` |
| ✅ | test_validate_subagent_empty_name | Subagent with empty name fails validation | `validate_subagent(&subagent)` |
| ✅ | test_validate_subagent_task_empty_input | Task with empty input fails validation | `validate_subagent_task(&task)` |
| ✅ | test_validate_task_context_invalid_repo | Invalid repo format fails validation | `validate_task_context(&context)` |
| ✅ | test_validate_task_context_success | Valid file and repo pass validation | `validate_task_context(&valid_context)` |
