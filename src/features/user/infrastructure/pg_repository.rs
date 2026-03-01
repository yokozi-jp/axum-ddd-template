//! `PostgreSQL` user repository implementation

use crate::features::user::domain::{User, UserRepository};
use crate::shared::domain::{DomainError, Email, Entity, UserId};
use crate::shared::infrastructure::database::map_db_error;
use sqlx::PgPool;

/// `PostgreSQL` implementation of user repository
#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    /// Create a new `PostgreSQL` user repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        Ok(sqlx::query_as::<_, UserRow>("SELECT id, name, email, updated_at FROM users WHERE id = $1")
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| map_db_error(e, "find", "user"))?
            .map(UserRow::into_domain))
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        Ok(sqlx::query_as::<_, UserRow>("SELECT id, name, email, updated_at FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| map_db_error(e, "find_all", "user"))?
            .into_iter()
            .map(UserRow::into_domain)
            .collect())
    }

    async fn insert(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO users (id, name, email) VALUES ($1, $2, $3)")
            .bind(user.id().value())
            .bind(user.name())
            .bind(user.email().value())
            .execute(&self.pool)
            .await
            .map_err(|e| map_db_error(e, "insert", "user"))?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query(
            "UPDATE users SET name = $1, email = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
        )
        .bind(user.name())
        .bind(user.email().value())
        .bind(user.id().value())
        .execute(&self.pool)
        .await
        .map_err(|e| map_db_error(e, "update", "user"))?;
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<bool, DomainError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| map_db_error(e, "delete", "user"))?;
        Ok(result.rows_affected() > 0)
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    name: String,
    email: String,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl UserRow {
    fn into_domain(self) -> User {
        User::reconstitute(
            UserId::from_trusted(self.id),
            self.name,
            Email::from_trusted(self.email),
            self.updated_at,
        )
    }
}
