use async_trait::async_trait;
use reqwest::Client;

use crate::adapters::external::github_parser::{parse_issue_from_json, parse_pr_from_json};
use crate::modules::automation::domain::models::issue_pr::{Issue, PullRequest};
use crate::modules::automation::ports::GitHubClient;
use crate::shared::kernel::result::AppError;

/// GitHub API client using reqwest
#[derive(Clone)]
pub struct ReqwestGitHubClient {
    client: Client,
    token: String,
    base_url: String,
}

impl ReqwestGitHubClient {
    pub(crate) fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
            base_url: "https://api.github.com".to_string(),
        }
    }

    pub(crate) fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

#[async_trait]
impl GitHubClient for ReqwestGitHubClient {
    async fn get_issue(&self, repository: &str, number: u32) -> Result<Issue, AppError> {
        let url = format!("{}/repos/{}/issues/{}", self.base_url, repository, number);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "agent-tui")
            .send()
            .await
            .map_err(|e| AppError::State(format!("GitHub API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::State(format!(
                "GitHub API returned {}: {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::State(format!("Failed to parse GitHub response: {}", e)))?;

        Ok(parse_issue_from_json(&json, repository)?)
    }

    async fn create_pull_request(
        &self,
        repository: &str,
        title: &str,
        body: &str,
        source_branch: &str,
        target_branch: &str,
    ) -> Result<PullRequest, AppError> {
        let url = format!("{}/repos/{}/pulls", self.base_url, repository);

        let payload = serde_json::json!({
            "title": title,
            "body": body,
            "head": source_branch,
            "base": target_branch,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "agent-tui")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::State(format!("GitHub API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::State(format!(
                "GitHub API returned {}: {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::State(format!("Failed to parse GitHub response: {}", e)))?;

        Ok(parse_pr_from_json(&json, repository)?)
    }

    async fn update_pull_request(
        &self,
        repository: &str,
        number: u32,
        pr: &PullRequest,
    ) -> Result<PullRequest, AppError> {
        let url = format!("{}/repos/{}/pulls/{}", self.base_url, repository, number);

        let payload = serde_json::json!({
            "title": pr.title,
            "body": pr.body,
        });

        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "agent-tui")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::State(format!("GitHub API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::State(format!(
                "GitHub API returned {}: {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::State(format!("Failed to parse GitHub response: {}", e)))?;

        Ok(parse_pr_from_json(&json, repository)?)
    }

    async fn add_labels(
        &self,
        repository: &str,
        issue_number: u32,
        labels: Vec<String>,
    ) -> Result<(), AppError> {
        let url = format!(
            "{}/repos/{}/issues/{}/labels",
            self.base_url, repository, issue_number
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "agent-tui")
            .json(&labels)
            .send()
            .await
            .map_err(|e| AppError::State(format!("GitHub API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::State(format!(
                "GitHub API returned {}: {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        Ok(())
    }

    async fn add_reviewers(
        &self,
        repository: &str,
        pr_number: u32,
        reviewers: Vec<String>,
    ) -> Result<(), AppError> {
        let url = format!(
            "{}/repos/{}/pulls/{}/requested_reviewers",
            self.base_url, repository, pr_number
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "agent-tui")
            .json(&reviewers)
            .send()
            .await
            .map_err(|e| AppError::State(format!("GitHub API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::State(format!(
                "GitHub API returned {}: {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        Ok(())
    }

    async fn get_default_branch(&self, repository: &str) -> Result<String, AppError> {
        let url = format!("{}/repos/{}", self.base_url, repository);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "agent-tui")
            .send()
            .await
            .map_err(|e| AppError::State(format!("GitHub API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::State(format!(
                "GitHub API returned {}: {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::State(format!("Failed to parse GitHub response: {}", e)))?;

        let default_branch = json["default_branch"]
            .as_str()
            .ok_or_else(|| AppError::State("Missing default_branch in response".to_string()))?;

        Ok(default_branch.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_client_creation() {
        let client = ReqwestGitHubClient::new("test_token".to_string());
        assert_eq!(client.token, "test_token");
    }
}
