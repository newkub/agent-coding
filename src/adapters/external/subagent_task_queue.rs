use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::subagents::domain::models::subagent::SubagentTask;
use crate::modules::subagents::ports::TaskQueue;
use crate::shared::kernel::result::AppError;

/// In-memory task queue
pub(crate) struct InMemoryTaskQueue {
    queue: Arc<RwLock<VecDeque<SubagentTask>>>,
}

impl InMemoryTaskQueue {
    pub(crate) fn new() -> Self {
        Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }
}

impl Default for InMemoryTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskQueue for InMemoryTaskQueue {
    async fn enqueue(&self, task: SubagentTask) -> Result<(), AppError> {
        let mut queue = self.queue.write().await;
        queue.push_back(task);
        Ok(())
    }

    async fn dequeue(&self) -> Result<SubagentTask, AppError> {
        let mut queue = self.queue.write().await;
        queue
            .pop_front()
            .ok_or_else(|| AppError::State("Queue is empty".to_string()))
    }

    async fn queue_size(&self) -> Result<usize, AppError> {
        let queue = self.queue.read().await;
        Ok(queue.len())
    }

    async fn clear_queue(&self) -> Result<(), AppError> {
        let mut queue = self.queue.write().await;
        queue.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::subagents::domain::models::subagent::{TaskContext, TaskType};

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let queue = InMemoryTaskQueue::new();
        let context = TaskContext::new();
        let task = SubagentTask::new(
            "agent-1".to_string(),
            TaskType::CodeReview,
            "Review".to_string(),
            context,
        );

        queue.enqueue(task).await.unwrap();
        assert_eq!(queue.queue_size().await.unwrap(), 1);

        let dequeued = queue.dequeue().await.unwrap();
        assert_eq!(dequeued.subagent_id, "agent-1");
    }

    #[tokio::test]
    async fn test_clear_queue() {
        let queue = InMemoryTaskQueue::new();
        let context = TaskContext::new();
        let task = SubagentTask::new(
            "agent-1".to_string(),
            TaskType::CodeReview,
            "Review".to_string(),
            context,
        );

        queue.enqueue(task).await.unwrap();
        queue.clear_queue().await.unwrap();
        assert_eq!(queue.queue_size().await.unwrap(), 0);
    }
}
