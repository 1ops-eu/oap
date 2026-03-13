//! Booking — a participant's reservation or registration for a Session.
//!
//! Booking is explicitly separate from payment, attendance, and identity details.
//! It has its own state machine governing its lifecycle.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::{Metadata, PaymentIntentRef};
use crate::ids::{ActivityId, ActorId, BookingId, ParticipantProfileId, SessionId};

/// Booking lifecycle status.
///
/// State machine transitions:
/// ```text
/// pending -> requires_payment -> reserved -> confirmed
///    |              |                            |
///    +-> reserved --+                            +-> attended
///    |                                           +-> no_show
///    +-> confirmed                               +-> cancelled_by_participant
///    |                                           +-> cancelled_by_provider
///    +-> waitlisted
///    |
///    +-> expired
///    +-> cancelled_by_participant
///    +-> cancelled_by_provider
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum BookingStatus {
    Pending,
    RequiresPayment,
    Reserved,
    Confirmed,
    Waitlisted,
    CancelledByParticipant,
    CancelledByProvider,
    Expired,
    Attended,
    NoShow,
    Refunded,
}

impl BookingStatus {
    /// Returns the set of statuses this status can transition to.
    #[must_use]
    pub fn allowed_transitions(&self) -> &'static [BookingStatus] {
        match self {
            Self::Pending => &[
                Self::RequiresPayment,
                Self::Reserved,
                Self::Confirmed,
                Self::Waitlisted,
                Self::CancelledByParticipant,
                Self::CancelledByProvider,
                Self::Expired,
            ],
            Self::RequiresPayment => &[
                Self::Reserved,
                Self::Confirmed,
                Self::Expired,
                Self::CancelledByParticipant,
                Self::CancelledByProvider,
            ],
            Self::Reserved => &[
                Self::Confirmed,
                Self::Expired,
                Self::CancelledByParticipant,
                Self::CancelledByProvider,
            ],
            Self::Confirmed => &[
                Self::Attended,
                Self::NoShow,
                Self::CancelledByParticipant,
                Self::CancelledByProvider,
                Self::Refunded,
            ],
            Self::Waitlisted => &[
                Self::Pending,
                Self::RequiresPayment,
                Self::Reserved,
                Self::CancelledByParticipant,
                Self::CancelledByProvider,
                Self::Expired,
            ],
            Self::CancelledByParticipant => &[Self::Refunded],
            Self::CancelledByProvider => &[Self::Refunded],
            Self::Expired => &[],
            Self::Attended => &[],
            Self::NoShow => &[],
            Self::Refunded => &[],
        }
    }

    /// Check if transitioning to the target status is allowed.
    #[must_use]
    pub fn can_transition_to(&self, target: Self) -> bool {
        self.allowed_transitions().contains(&target)
    }

    /// Whether this booking is currently "active" (holds or counts toward capacity).
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            Self::Pending | Self::RequiresPayment | Self::Reserved | Self::Confirmed
        )
    }

    /// Whether this booking is in a terminal state.
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Expired
                | Self::Attended
                | Self::NoShow
                | Self::Refunded
                | Self::CancelledByParticipant
                | Self::CancelledByProvider
        )
    }
}

impl std::fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::RequiresPayment => write!(f, "requires_payment"),
            Self::Reserved => write!(f, "reserved"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::Waitlisted => write!(f, "waitlisted"),
            Self::CancelledByParticipant => write!(f, "cancelled_by_participant"),
            Self::CancelledByProvider => write!(f, "cancelled_by_provider"),
            Self::Expired => write!(f, "expired"),
            Self::Attended => write!(f, "attended"),
            Self::NoShow => write!(f, "no_show"),
            Self::Refunded => write!(f, "refunded"),
        }
    }
}

/// Payment requirement for a booking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PaymentRequirement {
    /// No payment needed.
    NotRequired,
    /// Payment required before confirmation.
    RequiredBeforeConfirmation,
    /// Payment required but can be deferred.
    RequiredDeferred,
    /// Package/credit will be consumed.
    PackageCredit,
}

/// Payment status tracking (interoperable surface only — OAP is not a payment processor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    NotRequired,
    Pending,
    Authorized,
    Captured,
    Failed,
    Refunded,
    PartiallyRefunded,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotRequired => write!(f, "not_required"),
            Self::Pending => write!(f, "pending"),
            Self::Authorized => write!(f, "authorized"),
            Self::Captured => write!(f, "captured"),
            Self::Failed => write!(f, "failed"),
            Self::Refunded => write!(f, "refunded"),
            Self::PartiallyRefunded => write!(f, "partially_refunded"),
        }
    }
}

