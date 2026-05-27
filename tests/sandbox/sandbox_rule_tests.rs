//! Rule matching tests

use agent_tui::modules::sandbox::domain::operations::match_rule_pattern;

#[test]
fn test_match_rule_pattern_regex() {
    assert!(match_rule_pattern("git commit", r"git \w+"));
    assert!(match_rule_pattern("git status", r"git \w+")); // This matches because regex is valid
    assert!(!match_rule_pattern("git", r"git \w+")); // This doesn't match (no word after git)
}

#[test]
fn test_match_rule_pattern_fallback() {
    assert!(match_rule_pattern("curl http://example.com", "curl"));
    assert!(!match_rule_pattern("wget http://example.com", "curl"));
}

#[test]
fn test_match_rule_pattern_invalid_regex() {
    // Invalid regex falls back to contains check
    // "[invalid" is not contained in "test pattern", so this should be false
    assert!(!match_rule_pattern("test pattern", "[invalid"));
    // But it should match if the pattern is contained
    assert!(match_rule_pattern("test pattern", "test"));
}
