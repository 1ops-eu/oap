//! OAP API Server — Reference Runtime
//!
//! The main entry point for the OAP reference runtime, built on Axum.

mod config;
mod errors;
mod extractors;
mod handlers;
mod router;
mod state;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "oap_api=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::AppConfig::from_env()?;
    let bind_addr = format!("{}:{}", config.host, config.port);

    info!(
        host = %config.host,
        port = config.port,
        "starting OAP API server"
    );

    // Create database connection pool
    let pool = oap_db::create_pool(&config.database_url).await?;

    // Run migrations
    info!("running database migrations");
    oap_db::run_migrations(&pool).await?;
    info!("migrations complete");

    // Build application state
    let app_state = state::AppState::new(pool, config);

    // Build router
    let app = router::create_router(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!(addr = %bind_addr, "OAP API server listening");
    axum::serve(listener, app).await?;

    Ok(())
}
