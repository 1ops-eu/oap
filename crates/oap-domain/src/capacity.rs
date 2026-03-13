//! Capacity management logic.
//!
//! Handles seat availability checks, reservation semantics, and
//! capacity-aware booking decisions.

use oap_types::ids::SessionId;
use oap_types::session::Session;

use crate::errors::{OapError, OapResult};

/// Check whether a session has enough available seats for the requested count.
///
/// Returns `Ok(available)` if seats are available, or `Err(CapacityExceeded)` if not.
pub fn check_availability(session: &Session, requested_seats: i32) -> OapResult<i32> {
    match session.capacity {
        None => {
            // Unlimited capacity
            Ok(i32::MAX)
        }
        Some(capacity) => {
            let available = (capacity - session.booked_count - session.reserved_count).max(0);
            if available >= requested_seats {
                Ok(available)
            } else {
                Err(OapError::CapacityExceeded {
                    session_id: session.session_id.into_inner(),
                    capacity,
                    booked: session.booked_count,
                    reserved: session.reserved_count,
                })
            }
        }
    }
}

/// Determine if a session should transition to `Full` status based on capacity.
///
/// Returns `true` if the session has zero available seats.
pub fn is_at_capacity(session: &Session) -> bool {
    match session.capacity {
        None => false,
        Some(capacity) => {
            let used = session.booked_count + session.reserved_count;
            used >= capacity
        }
    }
}

/// Calculate the effective available seats after a hypothetical booking.
pub fn seats_after_booking(session: &Session, seat_count: i32) -> Option<i32> {
    session
        .capacity
        .map(|cap| (cap - session.booked_count - session.reserved_count - seat_count).max(0))
}

/// Reservation window check — determine if a reservation has expired.
pub fn is_reservation_expired(
    reservation_expires_at: Option<time::OffsetDateTime>,
    now: time::OffsetDateTime,
) -> bool {
    match reservation_expires_at {
        Some(expires_at) => now >= expires_at,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oap_types::activity::Visibility;
    use oap_types::common::Metadata;
    use oap_types::ids::{ActivityId, ActorId};
    use oap_types::session::{Session, SessionStatus};
    use time::OffsetDateTime;

    fn make_session(capacity: Option<i32>, booked: i32, reserved: i32) -> Session {
        Session {
            session_id: SessionId::new(),
            activity_id: ActivityId::new(),
            owner_actor_id: ActorId::new(),
            provider_profile_id: None,
            starts_at: OffsetDateTime::now_utc(),
            ends_at: OffsetDateTime::now_utc(),
            timezone: "UTC".to_string(),
            location_ref: None,
            capacity,
            booked_count: booked,
            reserved_count: reserved,
            waitlist_enabled: false,
            status: SessionStatus::Open,
            visibility: Visibility::Public,
            price_override: None,
            conversation_ref: None,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            metadata: Metadata::new(),
        }
    }

    #[test]
    fn test_availability_with_seats() {
        let session = make_session(Some(10), 3, 2);
        let result = check_availability(&session, 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_availability_no_seats() {
        let session = make_session(Some(4), 3, 1);
        let result = check_availability(&session, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_availability_unlimited() {
        let session = make_session(None, 100, 50);
        let result = check_availability(&session, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_at_capacity() {
        let session = make_session(Some(4), 3, 1);
        assert!(is_at_capacity(&session));
    }

    #[test]
    fn test_not_at_capacity() {
        let session = make_session(Some(10), 3, 1);
        assert!(!is_at_capacity(&session));
    }

    #[test]
    fn test_unlimited_never_at_capacity() {
        let session = make_session(None, 1000, 500);
        assert!(!is_at_capacity(&session));
    }

    #[test]
    fn test_reservation_not_expired() {
        let future = OffsetDateTime::now_utc() + time::Duration::hours(1);
        assert!(!is_reservation_expired(Some(future), OffsetDateTime::now_utc()));
    }

    #[test]
    fn test_reservation_expired() {
        let past = OffsetDateTime::now_utc() - time::Duration::hours(1);
        assert!(is_reservation_expired(Some(past), OffsetDateTime::now_utc()));
    }

    #[test]
    fn test_no_reservation_never_expired() {
        assert!(!is_reservation_expired(None, OffsetDateTime::now_utc()));
    }
}
