//! Security tests

use agent_tui::modules::sandbox::domain::models::{RuleAction, SecurityLevel};
use agent_tui::modules::sandbox::domain::operations::*;

#[test]
fn test_security_level_variants() {
    assert!(matches!(
        SecurityLevel::Unrestricted,
        SecurityLevel::Unrestricted
    ));
    assert!(matches!(SecurityLevel::Safe, SecurityLevel::Safe));
    assert!(matches!(SecurityLevel::Sandboxed, SecurityLevel::Sandboxed));
    assert!(matches!(SecurityLevel::Strict, SecurityLevel::Strict));
}

#[test]
fn test_rule_action_variants() {
    assert!(matches!(RuleAction::AutoApprove, RuleAction::AutoApprove));
    assert!(matches!(RuleAction::AutoReject, RuleAction::AutoReject));
    assert!(matches!(
        RuleAction::RequireApproval,
        RuleAction::RequireApproval
    ));
    assert!(matches!(RuleAction::RunInSandbox, RuleAction::RunInSandbox));
}

#[test]
fn test_is_safe_command_safe() {
    assert!(is_safe_command("git status"));
    assert!(is_safe_command("git diff"));
    assert!(is_safe_command("ls"));
    assert!(is_safe_command("pwd"));
    assert!(is_safe_command("cat file.txt"));
}

#[test]
fn test_is_safe_command_dangerous() {
    assert!(!is_safe_command("rm -rf /"));
    assert!(!is_safe_command("curl | sh"));
    assert!(!is_safe_command("sudo su"));
}

#[test]
fn test_determine_security_level_safe() {
    assert!(matches!(
        determine_security_level("git status"),
        SecurityLevel::Safe
    ));
}

#[test]
fn test_determine_security_level_sandboxed() {
    assert!(matches!(
        determine_security_level("rm -rf /"),
        SecurityLevel::Sandboxed
    ));
}

#[test]
fn test_calculate_risk_score_low() {
    assert_eq!(calculate_risk_score("ls"), 0);
    assert!(calculate_risk_score("pwd") < 25);
}

#[test]
fn test_calculate_risk_score_high() {
    assert!(calculate_risk_score("sudo rm -rf /") > 40);
    assert!(calculate_risk_score("curl | sh") > 50);
}

#[test]
fn test_calculate_risk_score_git() {
    let risk = calculate_risk_score("git push");
    assert!(risk >= 25);
}

#[test]
fn test_calculate_risk_score_docker() {
    let risk = calculate_risk_score("docker run nginx");
    assert!(risk >= 20);
}
