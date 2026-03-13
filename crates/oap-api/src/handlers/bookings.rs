//! Booking handlers.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use oap_types::booking::{
    Booking, BookingStatus, CancelBookingRequest, CancelledBy, CreateBookingRequest,
    PaymentRequirement, PaymentStatus,
};
use oap_types::common::Metadata;
use oap_types::ids::{ActivityId, BookingId};

use crate::errors::ApiError;
use crate::state::AppState;

/// POST /bookings
///
/// Creates a new booking. In a full implementation, this would:
/// 1. Validate the session exists and is open
/// 2. Check capacity
/// 3. Determine initial status based on payment requirements
/// 4. Reserve seats atomically
/// 5. Emit a webhook event via the outbox
pub async fn create_booking(
    State(state): State<AppState>,
    Json(req): Json<CreateBookingRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let now = time::OffsetDateTime::now_utc();

    // TODO: This should be transactional with capacity checks.
    // For now, this demonstrates the response shape.
    let booking = Booking {
        booking_id: BookingId::new(),
        session_id: req.session_id,
        activity_id: ActivityId::new(), // TODO: derive from session
        participant_actor_id: req.participant_actor_id,
        participant_profile_id: req.participant_profile_id,
        created_by_actor_id: req.participant_actor_id,
        status: BookingStatus::Pending,
        seat_count: req.seat_count,
        reservation_expires_at: None,
        payment_requirement: PaymentRequirement::NotRequired,
        payment_status: PaymentStatus::NotRequired,
        payment_intent_ref: None,
        package_consumption_ref: None,
        confirmed_at: None,
        cancelled_at: None,
        cancellation_reason: None,
        created_at: now,
        updated_at: now,
        metadata: req.metadata,
    };

    tracing::info!(booking_id = %booking.booking_id, "created booking");

    Ok((StatusCode::CREATED, Json(booking)))
}

/// GET /bookings/:id
pub async fn get_booking(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Booking",
        id,
    }))
}

/// POST /bookings/:id/cancel
pub async fn cancel_booking(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CancelBookingRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Fetch booking, validate transition, update status, release seats
    Err(ApiError::from(oap_domain::errors::OapError::NotFound {
        resource: "Booking",
        id,
    }))
}
