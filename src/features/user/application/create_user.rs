//! Create user use case

use crate::features::user::application::port::CreateUser;
use crate::features::user::domain::{User, UserId, UserRepository};
use crate::shared::domain::DomainError;
use std::sync::Arc;

/// Command to create a new user
#[derive(Debug)]
pub struct CreateUserCommand {
    /// User name
    pub name: String,
    /// User email
    pub email: String,
}

/// Use case for creating a user
pub struct CreateUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl CreateUserUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl CreateUser for CreateUserUseCase {
    async fn execute(&self, command: CreateUserCommand) -> Result<User, DomainError> {
        let user = User::new(UserId::generate(), command.name, &command.email)?;
        self.repository.insert(&user).await?;
        Ok(user)
    }
}
