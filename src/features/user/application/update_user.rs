//! Update user use case

use crate::features::user::domain::{User, UserId, UserRepository};
use crate::shared::domain::DomainError;
use std::sync::Arc;

/// Command to update a user
#[derive(Debug)]
pub struct UpdateUserCommand {
    /// User name
    pub name: String,
    /// User email
    pub email: String,
}

/// Use case for updating a user
pub struct UpdateUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl UpdateUserUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: &str, command: UpdateUserCommand) -> Result<User, DomainError> {
        let user_id = UserId::new(id)?;

        let mut user = self
            .repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("{} not found", UserId::entity_name())))?;

        user.update(command.name, &command.email)?;
        self.repository.update(&user).await?;
        Ok(user)
    }
}
