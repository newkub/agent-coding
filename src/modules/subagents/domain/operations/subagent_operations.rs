use crate::modules::subagents::domain::models::subagent::{AgentType, Subagent, TaskType};

/// Pure function to select appropriate subagent for task
pub fn select_subagent_for_task<'a>(
    subagents: &'a [Subagent],
    task_type: &TaskType,
) -> Option<&'a Subagent> {
    subagents
        .iter()
        .filter(|agent| agent.is_available())
        .find(|agent| agent.can_handle(task_type))
}

/// Pure function to estimate task complexity
pub const fn estimate_task_complexity(
    task_type: &TaskType,
    input_length: usize,
) -> ComplexityLevel {
    let base_complexity = match task_type {
        TaskType::CodeReview => ComplexityLevel::Medium,
        TaskType::BugDetection => ComplexityLevel::High,
        TaskType::Refactoring => ComplexityLevel::High,
        TaskType::Documentation => ComplexityLevel::Low,
        TaskType::TestGeneration => ComplexityLevel::Medium,
        TaskType::SecurityAudit => ComplexityLevel::High,
        TaskType::PerformanceAnalysis => ComplexityLevel::High,
        TaskType::DependencyUpdate => ComplexityLevel::Low,
        TaskType::Custom(_) => ComplexityLevel::Medium,
    };

    // Adjust based on input length
    if input_length > 10000 {
        match base_complexity {
            ComplexityLevel::Low => ComplexityLevel::Medium,
            ComplexityLevel::Medium => ComplexityLevel::High,
            ComplexityLevel::High => ComplexityLevel::Critical,
            ComplexityLevel::Critical => ComplexityLevel::Critical,
        }
    } else {
        base_complexity
    }
}

/// Pure function to calculate task priority
pub const fn calculate_task_priority(
    task_type: &TaskType,
    complexity: ComplexityLevel,
) -> TaskPriority {
    match (task_type, complexity) {
        (TaskType::SecurityAudit, _) => TaskPriority::Critical,
        (TaskType::BugDetection, ComplexityLevel::High | ComplexityLevel::Critical) => {
            TaskPriority::High
        }
        (TaskType::BugDetection, _) => TaskPriority::Medium,
        (TaskType::CodeReview, ComplexityLevel::High | ComplexityLevel::Critical) => {
            TaskPriority::High
        }
        (TaskType::CodeReview, _) => TaskPriority::Medium,
        (TaskType::Refactoring, ComplexityLevel::Critical) => TaskPriority::High,
        (TaskType::Refactoring, _) => TaskPriority::Medium,
        (_, _) => TaskPriority::Low,
    }
}

/// Pure function to generate system prompt for subagent
pub fn generate_system_prompt(agent_type: &AgentType) -> String {
    match agent_type {
        AgentType::CodeReviewer => {
            "You are a code reviewer. Analyze code for quality, maintainability, and best practices. Provide constructive feedback and suggestions for improvement.".to_string()
        }
        AgentType::BugHunter => {
            "You are a bug hunter. Identify potential bugs, edge cases, and logical errors in code. Focus on correctness and robustness.".to_string()
        }
        AgentType::Refactorer => {
            "You are a code refactoring expert. Suggest improvements to code structure, readability, and performance while maintaining functionality.".to_string()
        }
        AgentType::Documenter => {
            "You are a technical writer. Generate clear, comprehensive documentation for code, including usage examples and explanations.".to_string()
        }
        AgentType::Tester => {
            "You are a test engineer. Generate comprehensive test cases, including unit tests, integration tests, and edge case scenarios.".to_string()
        }
        AgentType::SecurityAuditor => {
            "You are a security auditor. Identify security vulnerabilities, injection risks, and potential attack vectors in code.".to_string()
        }
        AgentType::PerformanceOptimizer => {
            "You are a performance optimization expert. Identify bottlenecks, suggest optimizations, and provide performance metrics.".to_string()
        }
        AgentType::DependencyManager => {
            "You are a dependency manager. Analyze dependencies for security vulnerabilities, outdated versions, and licensing issues.".to_string()
        }
        AgentType::Custom(name) => {
            format!("You are a specialized agent for: {}. Provide expert assistance in this domain.", name)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_subagent_for_task() {
        let mut agent = Subagent::new(
            "Code Reviewer".to_string(),
            AgentType::CodeReviewer,
            "Reviews code".to_string(),
        );
        agent.status = crate::modules::subagents::domain::models::subagent::SubagentStatus::Idle;

        let subagents = vec![agent];
        let selected = select_subagent_for_task(&subagents, &TaskType::CodeReview);
        assert!(selected.is_some());
    }

    #[test]
    fn test_estimate_task_complexity() {
        let complexity = estimate_task_complexity(&TaskType::BugDetection, 5000);
        assert_eq!(complexity, ComplexityLevel::High);
    }

    #[test]
    fn test_calculate_task_priority() {
        let priority = calculate_task_priority(&TaskType::SecurityAudit, ComplexityLevel::Medium);
        assert_eq!(priority, TaskPriority::Critical);
    }

    #[test]
    fn test_generate_system_prompt() {
        let prompt = generate_system_prompt(&AgentType::CodeReviewer);
        assert!(prompt.contains("code reviewer"));
    }
}
