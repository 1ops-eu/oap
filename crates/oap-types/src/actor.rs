//! Actor — the protocol-level identity anchor.
//!
//! An Actor represents a person or organization within OAP. It is intentionally
//! broader than "user" to support providers, participants, organizers, coaches,
//! staff, peer-created activities, and organization accounts.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::{Capabilities, IdentityRef, Metadata};
use crate::ids::ActorId;

/// The type of actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    /// An individual person.
    Person,
    /// An organization (club, studio, school, etc.).
    Organization,
    /// A system/service actor.
    System,
}

/// Verification status of an actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Unverified,
    Pending,
    Verified,
    Rejected,
}

/// An Actor in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct Actor {
    pub actor_id: ActorId,
    pub actor_type: ActorType,

    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    /// Optional human-readable handle (e.g., username-like slug).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,

    /// Optional external identity binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ref: Option<IdentityRef>,

    pub verification_status: VerificationStatus,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Capabilities,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// Request body for creating an Actor.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateActorRequest {
    pub actor_type: ActorType,

    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ref: Option<IdentityRef>,

    #[serde(default)]
    pub metadata: Metadata,
}

/// Request body for updating an Actor.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateActorRequest {
    #[validate(length(min = 1, max = 255))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ref: Option<IdentityRef>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
