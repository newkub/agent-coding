use async_trait::async_trait;
use git2::{BranchType, DiffFormat, Repository, Status};
use std::path::Path;

use crate::modules::automation::ports::GitOperations;
use crate::shared::kernel::result::AppError;

/// Git adapter using git2 library
#[derive(Clone)]
pub(crate) struct Git2Adapter {
    repo_path: String,
}

impl Git2Adapter {
    pub(crate) const fn new(repo_path: String) -> Self {
        Self { repo_path }
    }

    fn get_repo(&self) -> Result<Repository, git2::Error> {
        Repository::open(&self.repo_path)
    }

    /// Current branch name, or `None` when not inside a repository / unborn HEAD.
    pub(crate) fn current_branch_name(&self) -> Option<String> {
        let repo = self.get_repo().ok()?;
        let head = repo.head().ok()?;
        head.shorthand().ok().map(str::to_string)
    }

    /// Working tree status as `(staged, unstaged)` path lists.
    pub(crate) fn status_entries(&self) -> Result<(Vec<String>, Vec<String>), AppError> {
        let repo = self.get_repo()?;
        let statuses = repo.statuses(None)?;

        let mut staged = Vec::new();
        let mut unstaged = Vec::new();

        for entry in statuses.iter() {
            let Ok(path) = entry.path() else {
                continue;
            };
            let status = entry.status();
            if status.intersects(
                Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_RENAMED
                    | Status::INDEX_TYPECHANGE,
            ) {
                staged.push(path.to_string());
            }
            if status.intersects(
                Status::WT_NEW
                    | Status::WT_MODIFIED
                    | Status::WT_DELETED
                    | Status::WT_RENAMED
                    | Status::WT_TYPECHANGE,
            ) {
                unstaged.push(path.to_string());
            }
        }

        Ok((staged, unstaged))
    }

    /// Stage a file (`git add <path>`).
    pub(crate) fn stage_file(&self, path: &str) -> Result<(), AppError> {
        let repo = self.get_repo()?;
        let mut index = repo.index()?;
        index.add_path(Path::new(path))?;
        index.write()?;
        Ok(())
    }

    /// Unstage a file (`git reset HEAD <path>`).
    ///
    /// When the repository has no HEAD yet (no commits), the entry is simply
    /// removed from the index, matching `git rm --cached` semantics for
    /// unborn branches.
    pub(crate) fn unstage_file(&self, path: &str) -> Result<(), AppError> {
        let repo = self.get_repo()?;
        match repo.head().and_then(|h| h.peel(git2::ObjectType::Any)) {
            Ok(target) => repo.reset_default(Some(&target), [path])?,
            Err(_) => {
                let mut index = repo.index()?;
                index.remove_path(Path::new(path))?;
                index.write()?;
            }
        }
        Ok(())
    }

    /// Unified diff patch for a single file (worktree vs index for unstaged
    /// files, HEAD vs index for staged files). Returns an empty string when
    /// there is no diff to show.
    pub(crate) fn diff_for_file(&self, path: &str, staged: bool) -> Result<String, AppError> {
        let repo = self.get_repo()?;
        let mut opts = git2::DiffOptions::new();
        opts.pathspec(path);

        let index = repo.index()?;
        let diff = if staged {
            match repo.head().and_then(|h| h.peel_to_tree()) {
                Ok(head_tree) => {
                    repo.diff_tree_to_index(Some(&head_tree), Some(&index), Some(&mut opts))?
                }
                // Unborn HEAD (no commits yet): fall back to a worktree diff.
                Err(_) => repo.diff_index_to_workdir(Some(&index), Some(&mut opts))?,
            }
        } else {
            repo.diff_index_to_workdir(Some(&index), Some(&mut opts))?
        };

        let mut text = String::new();
        let mut lines = 0usize;
        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            if let Ok(content) = std::str::from_utf8(line.content()) {
                match line.origin() {
                    '+' | '-' => text.push(line.origin()),
                    _ => {}
                }
                text.push_str(content);
            }
            // Keep the preview bounded.
            lines += 1;
            lines <= 200
        })?;

        Ok(text)
    }
}

#[async_trait]
impl GitOperations for Git2Adapter {
    async fn create_branch(&self, branch_name: &str) -> Result<(), AppError> {
        let repo = self.get_repo()?;
        let head = repo.head()?;
        let target = head.peel_to_commit()?;

        repo.branch(branch_name, &target, false)?;
        Ok(())
    }

    async fn checkout_branch(&self, branch_name: &str) -> Result<(), AppError> {
        let repo = self.get_repo()?;
        let obj = repo.revparse_single(&format!("refs/heads/{}", branch_name))?;
        let tree = obj.peel_to_tree()?;
        repo.checkout_tree(tree.as_object(), None)?;
        repo.set_head(&format!("refs/heads/{}", branch_name))?;
        Ok(())
    }

    async fn commit(&self, message: &str) -> Result<(), AppError> {
        let repo = self.get_repo()?;
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let head = repo.head()?;
        let parent_commit = head.peel_to_commit()?;

        let sig = repo.signature()?;
        let _oid = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent_commit])?;

        Ok(())
    }

    async fn push(&self, branch_name: &str) -> Result<(), AppError> {
        let repo = self.get_repo()?;
        let mut remote = repo.find_remote("origin")?;

        remote.push(&[format!("refs/heads/{}", branch_name)], None)?;
        Ok(())
    }

    async fn get_current_branch(&self) -> Result<String, AppError> {
        let repo = self.get_repo()?;
        let head = repo.head()?;
        let shorthand = head.shorthand().unwrap_or("unknown");
        Ok(shorthand.to_string())
    }

    async fn branch_exists(&self, branch_name: &str) -> Result<bool, AppError> {
        let repo = self.get_repo();
        match repo {
            Ok(r) => match r.find_branch(branch_name, BranchType::Local) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }

    async fn remote_branch_exists(&self, branch_name: &str) -> Result<bool, AppError> {
        let repo = self.get_repo();
        match repo {
            Ok(r) => match r.find_branch(branch_name, BranchType::Remote) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git2_adapter_creation() {
        let adapter = Git2Adapter::new(".".to_string());
        assert_eq!(adapter.repo_path, ".");
    }
}
