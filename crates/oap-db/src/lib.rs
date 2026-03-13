//! # oap-db
//!
//! Database layer for the Open Activity Protocol.
//!
//! Provides repository traits, PostgreSQL implementations via SQLx,
//! and database migrations.

pub mod repo;

use sqlx::PgPool;

/// Create a PostgreSQL connection pool.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}

/// Run all pending database migrations.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
