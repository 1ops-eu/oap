//! ParticipantProfile handlers.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use oap_types::common::Metadata;
use oap_types::ids::ParticipantProfileId;
use oap_types::participant::{CreateParticipantProfileRequest, ParticipantProfile};

use crate::errors::ApiError;
use crate::state::AppState;

/// POST /participant-profiles
pub async fn create_participant_profile(
    State(state): State<AppState>,
    Json(req): Json<CreateParticipantProfileRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let now = time::OffsetDateTime::now_utc();
    let profile = ParticipantProfile {
        participant_profile_id: ParticipantProfileId::new(),
        actor_id: req.actor_id,
        preferences: req.preferences,
        capabilities: Vec::new(),
        metadata: req.metadata,
        created_at: now,
        updated_at: now,
    };

    tracing::info!(
        participant_profile_id = %profile.participant_profile_id,
        "created participant profile"
    );

    Ok((StatusCode::CREATED, Json(profile)))
}

/// GET /participant-profiles/:id
pub async fn get_participant_profile(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "ParticipantProfile",
        id,
    }))
}
