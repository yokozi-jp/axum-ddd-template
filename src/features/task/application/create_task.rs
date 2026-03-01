//! Create task use case

use crate::features::task::application::port::CreateTask;
use crate::features::task::domain::{Task, TaskId, TaskRepository};
use crate::shared::domain::{DomainError, UserId};
use std::sync::Arc;

/// Command to create a new task
#[derive(Debug)]
pub struct CreateTaskCommand {
    /// User ID who owns the task
    pub user_id: String,
    /// Task title
    pub title: String,
    /// Task description
    pub description: String,
}

/// Use case for creating a task
pub struct CreateTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl CreateTaskUseCase {
    /// Create a new use case instance
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }
}

#[async_trait::async_trait]
impl CreateTask for CreateTaskUseCase {
    /// User existence is enforced by the database FK constraint.
    /// If the user doesn't exist, the insert will fail with `DomainError::NotFound`.
    async fn execute(&self, command: CreateTaskCommand) -> Result<Task, DomainError> {
        let user_id = UserId::new(&command.user_id)?;
        let task = Task::new(TaskId::generate(), user_id, command.title, command.description)?;
        self.task_repository.insert(&task).await?;
        Ok(task)
    }
}
