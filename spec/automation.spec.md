# Automation Domain Tests

## Issue PR Models
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_issue_creation | Issue created with number 1 and Open state | `Issue::new(1, "Test Issue", "Description", "user", "owner/repo")` |
| ✅ | test_issue_automatable_with_label | Issue with "automated" label is automatable | `issue.labels.push("automated".to_string())` |
| ✅ | test_pull_request_creation | PR created with number 1 and Open state | `PullRequest::new(1, "Test PR", "Description", "user", "feature/test", "main", "owner/repo")` |
| ✅ | test_automation_workflow_creation | Workflow created with Pending status | `AutomationWorkflow::new(issue)` |
| ✅ | test_automation_config_default | Config has auto_create_branch and auto_commit enabled | `AutomationConfig::default()` |

## Automation Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_generate_branch_name | Branch name contains issue number and formatted title | `generate_branch_name(&issue, &config)` |
| ✅ | test_generate_commit_message | Commit message contains title and "Closes #1" | `generate_commit_message(&issue, &config)` |
| ✅ | test_generate_pr_title | PR title format is "Test Issue (#1)" | `generate_pr_title(&issue)` |
| ✅ | test_extract_labels | Labels include "bug" and "automated" | `extract_labels(&issue, &config)` |
| ✅ | test_determine_target_branch | Target branch matches label "develop" | `determine_target_branch(&issue)` |

## Automation Validators
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_validate_issue_for_automation_success | Issue with "/automate" in description passes validation | `validate_issue_for_automation(&issue)` |
| ✅ | test_validate_issue_for_automation_closed | Closed issue fails validation | `issue.state = IssueState::Closed` |
| ✅ | test_validate_automation_config_success | Default config passes validation | `validate_automation_config(&config)` |
| ✅ | test_validate_automation_config_missing_placeholder | Config without placeholders fails validation | `config.branch_name_template = "feature/test"` |
| ✅ | test_validate_repository_access_success | Valid "owner/repo" format passes validation | `validate_repository_access("owner/repo")` |
| ✅ | test_validate_repository_access_invalid | Invalid format fails validation | `validate_repository_access("invalid")` |
