//! Get user use case

use crate::features::user::domain::{User, UserId, UserRepository};
use crate::shared::domain::DomainError;
use std::sync::Arc;

/// Use case for getting a user by ID
pub struct GetUserUseCase {
    repository: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: &str) -> Result<User, DomainError> {
        let user_id = UserId::new(id)?;
        self.repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("{} not found", UserId::entity_name())))
    }
}

/// Use case for listing all users
pub struct ListUsersUseCase {
    repository: Arc<dyn UserRepository>,
}

impl ListUsersUseCase {
    /// Create a new use case instance
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<User>, DomainError> {
        self.repository.find_all().await
    }
}
