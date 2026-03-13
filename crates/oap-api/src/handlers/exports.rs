//! Data export handlers.
//!
//! OAP supports portable data exports as canonical JSON bundles.
//! An open protocol without export/import is not meaningfully open.

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::Serialize;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::state::AppState;

/// Export bundle envelope.
#[derive(Debug, Serialize)]
pub struct ExportBundle {
    /// OAP protocol version.
    pub protocol_version: &'static str,
    /// Export format version.
    pub export_version: &'static str,
    /// ISO 8601 timestamp of export.
    pub exported_at: String,
    /// The exported resource.
    pub data: serde_json::Value,
}

/// GET /exports/actors/:id
pub async fn export_actor(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: fetch actor and format as export bundle
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Actor",
        id,
    }))
}

/// GET /exports/provider-profiles/:id
pub async fn export_provider_profile(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: fetch provider profile and format as export bundle
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "ProviderProfile",
        id,
    }))
}
