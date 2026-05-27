# Guardrails Domain Tests

## Guardrail Models
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_guardrail_creation | Guardrail created with name and type | `Guardrail::new("test", GuardrailType::Security)` |
| ✅ | test_guardrail_rule_creation | Rule created with name, type, and action | `GuardrailRule::new("test", RuleType::Pattern, RuleAction::Block)` |
| ✅ | test_guardrail_check_creation | Check created with ID and guardrail name, passed by default | `GuardrailCheck::new("check-1", "test", true)` |

## Guardrail Operations
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_check_guardrail_pass | Safe input passes guardrail check | `check_guardrail(&guardrail, "safe input")` |
| ✅ | test_should_take_action | Passed check returns Allow action | `should_take_action(&check)` |
| ✅ | test_filter_output | Output with "password" pattern is redacted | `filter_output("password: 123", &rules)` |

## Guardrail Validators
| Status | Description | Expected Output | Expected examples |
|--------|-------------|-----------------|-------------------|
| ✅ | test_validate_guardrail_empty_name | Guardrail with empty name fails validation | `validate_guardrail(&guardrail)` |
| ✅ | test_validate_guardrail_success | Valid guardrail passes validation | `validate_guardrail(&valid_guardrail)` |
| ✅ | test_validate_guardrail_rule_empty_name | Rule with empty name fails validation | `validate_guardrail_rule(&rule)` |
| ✅ | test_validate_guardrail_rule_success | Valid rule passes validation | `validate_guardrail_rule(&valid_rule)` |
