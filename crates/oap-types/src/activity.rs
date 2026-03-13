//! Activity — the reusable top-level domain object.
//!
//! An Activity can represent a provider offering, a peer-created recurring or
//! one-off activity, a club-managed activity, a private lesson template, a
//! public class listing, or a community-hosted concept.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::{Capabilities, Metadata, Price};
use crate::ids::{ActivityId, ActorId, PolicyId, ProviderProfileId};

/// The kind of activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    Class,
    PrivateLesson,
    OpenPlay,
    Workshop,
    CourseSeries,
    Rehearsal,
    Meetup,
    PeerActivity,
    Custom,
}

/// The broad domain of an activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivityDomain {
    Sports,
    Music,
    Fitness,
    Education,
    Social,
    Custom,
}

/// Visibility of an activity or session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    InviteOnly,
    Unlisted,
    Public,
}

/// How participation is structured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ParticipationMode {
    /// Led by a provider (coach, instructor, studio).
    ProviderLed,
    /// Led by peers (friends, community).
    PeerLed,
    /// Mixed mode.
    Mixed,
}

/// Pricing model for an activity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PricingModel {
    /// Free activity, no cost.
    Free,
    /// Fixed price per session.
    PerSession { price: Price },
    /// Price per participant per session.
    PerParticipant { price: Price },
    /// Package/credit-based (reference only in v0.1).
    PackageBased,
    /// Custom pricing determined externally.
    Custom,
}

/// An Activity in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct Activity {
    pub activity_id: ActivityId,
    pub owner_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_profile_id: Option<ProviderProfileId>,

    pub kind: ActivityKind,

    #[validate(length(min = 1, max = 500))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub domain: ActivityDomain,

    /// More specific category (e.g., "tennis", "violin", "yoga").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,

    pub visibility: Visibility,
    pub participation_mode: ParticipationMode,

    /// Default maximum number of participants per session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity: Option<i32>,

    /// Reference to booking rules policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_rules_ref: Option<PolicyId>,

    /// Reference to cancellation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_ref: Option<PolicyId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_model: Option<PricingModel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Capabilities,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// Request body for creating an Activity.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateActivityRequest {
    pub owner_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_profile_id: Option<ProviderProfileId>,

    pub kind: ActivityKind,

    #[validate(length(min = 1, max = 500))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub domain: ActivityDomain,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,

    #[serde(default = "default_visibility")]
    pub visibility: Visibility,

    #[serde(default = "default_participation_mode")]
    pub participation_mode: ParticipationMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_model: Option<PricingModel>,

    #[serde(default)]
    pub metadata: Metadata,
}

fn default_visibility() -> Visibility {
    Visibility::Public
}

fn default_participation_mode() -> ParticipationMode {
    ParticipationMode::ProviderLed
}

/// Request body for updating an Activity.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateActivityRequest {
    #[validate(length(min = 1, max = 500))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_model: Option<PricingModel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
