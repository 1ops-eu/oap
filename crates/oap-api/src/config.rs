//! Application configuration loaded from environment variables.

use anyhow::{Context, Result};

/// Application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Database connection URL.
    pub database_url: String,
    /// Server bind host.
    pub host: String,
    /// Server bind port.
    pub port: u16,
    /// Webhook signing secret.
    pub webhook_secret: String,
}

impl AppConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            host: std::env::var("OAP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("OAP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("OAP_PORT must be a valid port number")?,
            webhook_secret: std::env::var("OAP_WEBHOOK_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".to_string()),
        })
    }
}
