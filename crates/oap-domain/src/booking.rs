//! Booking domain service.
//!
//! Orchestrates the booking flow: capacity check → reservation → payment → confirmation.
//! This module defines the high-level booking workflow as a domain service trait.

use oap_types::booking::{Booking, BookingStatus, CreateBookingRequest, PaymentRequirement, PaymentStatus};
use oap_types::ids::{BookingId, SessionId};
use oap_types::session::Session;
use time::OffsetDateTime;

use crate::capacity;
use crate::errors::{OapError, OapResult};

/// Configuration for the booking service.
#[derive(Debug, Clone)]
pub struct BookingConfig {
    /// Default reservation window duration in minutes.
    pub reservation_window_minutes: i64,
}

impl Default for BookingConfig {
    fn default() -> Self {
        Self {
            reservation_window_minutes: 15,
        }
    }
}

/// Determines the initial booking status based on payment requirements and capacity.
///
/// This encodes the booking flow logic:
/// 1. If no payment required and seats available → `Confirmed`
/// 2. If payment required → `RequiresPayment`
/// 3. If no seats available and waitlist enabled → `Waitlisted`
/// 4. If no seats available and no waitlist → error
pub fn determine_initial_status(
    session: &Session,
    payment_requirement: PaymentRequirement,
    requested_seats: i32,
) -> OapResult<BookingStatus> {
    // Check capacity
    let has_capacity = capacity::check_availability(session, requested_seats).is_ok();

    if has_capacity {
        match payment_requirement {
            PaymentRequirement::NotRequired => Ok(BookingStatus::Confirmed),
            PaymentRequirement::RequiredBeforeConfirmation => Ok(BookingStatus::RequiresPayment),
            PaymentRequirement::RequiredDeferred => Ok(BookingStatus::Confirmed),
            PaymentRequirement::PackageCredit => Ok(BookingStatus::Confirmed),
        }
    } else if session.waitlist_enabled {
        Ok(BookingStatus::Waitlisted)
    } else {
        Err(OapError::CapacityExceeded {
            session_id: session.session_id.into_inner(),
            capacity: session.capacity.unwrap_or(0),
            booked: session.booked_count,
            reserved: session.reserved_count,
        })
    }
}

/// Calculate the reservation expiry time for a booking.
pub fn calculate_reservation_expiry(
    config: &BookingConfig,
    now: OffsetDateTime,
) -> OffsetDateTime {
    now + time::Duration::minutes(config.reservation_window_minutes)
}

/// Determine the initial payment status based on the requirement.
pub fn determine_initial_payment_status(requirement: PaymentRequirement) -> PaymentStatus {
    match requirement {
        PaymentRequirement::NotRequired => PaymentStatus::NotRequired,
        PaymentRequirement::RequiredBeforeConfirmation => PaymentStatus::Pending,
        PaymentRequirement::RequiredDeferred => PaymentStatus::Pending,
        PaymentRequirement::PackageCredit => PaymentStatus::NotRequired,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oap_types::activity::Visibility;
    use oap_types::common::Metadata;
    use oap_types::ids::{ActivityId, ActorId};
    use oap_types::session::{Session, SessionStatus};

    fn make_session(capacity: Option<i32>, booked: i32, reserved: i32, waitlist: bool) -> Session {
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
            waitlist_enabled: waitlist,
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
    fn test_free_booking_confirmed() {
        let session = make_session(Some(10), 2, 0, false);
        let status = determine_initial_status(&session, PaymentRequirement::NotRequired, 1);
        assert_eq!(status.unwrap(), BookingStatus::Confirmed);
    }

    #[test]
    fn test_paid_booking_requires_payment() {
        let session = make_session(Some(10), 2, 0, false);
        let status = determine_initial_status(&session, PaymentRequirement::RequiredBeforeConfirmation, 1);
        assert_eq!(status.unwrap(), BookingStatus::RequiresPayment);
    }

    #[test]
    fn test_full_session_with_waitlist() {
        let session = make_session(Some(4), 4, 0, true);
        let status = determine_initial_status(&session, PaymentRequirement::NotRequired, 1);
        assert_eq!(status.unwrap(), BookingStatus::Waitlisted);
    }

    #[test]
    fn test_full_session_no_waitlist() {
        let session = make_session(Some(4), 4, 0, false);
        let result = determine_initial_status(&session, PaymentRequirement::NotRequired, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_deferred_payment_confirmed() {
        let session = make_session(Some(10), 0, 0, false);
        let status = determine_initial_status(&session, PaymentRequirement::RequiredDeferred, 1);
        assert_eq!(status.unwrap(), BookingStatus::Confirmed);
    }

    #[test]
    fn test_reservation_expiry() {
        let config = BookingConfig::default();
        let now = OffsetDateTime::now_utc();
        let expiry = calculate_reservation_expiry(&config, now);
        assert!(expiry > now);
        assert_eq!(
            (expiry - now).whole_minutes(),
            config.reservation_window_minutes
        );
    }
}
