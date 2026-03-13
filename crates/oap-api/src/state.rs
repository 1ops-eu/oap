//! Shared application state.

use sqlx::PgPool;

use crate::config::AppConfig;

/// Shared application state available to all handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    /// PostgreSQL connection pool.
    pub db: PgPool,
    /// Application configuration.
    pub config: AppConfig,
}

impl AppState {
    /// Create new application state.
    pub fn new(db: PgPool, config: AppConfig) -> Self {
        Self { db, config }
    }
}
