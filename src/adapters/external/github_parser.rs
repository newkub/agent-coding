use crate::modules::automation::domain::models::issue_pr::{Issue, IssueState, PullRequest, PRState};
use crate::shared::kernel::result::AppError;

/// Parse GitHub Issue from JSON response
pub fn parse_issue_from_json(json: &serde_json::Value, repository: &str) -> Result<Issue, AppError> {
    let number = json["number"]
        .as_u64()
        .ok_or_else(|| AppError::State("Missing issue number".to_string()))? as u32;
    
    let title = json["title"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing issue title".to_string()))?
        .to_string();
    
    let body = json["body"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    let state_str = json["state"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing issue state".to_string()))?;
    
    let state = match state_str {
        "open" => IssueState::Open,
        "closed" => IssueState::Closed,
        _ => return Err(AppError::State(format!("Unknown issue state: {}", state_str))),
    };
    
    let author = json["user"]["login"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing issue author".to_string()))?
        .to_string();
    
    let assignees = json["assignees"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|a| a["login"].as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();
    
    let labels = json["labels"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l["name"].as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();
    
    let created_at = json["created_at"]
        .as_str()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);
    
    let updated_at = json["updated_at"]
        .as_str()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);

    Ok(Issue {
        id: json["id"].as_u64().unwrap_or(0),
        number,
        title,
        body,
        state,
        author,
        assignees,
        labels,
        created_at,
        updated_at,
        repository: repository.to_string(),
    })
}

/// Parse GitHub Pull Request from JSON response
pub fn parse_pr_from_json(json: &serde_json::Value, repository: &str) -> Result<PullRequest, AppError> {
    let number = json["number"]
        .as_u64()
        .ok_or_else(|| AppError::State("Missing PR number".to_string()))? as u32;
    
    let title = json["title"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing PR title".to_string()))?
        .to_string();
    
    let body = json["body"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    let state_str = json["state"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing PR state".to_string()))?;
    
    let state = match state_str {
        "open" => PRState::Open,
        "closed" => PRState::Closed,
        "merged" => PRState::Merged,
        _ => return Err(AppError::State(format!("Unknown PR state: {}", state_str))),
    };
    
    let author = json["user"]["login"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing PR author".to_string()))?
        .to_string();
    
    let source_branch = json["head"]["ref"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing PR source branch".to_string()))?
        .to_string();
    
    let target_branch = json["base"]["ref"]
        .as_str()
        .ok_or_else(|| AppError::State("Missing PR target branch".to_string()))?
        .to_string();
    
    let reviewers = json["requested_reviewers"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|r| r["login"].as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();
    
    let labels = json["labels"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l["name"].as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();
    
    let created_at = json["created_at"]
        .as_str()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);
    
    let updated_at = json["updated_at"]
        .as_str()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);

    Ok(PullRequest {
        id: json["id"].as_u64().unwrap_or(0),
        number,
        title,
        body,
        state,
        author,
        source_branch,
        target_branch,
        reviewers,
        labels,
        created_at,
        updated_at,
        repository: repository.to_string(),
    })
}
