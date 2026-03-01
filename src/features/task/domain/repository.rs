//! Task repository port

use super::entity::Task;
use super::value_objects::TaskId;
use crate::shared::domain::{DomainError, UserId};

/// Repository for task aggregate
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    /// Find task by ID
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>, DomainError>;
    /// Find tasks by user ID
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Task>, DomainError>;
    /// Find all tasks
    async fn find_all(&self) -> Result<Vec<Task>, DomainError>;
    /// Insert a new task (fails if ID already exists or FK violated)
    async fn insert(&self, task: &Task) -> Result<(), DomainError>;
    /// Update an existing task
    async fn update(&self, task: &Task) -> Result<(), DomainError>;
    /// Delete task by ID, returns true if a row was deleted
    async fn delete(&self, id: &TaskId) -> Result<bool, DomainError>;
}
