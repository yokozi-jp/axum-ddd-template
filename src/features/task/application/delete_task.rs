//! Delete task use case

use crate::features::task::application::port::DeleteTask;
use crate::features::task::domain::{TaskId, TaskRepository};
use crate::shared::domain::DomainError;
use std::sync::Arc;

/// Use case for deleting a task
pub struct DeleteTaskUseCase {
    repository: Arc<dyn TaskRepository>,
}

impl DeleteTaskUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl DeleteTask for DeleteTaskUseCase {
    async fn execute(&self, id: &str) -> Result<(), DomainError> {
        let task_id = TaskId::new(id)?;

        if !self.repository.delete(&task_id).await? {
            return Err(DomainError::NotFound("Task not found".into()));
        }

        Ok(())
    }
}
