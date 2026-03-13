//! Session — the concrete scheduled instance of an Activity.
//!
//! This is the primary bookable unit. A Session has a well-defined state machine
//! that governs its lifecycle.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::activity::Visibility;
use crate::common::{ConversationRef, Metadata, Price};
use crate::ids::{ActivityId, ActorId, LocationId, ProviderProfileId, SessionId};

/// Session lifecycle status.
///
/// State machine transitions:
/// ```text
/// draft -> scheduled -> open -> full -> in_progress -> completed -> archived
///                         |                                         ^
///                         +---> in_progress ---> completed ----------+
///
/// Any state -> cancelled (subject to business rules)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Draft,
    Scheduled,
    Open,
    Full,
    InProgress,
    Completed,
    Cancelled,
    Archived,
}

impl SessionStatus {
    /// Returns the set of statuses this status can transition to.
    #[must_use]
    pub fn allowed_transitions(&self) -> &'static [SessionStatus] {
        match self {
            Self::Draft => &[Self::Scheduled, Self::Cancelled],
            Self::Scheduled => &[Self::Open, Self::Cancelled],
            Self::Open => &[Self::Full, Self::InProgress, Self::Cancelled],
            Self::Full => &[Self::Open, Self::InProgress, Self::Cancelled],
            Self::InProgress => &[Self::Completed, Self::Cancelled],
            Self::Completed => &[Self::Archived],
            Self::Cancelled => &[],
            Self::Archived => &[],
        }
    }

    /// Check if transitioning to the target status is allowed.
    #[must_use]
    pub fn can_transition_to(&self, target: Self) -> bool {
        self.allowed_transitions().contains(&target)
    }
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Open => write!(f, "open"),
            Self::Full => write!(f, "full"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

/// A Session in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct Session {
    pub session_id: SessionId,
    pub activity_id: ActivityId,
    pub owner_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_profile_id: Option<ProviderProfileId>,

    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,

    /// IANA timezone name (e.g., "Europe/Berlin").
    pub timezone: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ref: Option<LocationId>,

    /// Maximum number of participants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,

    /// Current confirmed booking count.
    #[serde(default)]
    pub booked_count: i32,

    /// Current reserved (pending payment) count.
    #[serde(default)]
    pub reserved_count: i32,

    #[serde(default)]
    pub waitlist_enabled: bool,

    pub status: SessionStatus,
    pub visibility: Visibility,

    /// Optional price override for this specific session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_override: Option<Price>,

    /// Optional conversation/messaging thread reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_ref: Option<ConversationRef>,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

impl Session {
    /// Returns the number of available seats (capacity - booked - reserved).
    /// Returns `None` if capacity is unlimited.
    #[must_use]
    pub fn available_seats(&self) -> Option<i32> {
        self.capacity
            .map(|cap| (cap - self.booked_count - self.reserved_count).max(0))
    }

    /// Returns whether the session has available capacity.
    #[must_use]
    pub fn has_availability(&self) -> bool {
        match self.available_seats() {
            Some(seats) => seats > 0,
            None => true, // Unlimited capacity
        }
    }
}

/// Request body for creating a Session.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateSessionRequest {
    pub activity_id: ActivityId,
    pub owner_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_profile_id: Option<ProviderProfileId>,

    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,

    pub timezone: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ref: Option<LocationId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,

    #[serde(default)]
    pub waitlist_enabled: bool,

    #[serde(default = "default_session_visibility")]
    pub visibility: Visibility,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_override: Option<Price>,

    #[serde(default)]
    pub metadata: Metadata,
}

fn default_session_visibility() -> Visibility {
    Visibility::Public
}

/// Request body for updating a Session.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateSessionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub starts_at: Option<OffsetDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub ends_at: Option<OffsetDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ref: Option<LocationId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SessionStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Session availability response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionAvailability {
    pub session_id: SessionId,
    pub status: SessionStatus,
    pub capacity: Option<i32>,
    pub booked_count: i32,
    pub reserved_count: i32,
    pub available_seats: Option<i32>,
    pub waitlist_enabled: bool,
    pub has_availability: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_status_transitions() {
        assert!(SessionStatus::Draft.can_transition_to(SessionStatus::Scheduled));
        assert!(SessionStatus::Scheduled.can_transition_to(SessionStatus::Open));
        assert!(SessionStatus::Open.can_transition_to(SessionStatus::Full));
        assert!(SessionStatus::Open.can_transition_to(SessionStatus::InProgress));
        assert!(SessionStatus::Full.can_transition_to(SessionStatus::InProgress));
        assert!(SessionStatus::InProgress.can_transition_to(SessionStatus::Completed));
        assert!(SessionStatus::Completed.can_transition_to(SessionStatus::Archived));
    }

    #[test]
    fn test_session_status_invalid_transitions() {
        assert!(!SessionStatus::Draft.can_transition_to(SessionStatus::Open));
        assert!(!SessionStatus::Draft.can_transition_to(SessionStatus::Completed));
        assert!(!SessionStatus::Archived.can_transition_to(SessionStatus::Draft));
        assert!(!SessionStatus::Cancelled.can_transition_to(SessionStatus::Open));
        assert!(!SessionStatus::Completed.can_transition_to(SessionStatus::Open));
    }

    #[test]
    fn test_session_status_cancellation() {
        // Most states can transition to cancelled
        assert!(SessionStatus::Draft.can_transition_to(SessionStatus::Cancelled));
        assert!(SessionStatus::Scheduled.can_transition_to(SessionStatus::Cancelled));
        assert!(SessionStatus::Open.can_transition_to(SessionStatus::Cancelled));
        assert!(SessionStatus::Full.can_transition_to(SessionStatus::Cancelled));
        assert!(SessionStatus::InProgress.can_transition_to(SessionStatus::Cancelled));
        // But completed/archived/cancelled cannot
        assert!(!SessionStatus::Completed.can_transition_to(SessionStatus::Cancelled));
        assert!(!SessionStatus::Archived.can_transition_to(SessionStatus::Cancelled));
    }

    #[test]
    fn test_session_status_display() {
        assert_eq!(SessionStatus::Draft.to_string(), "draft");
        assert_eq!(SessionStatus::InProgress.to_string(), "in_progress");
        assert_eq!(SessionStatus::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_full_can_reopen() {
        // A full session can go back to open (e.g., after a cancellation frees a seat)
        assert!(SessionStatus::Full.can_transition_to(SessionStatus::Open));
    }
}
