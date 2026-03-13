//! Domain error types.

use oap_types::booking::BookingStatus;
use oap_types::session::SessionStatus;
use thiserror::Error;
use uuid::Uuid;

/// Domain-level errors for OAP operations.
#[derive(Debug, Error)]
pub enum OapError {
    /// Invalid state transition.
    #[error("invalid transition: {resource} cannot transition from `{from}` to `{to}`")]
    InvalidTransition {
        resource: &'static str,
        from: String,
        to: String,
    },

    /// Resource not found.
    #[error("{resource} not found: {id}")]
    NotFound { resource: &'static str, id: Uuid },

    /// Capacity exceeded — no available seats.
    #[error("session {session_id} has no available seats (capacity: {capacity}, booked: {booked}, reserved: {reserved})")]
    CapacityExceeded {
        session_id: Uuid,
        capacity: i32,
        booked: i32,
        reserved: i32,
    },

    /// Reservation has expired.
    #[error("reservation for booking {booking_id} has expired")]
    ReservationExpired { booking_id: Uuid },

    /// Duplicate detected (idempotency).
    #[error("duplicate request detected (idempotency key: {key})")]
    DuplicateRequest { key: String },

    /// Validation error.
    #[error("validation error: {message}")]
    Validation { message: String },

    /// Conflict — concurrent modification detected.
    #[error("conflict: {message}")]
    Conflict { message: String },

    /// Authorization error.
    #[error("unauthorized: {message}")]
    Unauthorized { message: String },

    /// Internal error.
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl OapError {
    /// Create an invalid session transition error.
    pub fn invalid_session_transition(from: SessionStatus, to: SessionStatus) -> Self {
        Self::InvalidTransition {
            resource: "Session",
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    /// Create an invalid booking transition error.
    pub fn invalid_booking_transition(from: BookingStatus, to: BookingStatus) -> Self {
        Self::InvalidTransition {
            resource: "Booking",
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

/// Result type alias for OAP domain operations.
pub type OapResult<T> = Result<T, OapError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = OapError::invalid_session_transition(
            SessionStatus::Draft,
            SessionStatus::Completed,
        );
        let msg = err.to_string();
        assert!(msg.contains("Session"));
        assert!(msg.contains("draft"));
        assert!(msg.contains("completed"));
    }

    #[test]
    fn test_capacity_exceeded_display() {
        let err = OapError::CapacityExceeded {
            session_id: Uuid::nil(),
            capacity: 4,
            booked: 3,
            reserved: 1,
        };
        let msg = err.to_string();
        assert!(msg.contains("no available seats"));
        assert!(msg.contains("capacity: 4"));
    }
}
