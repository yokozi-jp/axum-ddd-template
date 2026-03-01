//! Shared value objects

use crate::shared::domain::DomainError;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

/// Generate a typed string ID value object with validation.
macro_rules! string_id {
    ($name:ident, $label:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            #[must_use]
            pub fn generate() -> Self {
                Self(uuid::Uuid::new_v4().to_string())
            }

            pub fn new(id: &str) -> Result<Self, crate::shared::domain::DomainError> {
                if id.is_empty() {
                    return Err(crate::shared::domain::DomainError::Validation(
                        concat!($label, " ID cannot be empty").into(),
                    ));
                }
                Ok(Self(id.to_owned()))
            }

            /// Reconstitute from trusted storage without re-validation
            pub fn from_trusted(value: String) -> Self {
                Self(value)
            }

            pub fn value(&self) -> &str {
                &self.0
            }

            pub fn entity_name() -> &'static str {
                $label
            }
        }
    };
}

pub(crate) use string_id;

string_id!(UserId, "User");

/// Email value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Create a new email with validation
    pub fn new(email: &str) -> Result<Self, DomainError> {
        if !EmailAddress::is_valid(email) {
            return Err(DomainError::Validation("Invalid email format".into()));
        }
        Ok(Self(email.to_owned()))
    }

    /// Reconstitute from trusted storage without re-validation
    pub fn from_trusted(value: String) -> Self {
        Self(value)
    }

    /// Get email value
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_new_should_reject_invalid_format() {
        assert!(Email::new("invalid").is_err());
        assert!(Email::new("missing-at.com").is_err());
    }

    #[test]
    fn email_new_should_accept_valid_email() {
        assert!(Email::new("test@example.com").is_ok());
    }

    #[test]
    fn user_id_new_should_reject_empty() {
        assert!(UserId::new("").is_err());
    }

    #[test]
    fn user_id_generate_should_be_non_empty() {
        assert!(!UserId::generate().value().is_empty());
    }
}
