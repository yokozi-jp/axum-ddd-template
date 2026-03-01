//! `PostgreSQL` task repository implementation

use crate::features::task::domain::{Task, TaskId, TaskRepository};
use crate::shared::domain::{DomainError, Entity, UserId};
use crate::shared::infrastructure::database::map_db_error;
use sqlx::PgPool;

/// `PostgreSQL` implementation of task repository
#[derive(Clone)]
pub struct PgTaskRepository {
    pool: PgPool,
}

impl PgTaskRepository {
    /// Create a new `PostgreSQL` task repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TaskRepository for PgTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>, DomainError> {
        Ok(sqlx::query_as::<_, TaskRow>(
            "SELECT id, user_id, title, description, completed, updated_at FROM tasks WHERE id = $1",
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| map_db_error(e, "find", "task"))?
        .map(TaskRow::into_domain))
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Task>, DomainError> {
        Ok(sqlx::query_as::<_, TaskRow>(
            "SELECT id, user_id, title, description, completed, updated_at FROM tasks WHERE user_id = $1",
        )
        .bind(user_id.value())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_db_error(e, "find_by_user_id", "task"))?
        .into_iter()
        .map(TaskRow::into_domain)
        .collect())
    }

    async fn find_all(&self) -> Result<Vec<Task>, DomainError> {
        Ok(sqlx::query_as::<_, TaskRow>(
            "SELECT id, user_id, title, description, completed, updated_at FROM tasks",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| map_db_error(e, "find_all", "task"))?
        .into_iter()
        .map(TaskRow::into_domain)
        .collect())
    }

    async fn insert(&self, task: &Task) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO tasks (id, user_id, title, description) VALUES ($1, $2, $3, $4)",
        )
        .bind(task.id().value())
        .bind(task.user_id().value())
        .bind(task.title())
        .bind(task.description())
        .execute(&self.pool)
        .await
        .map_err(|e| map_db_error(e, "insert", "task"))?;
        Ok(())
    }

    async fn update(&self, task: &Task) -> Result<(), DomainError> {
        sqlx::query(
            "UPDATE tasks SET title = $1, description = $2, completed = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $4",
        )
        .bind(task.title())
        .bind(task.description())
        .bind(task.is_completed())
        .bind(task.id().value())
        .execute(&self.pool)
        .await
        .map_err(|e| map_db_error(e, "update", "task"))?;
        Ok(())
    }

    async fn delete(&self, id: &TaskId) -> Result<bool, DomainError> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| map_db_error(e, "delete", "task"))?;
        Ok(result.rows_affected() > 0)
    }
}

#[derive(sqlx::FromRow)]
struct TaskRow {
    id: String,
    user_id: String,
    title: String,
    description: String,
    completed: bool,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TaskRow {
    fn into_domain(self) -> Task {
        Task::reconstitute(
            TaskId::from_trusted(self.id),
            UserId::from_trusted(self.user_id),
            self.title,
            self.description,
            self.completed,
            self.updated_at,
        )
    }
}
