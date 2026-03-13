//! State transition validation.
//!
//! Provides validated transition functions that return domain errors
//! on invalid state changes. The raw `can_transition_to` logic lives
//! in `oap-types`; this module wraps it with error semantics.

use oap_types::booking::BookingStatus;
use oap_types::session::SessionStatus;

use crate::errors::{OapError, OapResult};

/// Validate and execute a session status transition.
///
/// Returns the new status on success, or an `OapError::InvalidTransition` on failure.
pub fn transition_session(current: SessionStatus, target: SessionStatus) -> OapResult<SessionStatus> {
    if current.can_transition_to(target) {
        Ok(target)
    } else {
        Err(OapError::invalid_session_transition(current, target))
    }
}

/// Validate and execute a booking status transition.
///
/// Returns the new status on success, or an `OapError::InvalidTransition` on failure.
pub fn transition_booking(current: BookingStatus, target: BookingStatus) -> OapResult<BookingStatus> {
    if current.can_transition_to(target) {
        Ok(target)
    } else {
        Err(OapError::invalid_booking_transition(current, target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_session_transition() {
        let result = transition_session(SessionStatus::Draft, SessionStatus::Scheduled);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SessionStatus::Scheduled);
    }

    #[test]
    fn test_invalid_session_transition() {
        let result = transition_session(SessionStatus::Archived, SessionStatus::Open);
        assert!(result.is_err());
        match result.unwrap_err() {
            OapError::InvalidTransition { resource, .. } => {
                assert_eq!(resource, "Session");
            }
            _ => panic!("Expected InvalidTransition error"),
        }
    }

    #[test]
    fn test_valid_booking_transition() {
        let result = transition_booking(BookingStatus::Pending, BookingStatus::Confirmed);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BookingStatus::Confirmed);
    }

    #[test]
    fn test_invalid_booking_transition() {
        let result = transition_booking(BookingStatus::Expired, BookingStatus::Confirmed);
        assert!(result.is_err());
        match result.unwrap_err() {
            OapError::InvalidTransition { resource, .. } => {
                assert_eq!(resource, "Booking");
            }
            _ => panic!("Expected InvalidTransition error"),
        }
    }

    #[test]
    fn test_full_session_lifecycle() {
        let mut status = SessionStatus::Draft;
        status = transition_session(status, SessionStatus::Scheduled).unwrap();
        status = transition_session(status, SessionStatus::Open).unwrap();
        status = transition_session(status, SessionStatus::InProgress).unwrap();
        status = transition_session(status, SessionStatus::Completed).unwrap();
        let result = transition_session(status, SessionStatus::Archived);
        assert!(result.is_ok());
    }

    #[test]
    fn test_full_booking_lifecycle() {
        let mut status = BookingStatus::Pending;
        status = transition_booking(status, BookingStatus::RequiresPayment).unwrap();
        status = transition_booking(status, BookingStatus::Reserved).unwrap();
        status = transition_booking(status, BookingStatus::Confirmed).unwrap();
        let result = transition_booking(status, BookingStatus::Attended);
        assert!(result.is_ok());
    }
}
