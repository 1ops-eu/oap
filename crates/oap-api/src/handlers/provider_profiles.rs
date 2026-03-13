//! ProviderProfile handlers.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use oap_types::provider::{
    CreateProviderProfileRequest, ProviderProfile, UpdateProviderProfileRequest,
};
use oap_types::ids::ProviderProfileId;

use crate::errors::ApiError;
use crate::state::AppState;

/// POST /provider-profiles
pub async fn create_provider_profile(
    State(state): State<AppState>,
    Json(req): Json<CreateProviderProfileRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let now = time::OffsetDateTime::now_utc();
    let profile = ProviderProfile {
        provider_profile_id: ProviderProfileId::new(),
        actor_id: req.actor_id,
        provider_type: req.provider_type,
        display_name: req.display_name,
        slug: req.slug,
        contact: req.contact,
        default_currency: req.default_currency,
        payment_account_ref: None,
        policies_ref: Vec::new(),
        capabilities: Vec::new(),
        metadata: req.metadata,
        created_at: now,
        updated_at: now,
    };

    tracing::info!(
        provider_profile_id = %profile.provider_profile_id,
        "created provider profile"
    );

    Ok((StatusCode::CREATED, Json(profile)))
}

/// GET /provider-profiles/:id
pub async fn get_provider_profile(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "ProviderProfile",
        id,
    }))
}

/// PATCH /provider-profiles/:id
pub async fn update_provider_profile(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateProviderProfileRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "ProviderProfile",
        id,
    }))
}
