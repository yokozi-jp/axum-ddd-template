//! Task domain

use crate::features::task::domain::value_objects::TaskId;
use crate::shared::domain::{DomainError, Entity, UserId};

/// Task aggregate root
#[derive(Debug, Clone)]
pub struct Task {
    id: TaskId,
    user_id: UserId,
    title: String,
    description: String,
    completed: bool,
    #[expect(dead_code, reason = "persisted for auditing, not yet exposed in API")]
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Task {
    /// Create a new task
    pub fn new(
        id: TaskId,
        user_id: UserId,
        title: String,
        description: String,
    ) -> Result<Self, DomainError> {
        if title.is_empty() {
            return Err(DomainError::Validation("Title cannot be empty".into()));
        }
        Ok(Self {
            id,
            user_id,
            title,
            description,
            completed: false,
            updated_at: None,
        })
    }

    /// Reconstitute a task from persistence (bypasses business rules)
    pub fn reconstitute(
        id: TaskId,
        user_id: UserId,
        title: String,
        description: String,
        completed: bool,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self { id, user_id, title, description, completed, updated_at }
    }

    /// Get user ID
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    /// Get task title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get task description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Check if task is completed
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Mark task as completed
    ///
    /// # Errors
    /// Returns `DomainError::Validation` if the task is already completed.
    pub fn complete(&mut self) -> Result<(), DomainError> {
        if self.completed {
            return Err(DomainError::Validation("Task is already completed".into()));
        }
        self.completed = true;
        Ok(())
    }
}

impl Entity for Task {
    type Id = TaskId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(test)]
#[expect(clippy::expect_used, reason = "expect is acceptable in tests")]
mod tests {
    use super::*;

    #[test]
    fn task_new_should_reject_empty_title() {
        let user_id = UserId::new("user1").expect("valid user id");
        let result = Task::new(TaskId::generate(), user_id, String::new(), String::new());
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn task_new_should_succeed_with_valid_input() {
        let user_id = UserId::new("user1").expect("valid user id");
        let result = Task::new(TaskId::generate(), user_id, "Buy milk".to_string(), String::new());
        assert!(result.is_ok());
    }

    #[test]
    fn task_complete_should_mark_as_completed() {
        let user_id = UserId::new("user1").expect("valid user id");
        let mut task = Task::new(TaskId::generate(), user_id, "Buy milk".to_string(), String::new())
            .expect("valid task");
        assert!(!task.is_completed());
        task.complete().expect("first complete should succeed");
        assert!(task.is_completed());
    }

    #[test]
    fn task_complete_should_reject_already_completed() {
        let user_id = UserId::new("user1").expect("valid user id");
        let mut task = Task::new(TaskId::generate(), user_id, "Buy milk".to_string(), String::new())
            .expect("valid task");
        task.complete().expect("first complete should succeed");
        assert!(matches!(task.complete(), Err(DomainError::Validation(_))));
    }

    #[test]
    fn task_id_new_should_reject_empty() {
        assert!(matches!(TaskId::new(""), Err(DomainError::Validation(_))));
    }
}
