use async_trait::async_trait;
use git2::{BranchType, Repository};

use crate::modules::automation::ports::GitOperations;
use crate::shared::kernel::result::AppError;

/// Git adapter using git2 library
#[derive(Clone)]
pub struct Git2Adapter {
    repo_path: String,
}

impl Git2Adapter {
    pub(crate) const fn new(repo_path: String) -> Self {
        Self { repo_path }
    }

    fn get_repo(&self) -> Result<Repository, git2::Error> {
        Repository::open(&self.repo_path)
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
