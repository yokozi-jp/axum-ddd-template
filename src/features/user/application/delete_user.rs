//! Delete user use case

use crate::features::user::application::port::DeleteUser;
use crate::features::user::domain::{UserId, UserRepository};
use crate::shared::domain::DomainError;
use std::sync::Arc;

/// Use case for deleting a user
pub struct DeleteUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl DeleteUserUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl DeleteUser for DeleteUserUseCase {
    /// Note: deleting a user will cascade-delete all their tasks
    /// (enforced by `ON DELETE CASCADE` on the tasks FK constraint).
    async fn execute(&self, id: &str) -> Result<(), DomainError> {
        let user_id = UserId::new(id)?;

        if !self.repository.delete(&user_id).await? {
            return Err(DomainError::NotFound("User not found".into()));
        }

        Ok(())
    }
}
