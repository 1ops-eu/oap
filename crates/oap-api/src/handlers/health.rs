//! Health check and protocol info handlers.

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

/// GET /health
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[derive(Serialize)]
pub struct ProtocolInfoResponse {
    pub name: &'static str,
    pub protocol_version: &'static str,
    pub media_type: &'static str,
    pub server_version: &'static str,
}

/// GET /
pub async fn protocol_info() -> Json<ProtocolInfoResponse> {
    Json(ProtocolInfoResponse {
        name: "Open Activity Protocol",
        protocol_version: "oap/v0.1",
        media_type: "application/vnd.oap+json;version=0.1",
        server_version: env!("CARGO_PKG_VERSION"),
    })
}
