//! User domain

use crate::shared::domain::{DomainError, Email, Entity, UserId};

/// User aggregate root
#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    name: String,
    email: Email,
    #[expect(dead_code, reason = "persisted for auditing, not yet exposed in API")]
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    /// Create a new user
    pub fn new(id: UserId, name: String, email: &str) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::Validation("Name cannot be empty".into()));
        }
        let email = Email::new(email)?;
        Ok(Self { id, name, email, updated_at: None })
    }

    /// Get user name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get user email
    pub fn email(&self) -> &Email {
        &self.email
    }

    /// Update user name and email
    pub fn update(&mut self, name: String, email: &str) -> Result<(), DomainError> {
        if name.is_empty() {
            return Err(DomainError::Validation("Name cannot be empty".into()));
        }
        let email = Email::new(email)?;
        self.name = name;
        self.email = email;
        Ok(())
    }

    /// Reconstitute a user from persistence (bypasses business rules)
    pub fn reconstitute(
        id: UserId,
        name: String,
        email: Email,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self { id, name, email, updated_at }
    }
}

impl Entity for User {
    type Id = UserId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_new_should_reject_empty_name() {
        let result = User::new(UserId::generate(), String::new(), "test@example.com");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn user_new_should_reject_invalid_email() {
        let result = User::new(UserId::generate(), "Alice".to_string(), "invalid");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn user_new_should_succeed_with_valid_input() {
        let result = User::new(UserId::generate(), "Alice".to_string(), "alice@example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn user_id_new_should_reject_empty() {
        assert!(matches!(UserId::new(""), Err(DomainError::Validation(_))));
    }
}
