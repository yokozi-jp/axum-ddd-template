//! Get task use case

use crate::features::task::application::port::{GetTask, ListTasks};
use crate::features::task::domain::{Task, TaskId, TaskRepository};
use crate::shared::domain::{DomainError, UserId};
use std::sync::Arc;

/// Use case for getting a task by ID
pub struct GetTaskUseCase {
    repository: Arc<dyn TaskRepository>,
}

impl GetTaskUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl GetTask for GetTaskUseCase {
    async fn execute(&self, id: &str) -> Result<Task, DomainError> {
        let task_id = TaskId::new(id)?;
        self.repository
            .find_by_id(&task_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("{} not found", TaskId::entity_name())))
    }
}

/// Use case for listing tasks, optionally filtered by user ID
pub struct ListTasksUseCase {
    repository: Arc<dyn TaskRepository>,
}

impl ListTasksUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ListTasks for ListTasksUseCase {
    /// Pass `Some(user_id)` to filter by user, `None` to list all
    async fn execute(&self, user_id: Option<&str>) -> Result<Vec<Task>, DomainError> {
        match user_id {
            Some(id) => {
                let uid = UserId::new(id)?;
                self.repository.find_by_user_id(&uid).await
            }
            None => self.repository.find_all().await,
        }
    }
}
