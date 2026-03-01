//! User application ports (driver-side interfaces)

use crate::features::user::domain::User;
use crate::shared::domain::DomainError;

use super::{CreateUserCommand, UpdateUserCommand};

#[async_trait::async_trait]
pub trait CreateUser: Send + Sync {
    async fn execute(&self, command: CreateUserCommand) -> Result<User, DomainError>;
}

#[async_trait::async_trait]
pub trait GetUser: Send + Sync {
    async fn execute(&self, id: &str) -> Result<User, DomainError>;
}

#[async_trait::async_trait]
pub trait ListUsers: Send + Sync {
    async fn execute(&self) -> Result<Vec<User>, DomainError>;
}

#[async_trait::async_trait]
pub trait UpdateUser: Send + Sync {
    async fn execute(&self, id: &str, command: UpdateUserCommand) -> Result<User, DomainError>;
}

#[async_trait::async_trait]
pub trait DeleteUser: Send + Sync {
    async fn execute(&self, id: &str) -> Result<(), DomainError>;
}
