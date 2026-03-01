//! User domain layer

pub mod entity;
pub mod repository;

pub use crate::shared::domain::UserId;
pub use entity::User;
pub use repository::UserRepository;
