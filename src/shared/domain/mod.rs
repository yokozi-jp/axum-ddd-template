//! Shared domain types and abstractions

pub mod entity;
pub mod error;
pub mod value_objects;

pub use entity::Entity;
pub use error::DomainError;
pub use value_objects::{Email, UserId};
