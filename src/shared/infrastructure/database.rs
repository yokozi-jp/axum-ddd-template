//! Database connection and pool management

use crate::shared::infrastructure::config::Config;
use sqlx::{postgres::PgPoolOptions, PgPool};

/// Create database connection pool with configurable settings
pub async fn create_pool(config: &Config) -> Result<PgPool, anyhow::Error> {
    Ok(PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .acquire_timeout(config.db_acquire_timeout())
        .idle_timeout(config.db_idle_timeout())
        .connect(&config.database_url)
        .await?)
}

/// Run pending migrations from the `migrations/` directory.
///
/// Uses sqlx's built-in migration runner which tracks applied migrations
/// in a `_sqlx_migrations` table and verifies checksums.
pub async fn run_migrations(pool: &PgPool) -> Result<(), anyhow::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

/// Map a sqlx error to a `DomainError`, checking for common `PostgreSQL` constraint codes.
///
/// - `23505` `unique_violation` → `DomainError::AlreadyExists`
/// - `23503` `foreign_key_violation` → `DomainError::NotFound`
/// - anything else → `DomainError::Infrastructure`
#[expect(clippy::needless_pass_by_value, reason = "sqlx::Error is not Clone; consumed by value")]
pub fn map_db_error(e: sqlx::Error, operation: &str, entity: &str) -> crate::shared::domain::DomainError {
    use crate::shared::domain::DomainError;
    if let sqlx::Error::Database(ref db_err) = e {
        match db_err.code().as_deref() {
            Some("23505") => {
                let msg = db_err.message();
                return if msg.contains("email") {
                    DomainError::AlreadyExists("Email already exists".into())
                } else {
                    DomainError::AlreadyExists(format!("{entity} already exists"))
                };
            }
            Some("23503") => return DomainError::NotFound(format!("{entity} not found")),
            _ => {}
        }
    }
    tracing::error!("Database error in {operation}: {e}");
    DomainError::Infrastructure(format!("Failed to {operation} {entity}"))
}
