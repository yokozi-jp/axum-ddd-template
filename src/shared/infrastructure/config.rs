//! Application configuration

use std::net::SocketAddr;
use std::time::Duration;

/// Parse an environment variable with a default value, providing clear error context
fn parse_env_or<T>(key: &str, default: T) -> Result<T, anyhow::Error>
where
    T: std::str::FromStr + std::fmt::Display,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    match std::env::var(key) {
        Ok(val) => val
            .parse()
            .map_err(|e: T::Err| anyhow::anyhow!("Failed to parse {key}={val:?}: {e}")),
        Err(_) => Ok(default),
    }
}

/// Application configuration
#[derive(Debug)]
pub struct Config {
    /// Database connection URL
    pub database_url: String,
    /// Server socket address
    pub server_addr: SocketAddr,
    /// Maximum database connections
    pub db_max_connections: u32,
    /// Minimum database connections
    pub db_min_connections: u32,
    /// Database connection acquire timeout in seconds
    db_acquire_timeout_secs: u64,
    /// Database idle connection timeout in seconds
    db_idle_timeout_secs: u64,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenvy::dotenv().ok();

        let host: String = parse_env_or("SERVER_HOST", "0.0.0.0".to_string())?;
        let port = parse_env_or("SERVER_PORT", 3000u16)?;
        let server_addr = format!("{host}:{port}")
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid SERVER_HOST or SERVER_PORT: {e}"))?;

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|e| anyhow::anyhow!("DATABASE_URL is required: {e}"))?,
            server_addr,
            db_max_connections: parse_env_or("DB_MAX_CONNECTIONS", 10)?,
            db_min_connections: parse_env_or("DB_MIN_CONNECTIONS", 2)?,
            db_acquire_timeout_secs: parse_env_or("DB_ACQUIRE_TIMEOUT_SECS", 30)?,
            db_idle_timeout_secs: parse_env_or("DB_IDLE_TIMEOUT_SECS", 600)?,
        })
    }

    /// Get database acquire timeout as Duration
    pub fn db_acquire_timeout(&self) -> Duration {
        Duration::from_secs(self.db_acquire_timeout_secs)
    }

    /// Get database idle timeout as Duration
    pub fn db_idle_timeout(&self) -> Duration {
        Duration::from_secs(self.db_idle_timeout_secs)
    }
}
