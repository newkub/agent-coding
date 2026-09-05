use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// GitHub Issue entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Issue {
    pub id: u64,
    pub number: u32,
    pub title: String,
    pub body: String,
    pub state: IssueState,
    pub author: String,
    pub assignees: Vec<String>,
    pub labels: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub repository: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueState {
    Open,
    Closed,
}

/// Pull Request entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PullRequest {
    pub id: u64,
    pub number: u32,
    pub title: String,
    pub body: String,
    pub state: PRState,
    pub author: String,
    pub source_branch: String,
    pub target_branch: String,
    pub reviewers: Vec<String>,
    pub labels: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub repository: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PRState {
    Open,
    Closed,
    Merged,
}

/// Automation workflow for Issue-to-PR conversion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutomationWorkflow {
    pub id: String,
    pub issue: Issue,
    pub pr: Option<PullRequest>,
    pub status: WorkflowStatus,
    pub steps: Vec<WorkflowStep>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub status: StepStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Automation configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutomationConfig {
    pub auto_create_branch: bool,
    pub auto_commit: bool,
    pub auto_push: bool,
    pub auto_create_pr: bool,
    pub pr_template: Option<String>,
    pub branch_name_template: String,
    pub commit_message_template: String,
    pub default_reviewers: Vec<String>,
    pub default_labels: Vec<String>,
}

impl Default for AutomationConfig {
    fn default() -> Self {
        Self {
            auto_create_branch: true,
            auto_commit: true,
            auto_push: true,
            auto_create_pr: true,
            pr_template: None,
            branch_name_template: "feature/issue-{number}".to_string(),
            commit_message_template: "feat: {title}".to_string(),
            default_reviewers: Vec::new(),
            default_labels: vec!["automated".to_string()],
        }
    }
}

impl Issue {
    pub fn new(
        number: u32,
        title: String,
        body: String,
        author: String,
        repository: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            number,
            title,
            body,
            state: IssueState::Open,
            author,
            assignees: Vec::new(),
            labels: Vec::new(),
            created_at: now,
            updated_at: now,
            repository,
        }
    }

    pub fn is_automatable(&self) -> bool {
        // Check if issue has required labels or keywords
        let automatable_labels = ["automated", "auto-pr", "bot"];
        let has_automatable_label = self
            .labels
            .iter()
            .any(|l| automatable_labels.contains(&l.to_lowercase().as_str()));

        let body_lower = self.body.to_lowercase();
        let has_automatable_keyword =
            body_lower.contains("/automate") || body_lower.contains("/auto-pr");

        has_automatable_label || has_automatable_keyword
    }
}

impl PullRequest {
    pub fn new(
        number: u32,
        title: String,
        body: String,
        author: String,
        source_branch: String,
        target_branch: String,
        repository: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            number,
            title,
            body,
            state: PRState::Open,
            author,
            source_branch,
            target_branch,
            reviewers: Vec::new(),
            labels: Vec::new(),
            created_at: now,
            updated_at: now,
            repository,
        }
    }
}

impl AutomationWorkflow {
    pub fn new(issue: Issue) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            issue,
            pr: None,
            status: WorkflowStatus::Pending,
            steps: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_step(&mut self, name: String) {
        let step = WorkflowStep {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            status: StepStatus::Pending,
            started_at: None,
            completed_at: None,
            error: None,
        };
        self.steps.push(step);
    }

    pub fn update_step(&mut self, step_id: &str, status: StepStatus, error: Option<String>) {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == step_id) {
            step.status = status.clone();
            match status {
                StepStatus::Running => {
                    step.started_at = Some(Utc::now());
                }
                StepStatus::Completed | StepStatus::Failed => {
                    step.completed_at = Some(Utc::now());
                }
                _ => {}
            }
            step.error = error;
        }
    }

    pub fn current_step(&self) -> Option<&WorkflowStep> {
        self.steps.iter().find(|s| s.status == StepStatus::Running)
    }
}
