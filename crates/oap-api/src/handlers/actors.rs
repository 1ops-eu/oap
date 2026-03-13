//! Actor handlers.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use oap_types::actor::{
    Actor, ActorType, CreateActorRequest, UpdateActorRequest, VerificationStatus,
};
use oap_types::common::Metadata;
use oap_types::ids::ActorId;

use crate::errors::ApiError;
use crate::state::AppState;

/// POST /actors
pub async fn create_actor(
    State(state): State<AppState>,
    Json(req): Json<CreateActorRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let now = time::OffsetDateTime::now_utc();
    let actor = Actor {
        actor_id: ActorId::new(),
        actor_type: req.actor_type,
        display_name: req.display_name,
        handle: req.handle,
        identity_ref: req.identity_ref,
        verification_status: VerificationStatus::Unverified,
        capabilities: Vec::new(),
        metadata: req.metadata,
        created_at: now,
        updated_at: now,
    };

    // TODO: persist via repository
    tracing::info!(actor_id = %actor.actor_id, "created actor");

    Ok((StatusCode::CREATED, Json(actor)))
}

/// GET /actors/:id
pub async fn get_actor(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: fetch from repository
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Actor",
        id,
    }))
}

/// PATCH /actors/:id
pub async fn update_actor(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateActorRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: update via repository
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Actor",
        id,
    }))
}
