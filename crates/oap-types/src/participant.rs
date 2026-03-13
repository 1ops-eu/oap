//! ParticipantProfile — an optional role/profile attached to an Actor.
//!
//! Even if the same human is both a coach and a player, protocol semantics
//! remain cleaner if participation is represented explicitly.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::{Capabilities, Metadata};
use crate::ids::{ActorId, ParticipantProfileId};

/// A ParticipantProfile in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ParticipantProfile {
    pub participant_profile_id: ParticipantProfileId,
    pub actor_id: ActorId,

    /// Optional participant preferences (limited scope).
    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub preferences: Metadata,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Capabilities,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// Request body for creating a ParticipantProfile.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateParticipantProfileRequest {
    pub actor_id: ActorId,

    #[serde(default)]
    pub preferences: Metadata,

    #[serde(default)]
    pub metadata: Metadata,
}
