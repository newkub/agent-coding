use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Guardrail entity - safety and compliance rules for AI operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Guardrail {
    pub id: String,
    pub name: String,
    pub description: String,
    pub guardrail_type: GuardrailType,
    pub rules: Vec<GuardrailRule>,
    pub severity: Severity,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GuardrailType {
    InputValidation,
    OutputFiltering,
    ContentModeration,
    SecurityCheck,
    PermissionCheck,
    RateLimiting,
    DataPrivacy,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuardrailRule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub pattern: Option<String>,
    pub action: GuardrailAction,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleType {
    PatternMatch,
    KeywordDetection,
    LengthCheck,
    FormatValidation,
    ContentClassification,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GuardrailAction {
    Allow,
    Block,
    Warn,
    Modify,
    Escalate,
}

/// Guardrail check result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuardrailCheck {
    pub guardrail_id: String,
    pub guardrail_name: String,
    pub passed: bool,
    pub violations: Vec<GuardrailViolation>,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuardrailViolation {
    pub rule_id: String,
    pub rule_name: String,
    pub message: String,
    pub severity: Severity,
    pub suggested_action: String,
}

impl Guardrail {
    pub fn new(name: String, guardrail_type: GuardrailType, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            guardrail_type,
            rules: Vec::new(),
            severity: Severity::Medium,
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_rules(mut self, rules: Vec<GuardrailRule>) -> Self {
        self.rules = rules;
        self
    }

    pub const fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn has_critical_rules(&self) -> bool {
        self.rules.iter().any(|r| {
            matches!(r.action, GuardrailAction::Block | GuardrailAction::Escalate)
        })
    }
}

impl GuardrailRule {
    pub fn new(name: String, rule_type: RuleType, action: GuardrailAction) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            rule_type,
            pattern: None,
            action,
            enabled: true,
        }
    }

    pub fn with_pattern(mut self, pattern: String) -> Self {
        self.pattern = Some(pattern);
        self
    }
}

impl GuardrailCheck {
    pub fn new(guardrail_id: String, guardrail_name: String) -> Self {
        Self {
            guardrail_id,
            guardrail_name,
            passed: true,
            violations: Vec::new(),
            checked_at: Utc::now(),
        }
    }

    pub fn add_violation(&mut self, violation: GuardrailViolation) {
        self.passed = false;
        self.violations.push(violation);
    }

    pub fn has_critical_violations(&self) -> bool {
        self.violations.iter().any(|v| matches!(v.severity, Severity::Critical))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guardrail_creation() {
        let guardrail = Guardrail::new(
            "Content Filter".to_string(),
            GuardrailType::ContentModeration,
            "Filters inappropriate content".to_string(),
        );
        assert_eq!(guardrail.name, "Content Filter");
        assert!(guardrail.is_enabled());
    }

    #[test]
    fn test_guardrail_rule_creation() {
        let rule = GuardrailRule::new(
            "Keyword Check".to_string(),
            RuleType::KeywordDetection,
            GuardrailAction::Block,
        );
        assert_eq!(rule.name, "Keyword Check");
    }

    #[test]
    fn test_guardrail_check() {
        let check = GuardrailCheck::new("guard-1".to_string(), "Test Guardrail".to_string());
        assert!(check.passed);
        assert!(check.violations.is_empty());
    }

    #[test]
    fn test_guardrail_check_add_violation() {
        let mut check = GuardrailCheck::new("guard-1".to_string(), "Test Guardrail".to_string());
        let violation = GuardrailViolation {
            rule_id: "rule-1".to_string(),
            rule_name: "Test Rule".to_string(),
            message: "Violation detected".to_string(),
            severity: Severity::High,
            suggested_action: "Remove content".to_string(),
        };
        check.add_violation(violation);
        assert!(!check.passed);
    }
}
