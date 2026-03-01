//! Task application ports (driver-side interfaces)

use crate::features::task::domain::Task;
use crate::shared::domain::DomainError;

use super::CreateTaskCommand;

#[async_trait::async_trait]
pub trait CreateTask: Send + Sync {
    async fn execute(&self, command: CreateTaskCommand) -> Result<Task, DomainError>;
}

#[async_trait::async_trait]
pub trait GetTask: Send + Sync {
    async fn execute(&self, id: &str) -> Result<Task, DomainError>;
}

#[async_trait::async_trait]
pub trait ListTasks: Send + Sync {
    async fn execute(&self, user_id: Option<&str>) -> Result<Vec<Task>, DomainError>;
}

#[async_trait::async_trait]
pub trait CompleteTask: Send + Sync {
    async fn execute(&self, id: &str) -> Result<Task, DomainError>;
}

#[async_trait::async_trait]
pub trait DeleteTask: Send + Sync {
    async fn execute(&self, id: &str) -> Result<(), DomainError>;
}
