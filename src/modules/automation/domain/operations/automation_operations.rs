use crate::modules::automation::domain::models::issue_pr::{AutomationConfig, Issue};
use std::collections::HashMap;

/// Pure function to generate branch name from issue
pub fn generate_branch_name(issue: &Issue, config: &AutomationConfig) -> String {
    let template = &config.branch_name_template;
    let mut branch_name = template.clone();

    // Replace placeholders
    branch_name = branch_name.replace("{number}", &issue.number.to_string());
    branch_name = branch_name.replace("{title}", &slugify(&issue.title));
    branch_name = branch_name.replace("{author}", &slugify(&issue.author));

    // Sanitize branch name
    branch_name = branch_name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '/' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .to_lowercase();

    // Remove consecutive hyphens
    while branch_name.contains("--") {
        branch_name = branch_name.replace("--", "-");
    }

    // Trim hyphens from start/end
    branch_name.trim_matches('-').to_string()
}

/// Pure function to generate commit message from issue
pub fn generate_commit_message(issue: &Issue, config: &AutomationConfig) -> String {
    let template = &config.commit_message_template;
    let mut message = template.clone();

    message = message.replace("{title}", &issue.title);
    message = message.replace("{number}", &format!("#{}", issue.number));
    message = message.replace("{author}", &issue.author);

    // Add issue reference
    format!("{}\n\nCloses #{}", message, issue.number)
}

/// Pure function to generate PR title from issue
pub fn generate_pr_title(issue: &Issue) -> String {
    format!("{} (#{})", issue.title, issue.number)
}

/// Pure function to generate PR body from issue
pub fn generate_pr_body(issue: &Issue, config: &AutomationConfig) -> String {
    let mut body = if let Some(template) = &config.pr_template {
        template.clone()
    } else {
        String::new()
    };

    body = body.replace("{title}", &issue.title);
    body = body.replace("{number}", &issue.number.to_string());
    body = body.replace("{author}", &issue.author);
    body = body.replace("{body}", &issue.body);

    if body.is_empty() {
        body = format!(
            "Closes #{}\n\n{}\n\nAutomated PR from issue #{}",
            issue.number, issue.body, issue.number
        );
    }

    body
}

/// Pure function to extract labels from issue
pub fn extract_labels(issue: &Issue, config: &AutomationConfig) -> Vec<String> {
    let mut labels = config.default_labels.clone();

    // Add labels from issue that are not already in defaults
    for label in &issue.labels {
        if !labels.contains(label) {
            labels.push(label.clone());
        }
    }

    labels
}

/// Pure function to determine target branch from issue
pub fn determine_target_branch(issue: &Issue) -> String {
    // Check for branch labels
    let branch_labels: HashMap<&str, &str> = [
        ("main", "main"),
        ("develop", "develop"),
        ("staging", "staging"),
        ("production", "main"),
    ]
    .iter()
    .cloned()
    .collect();

    for label in &issue.labels {
        let label_lower = label.to_lowercase();
        if let Some(branch) = branch_labels.get(label_lower.as_str()) {
            return branch.to_string();
        }
    }

    // Default to main
    "main".to_string()
}

/// Pure function to slugify text
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_branch_name() {
        let issue = Issue::new(
            1,
            "Test Feature".to_string(),
            "Description".to_string(),
            "user".to_string(),
            "repo".to_string(),
        );

        let config = AutomationConfig {
            branch_name_template: "feature/{number}-{title}".to_string(),
            ..Default::default()
        };

        let branch = generate_branch_name(&issue, &config);
        assert!(branch.contains("1"));
        assert!(branch.contains("test-feature"));
    }

    #[test]
    fn test_generate_commit_message() {
        let issue = Issue::new(
            1,
            "Add new feature".to_string(),
            "Description".to_string(),
            "user".to_string(),
            "repo".to_string(),
        );

        let config = AutomationConfig {
            commit_message_template: "feat: {title}".to_string(),
            ..Default::default()
        };

        let message = generate_commit_message(&issue, &config);
        assert!(message.contains("Add new feature"));
        assert!(message.contains("Closes #1"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Test Feature"), "test-feature");
        assert_eq!(slugify("Test@Feature#123"), "test-feature-123");
        assert_eq!(slugify("Hello World"), "hello-world");
    }
}
