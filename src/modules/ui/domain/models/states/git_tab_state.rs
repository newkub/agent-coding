use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct GitTabState {
    pub selected_file_index: usize,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    // Stash management
    pub stashes: Vec<GitStash>,
    pub selected_stash_index: usize,
    // Branch management
    pub branches: Vec<GitBranch>,
    pub selected_branch_index: usize,
    // Commit management
    pub commits: Vec<GitCommit>,
    pub selected_commit_index: usize,
    // Rebase mode
    pub rebase_mode: bool,
    pub rebase_commits: Vec<GitCommit>,
    // Search
    pub search_query: String,
    pub search_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GitStash {
    pub id: String,
    pub message: String,
    pub branch: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GitBranch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub commit_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GitCommit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub is_merged: bool,
}

impl TabState for GitTabState {
    fn tab(&self) -> Tab {
        Tab::Git
    }
}