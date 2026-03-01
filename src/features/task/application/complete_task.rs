//! Complete task use case

use crate::features::task::application::port::CompleteTask;
use crate::features::task::domain::{Task, TaskId, TaskRepository};
use crate::shared::domain::DomainError;
use std::sync::Arc;

/// Use case for completing a task
pub struct CompleteTaskUseCase {
    repository: Arc<dyn TaskRepository>,
}

impl CompleteTaskUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl CompleteTask for CompleteTaskUseCase {
    async fn execute(&self, id: &str) -> Result<Task, DomainError> {
        let task_id = TaskId::new(id)?;

        let mut task = self
            .repository
            .find_by_id(&task_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("{} not found", TaskId::entity_name())))?;

        task.complete()?;
        self.repository.update(&task).await?;
        Ok(task)
    }
}
