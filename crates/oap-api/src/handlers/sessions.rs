//! Session handlers.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use oap_types::ids::SessionId;
use oap_types::session::{
    CreateSessionRequest, Session, SessionAvailability, SessionStatus, UpdateSessionRequest,
};

use crate::errors::ApiError;
use crate::extractors::Pagination;
use crate::state::AppState;

/// POST /sessions
pub async fn create_session(
    State(state): State<AppState>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let now = time::OffsetDateTime::now_utc();
    let session = Session {
        session_id: SessionId::new(),
        activity_id: req.activity_id,
        owner_actor_id: req.owner_actor_id,
        provider_profile_id: req.provider_profile_id,
        starts_at: req.starts_at,
        ends_at: req.ends_at,
        timezone: req.timezone,
        location_ref: req.location_ref,
        capacity: req.capacity,
        booked_count: 0,
        reserved_count: 0,
        waitlist_enabled: req.waitlist_enabled,
        status: SessionStatus::Draft,
        visibility: req.visibility,
        price_override: req.price_override,
        conversation_ref: None,
        created_at: now,
        updated_at: now,
        metadata: req.metadata,
    };

    tracing::info!(session_id = %session.session_id, "created session");

    Ok((StatusCode::CREATED, Json(session)))
}

/// GET /sessions
pub async fn list_sessions(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, ApiError> {
    let sessions: Vec<Session> = Vec::new();
    Ok(Json(sessions))
}

/// GET /sessions/:id
pub async fn get_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Session",
        id,
    }))
}

/// PATCH /sessions/:id
pub async fn update_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSessionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Session",
        id,
    }))
}

/// GET /sessions/:id/availability
pub async fn get_session_availability(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: fetch session and compute availability
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Session",
        id,
    }))
}
