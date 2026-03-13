//! Activity handlers.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use oap_types::activity::{
    Activity, CreateActivityRequest, UpdateActivityRequest, Visibility, ParticipationMode,
};
use oap_types::common::Metadata;
use oap_types::ids::ActivityId;

use crate::errors::ApiError;
use crate::extractors::Pagination;
use crate::state::AppState;

/// POST /activities
pub async fn create_activity(
    State(state): State<AppState>,
    Json(req): Json<CreateActivityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let now = time::OffsetDateTime::now_utc();
    let activity = Activity {
        activity_id: ActivityId::new(),
        owner_actor_id: req.owner_actor_id,
        provider_profile_id: req.provider_profile_id,
        kind: req.kind,
        title: req.title,
        description: req.description,
        domain: req.domain,
        subcategory: req.subcategory,
        visibility: req.visibility,
        participation_mode: req.participation_mode,
        default_capacity: req.default_capacity,
        booking_rules_ref: None,
        cancellation_policy_ref: None,
        pricing_model: req.pricing_model,
        capabilities: Vec::new(),
        metadata: req.metadata,
        created_at: now,
        updated_at: now,
    };

    tracing::info!(activity_id = %activity.activity_id, "created activity");

    Ok((StatusCode::CREATED, Json(activity)))
}

/// GET /activities
pub async fn list_activities(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: fetch from repository with pagination
    let activities: Vec<Activity> = Vec::new();
    Ok(Json(activities))
}

/// GET /activities/:id
pub async fn get_activity(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Activity",
        id,
    }))
}

/// PATCH /activities/:id
pub async fn update_activity(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateActivityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Activity",
        id,
    }))
}
