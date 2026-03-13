//! Webhook endpoint handlers.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use oap_types::ids::WebhookEndpointId;
use oap_types::webhook::{CreateWebhookEndpointRequest, WebhookEndpoint};

use crate::errors::ApiError;
use crate::state::AppState;

/// POST /webhook-endpoints
pub async fn create_webhook_endpoint(
    State(state): State<AppState>,
    Json(req): Json<CreateWebhookEndpointRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let now = time::OffsetDateTime::now_utc();

    // Generate a signing secret for this endpoint
    let secret = uuid::Uuid::now_v7().to_string(); // TODO: use proper random secret

    let endpoint = WebhookEndpoint {
        webhook_endpoint_id: WebhookEndpointId::new(),
        owner_actor_id: req.owner_actor_id,
        url: req.url,
        subscribed_events: req.subscribed_events,
        active: true,
        secret,
        metadata: req.metadata,
        created_at: now,
        updated_at: now,
    };

    tracing::info!(
        webhook_endpoint_id = %endpoint.webhook_endpoint_id,
        "created webhook endpoint"
    );

    Ok((StatusCode::CREATED, Json(endpoint)))
}

/// GET /webhook-endpoints
pub async fn list_webhook_endpoints(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let endpoints: Vec<WebhookEndpoint> = Vec::new();
    Ok(Json(endpoints))
}
