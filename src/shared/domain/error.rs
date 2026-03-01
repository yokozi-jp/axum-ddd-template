//! Domain errors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[expect(dead_code, reason = "reserved for future use")]
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