/// A Booking in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct Booking {
    pub booking_id: BookingId,
    pub session_id: SessionId,
    pub activity_id: ActivityId,
    pub participant_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_profile_id: Option<ParticipantProfileId>,

    /// The actor who created this booking (may differ from participant).
    pub created_by_actor_id: ActorId,

    pub status: BookingStatus,

    /// Number of seats this booking covers.
    #[serde(default = "default_seat_count")]
    pub seat_count: i32,

    /// Expiration time for a reservation hold.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        with = "time::serde::rfc3339::option"
    )]
    pub reservation_expires_at: Option<OffsetDateTime>,

    pub payment_requirement: PaymentRequirement,
    pub payment_status: PaymentStatus,

    /// External payment intent reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_ref: Option<PaymentIntentRef>,

    /// Reference to a package credit consumption (v0.2+).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_consumption_ref: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        with = "time::serde::rfc3339::option"
    )]
    pub confirmed_at: Option<OffsetDateTime>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        with = "time::serde::rfc3339::option"
    )]
    pub cancelled_at: Option<OffsetDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

fn default_seat_count() -> i32 {
    1
}

/// Request body for creating a Booking.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateBookingRequest {
    pub session_id: SessionId,
    pub participant_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_profile_id: Option<ParticipantProfileId>,

    #[serde(default = "default_seat_count")]
    pub seat_count: i32,

    #[serde(default)]
    pub metadata: Metadata,
}

/// Request body for cancelling a Booking.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CancelBookingRequest {
    /// Who is cancelling (participant or provider).
    pub cancelled_by: CancelledBy,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Who initiated the cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CancelledBy {
    Participant,
    Provider,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_booking_status_transitions() {
        assert!(BookingStatus::Pending.can_transition_to(BookingStatus::Confirmed));
        assert!(BookingStatus::Pending.can_transition_to(BookingStatus::RequiresPayment));
        assert!(BookingStatus::Pending.can_transition_to(BookingStatus::Reserved));
        assert!(BookingStatus::Pending.can_transition_to(BookingStatus::Waitlisted));
        assert!(BookingStatus::RequiresPayment.can_transition_to(BookingStatus::Confirmed));
        assert!(BookingStatus::Reserved.can_transition_to(BookingStatus::Confirmed));
        assert!(BookingStatus::Confirmed.can_transition_to(BookingStatus::Attended));
        assert!(BookingStatus::Confirmed.can_transition_to(BookingStatus::NoShow));
    }

    #[test]
    fn test_booking_status_invalid_transitions() {
        assert!(!BookingStatus::Expired.can_transition_to(BookingStatus::Confirmed));
        assert!(!BookingStatus::Attended.can_transition_to(BookingStatus::Confirmed));
        assert!(!BookingStatus::NoShow.can_transition_to(BookingStatus::Confirmed));
        assert!(!BookingStatus::Refunded.can_transition_to(BookingStatus::Confirmed));
    }

    #[test]
    fn test_booking_status_cancellation() {
        assert!(BookingStatus::Pending.can_transition_to(BookingStatus::CancelledByParticipant));
        assert!(BookingStatus::Pending.can_transition_to(BookingStatus::CancelledByProvider));
        assert!(BookingStatus::Confirmed.can_transition_to(BookingStatus::CancelledByParticipant));
        assert!(BookingStatus::Confirmed.can_transition_to(BookingStatus::CancelledByProvider));
    }

    #[test]
    fn test_booking_status_is_active() {
        assert!(BookingStatus::Pending.is_active());
        assert!(BookingStatus::RequiresPayment.is_active());
        assert!(BookingStatus::Reserved.is_active());
        assert!(BookingStatus::Confirmed.is_active());
        assert!(!BookingStatus::Expired.is_active());
        assert!(!BookingStatus::CancelledByParticipant.is_active());
        assert!(!BookingStatus::Attended.is_active());
    }

    #[test]
    fn test_booking_status_is_terminal() {
        assert!(!BookingStatus::Pending.is_terminal());
        assert!(!BookingStatus::Confirmed.is_terminal());
        assert!(BookingStatus::Expired.is_terminal());
        assert!(BookingStatus::Attended.is_terminal());
        assert!(BookingStatus::NoShow.is_terminal());
        assert!(BookingStatus::Refunded.is_terminal());
        assert!(BookingStatus::CancelledByParticipant.is_terminal());
    }

    #[test]
    fn test_waitlisted_can_be_promoted() {
        assert!(BookingStatus::Waitlisted.can_transition_to(BookingStatus::Pending));
        assert!(BookingStatus::Waitlisted.can_transition_to(BookingStatus::RequiresPayment));
        assert!(BookingStatus::Waitlisted.can_transition_to(BookingStatus::Reserved));
    }
}
