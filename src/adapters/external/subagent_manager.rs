use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::subagents::domain::models::subagent::Subagent;
use crate::modules::subagents::ports::SubagentManager;
use crate::shared::kernel::result::AppError;

/// In-memory subagent manager
pub(crate) struct InMemorySubagentManager {
    subagents: Arc<RwLock<HashMap<String, Subagent>>>,
}

impl InMemorySubagentManager {
    pub(crate) fn new() -> Self {
        Self {
            subagents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) async fn initialize_default_subagents(&self) -> Result<(), AppError> {
        let default_subagents = vec![
            (
                "code-reviewer",
                crate::modules::subagents::domain::models::subagent::AgentType::CodeReviewer,
                "Reviews code for quality and best practices",
            ),
            (
                "bug-hunter",
                crate::modules::subagents::domain::models::subagent::AgentType::BugHunter,
                "Identifies bugs and edge cases in code",
            ),
            (
                "refactorer",
                crate::modules::subagents::domain::models::subagent::AgentType::Refactorer,
                "Suggests code refactoring improvements",
            ),
            (
                "documenter",
                crate::modules::subagents::domain::models::subagent::AgentType::Documenter,
                "Generates comprehensive documentation",
            ),
            (
                "tester",
                crate::modules::subagents::domain::models::subagent::AgentType::Tester,
                "Generates test cases and scenarios",
            ),
            (
                "security-auditor",
                crate::modules::subagents::domain::models::subagent::AgentType::SecurityAuditor,
                "Identifies security vulnerabilities",
            ),
            (
                "performance-optimizer",
                crate::modules::subagents::domain::models::subagent::AgentType::PerformanceOptimizer,
                "Analyzes and optimizes performance",
            ),
        ];

        for (name, agent_type, description) in default_subagents {
            let mut agent = Subagent::new(
                name.to_string(),
                agent_type.clone(),
                description.to_string(),
            );
            agent.capabilities = vec![description.to_string()];
            agent.config.system_prompt = crate::modules::subagents::domain::operations::subagent_operations::generate_system_prompt(&agent_type);

            self.create_subagent(agent).await?;
        }

        Ok(())
    }
}

impl Default for InMemorySubagentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SubagentManager for InMemorySubagentManager {
    async fn create_subagent(&self, subagent: Subagent) -> Result<Subagent, AppError> {
        let mut subagents = self.subagents.write().await;
        subagents.insert(subagent.id.clone(), subagent.clone());
        Ok(subagent)
    }

    async fn get_subagent(&self, id: &str) -> Result<Subagent, AppError> {
        let subagents = self.subagents.read().await;
        subagents
            .get(id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Subagent {} not found", id)))
    }

    async fn list_subagents(&self) -> Result<Vec<Subagent>, AppError> {
        let subagents = self.subagents.read().await;
        Ok(subagents.values().cloned().collect())
    }

    async fn update_subagent(&self, subagent: Subagent) -> Result<Subagent, AppError> {
        let mut subagents = self.subagents.write().await;
        if subagents.contains_key(&subagent.id) {
            subagents.insert(subagent.id.clone(), subagent.clone());
            Ok(subagent)
        } else {
            Err(AppError::NotFound(format!(
                "Subagent {} not found",
                subagent.id
            )))
        }
    }

    async fn delete_subagent(&self, id: &str) -> Result<(), AppError> {
        let mut subagents = self.subagents.write().await;
        if subagents.remove(id).is_some() {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Subagent {} not found", id)))
        }
    }

    async fn get_available_subagents(&self) -> Result<Vec<Subagent>, AppError> {
        let subagents = self.subagents.read().await;
        Ok(subagents
            .values()
            .filter(|agent| agent.is_available())
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_subagent() {
        let manager = InMemorySubagentManager::new();
        let agent = Subagent::new(
            "Test Agent".to_string(),
            crate::modules::subagents::domain::models::subagent::AgentType::CodeReviewer,
            "Test".to_string(),
        );
        let result = manager.create_subagent(agent).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_subagents() {
        let manager = InMemorySubagentManager::new();
        manager.initialize_default_subagents().await.unwrap();
        let agents = manager.list_subagents().await.unwrap();
        assert!(!agents.is_empty());
    }

    #[tokio::test]
    async fn test_get_available_subagents() {
        let manager = InMemorySubagentManager::new();
        manager.initialize_default_subagents().await.unwrap();
        let agents = manager.get_available_subagents().await.unwrap();
        assert!(!agents.is_empty());
    }
}
