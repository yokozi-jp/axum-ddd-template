//! User repository port

use super::entity::User;
use crate::shared::domain::{DomainError, UserId};

/// Repository for user aggregate
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    /// Find user by ID
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    /// Find all users
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;
    /// Insert a new user (fails if ID or email already exists)
    async fn insert(&self, user: &User) -> Result<(), DomainError>;
    /// Update an existing user
    async fn update(&self, user: &User) -> Result<(), DomainError>;
    /// Delete user by ID, returns true if a row was deleted
    async fn delete(&self, id: &UserId) -> Result<bool, DomainError>;
}
