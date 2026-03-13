//! Typed ID wrappers using UUIDv7.
//!
//! All core OAP resources use UUIDv7 for globally unique, time-sortable identifiers.
//! Each resource type gets a distinct newtype wrapper for compile-time safety.

use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

/// Generate a new UUIDv7 (time-sortable, globally unique).
pub fn new_id() -> Uuid {
    Uuid::now_v7()
}

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
        #[serde(transparent)]
        pub struct $name(pub Uuid);

        impl $name {
            /// Create a new random ID (UUIDv7).
            #[must_use]
            pub fn new() -> Self {
                Self(new_id())
            }

            /// Create from an existing UUID.
            #[must_use]
            pub const fn from_uuid(id: Uuid) -> Self {
                Self(id)
            }

            /// Get the inner UUID.
            #[must_use]
            pub const fn into_inner(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<Uuid> for $name {
            fn from(id: Uuid) -> Self {
                Self(id)
            }
        }

        impl From<$name> for Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

define_id!(
    /// Unique identifier for an Actor.
    ActorId
);

define_id!(
    /// Unique identifier for a ProviderProfile.
    ProviderProfileId
);

define_id!(
    /// Unique identifier for a ParticipantProfile.
    ParticipantProfileId
);

define_id!(
    /// Unique identifier for an Activity.
    ActivityId
);

define_id!(
    /// Unique identifier for a Session.
    SessionId
);

define_id!(
    /// Unique identifier for a Booking.
    BookingId
);

define_id!(
    /// Unique identifier for an Attendance record.
    AttendanceId
);

define_id!(
    /// Unique identifier for a Location.
    LocationId
);

define_id!(
    /// Unique identifier for a Policy.
    PolicyId
);

define_id!(
    /// Unique identifier for a Package.
    PackageId
);

define_id!(
    /// Unique identifier for a Membership.
    MembershipId
);

define_id!(
    /// Unique identifier for a WebhookEndpoint.
    WebhookEndpointId
);

define_id!(
    /// Unique identifier for a WebhookEvent.
    WebhookEventId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let id1 = ActorId::new();
        let id2 = ActorId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_display() {
        let id = ActorId::new();
        let display = format!("{id}");
        assert!(!display.is_empty());
        // UUIDv7 has standard format
        assert_eq!(display.len(), 36);
    }

    #[test]
    fn test_id_roundtrip() {
        let id = SessionId::new();
        let uuid: Uuid = id.into();
        let back = SessionId::from(uuid);
        assert_eq!(id, back);
    }

    #[test]
    fn test_id_serde_roundtrip() {
        let id = BookingId::new();
        let json = serde_json::to_string(&id).expect("serialize");
        let back: BookingId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(id, back);
    }
}
