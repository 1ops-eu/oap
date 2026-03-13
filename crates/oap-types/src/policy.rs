//! Policy — defines business logic references for booking, cancellation, etc.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::Metadata;
use crate::ids::{ActorId, PolicyId, ProviderProfileId};

/// The type of policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PolicyType {
    Cancellation,
    NoShow,
    BookingWindow,
    LatePayment,
    WaitlistPromotion,
    Custom,
}

/// A Policy in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct Policy {
    pub policy_id: PolicyId,
    pub policy_type: PolicyType,
    pub owner_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_profile_id: Option<ProviderProfileId>,

    /// Policy rules as structured JSON.
    pub rules: serde_json::Value,

    /// Policy version identifier.
    pub version: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// Request body for creating a Policy.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePolicyRequest {
    pub policy_type: PolicyType,
    pub owner_actor_id: ActorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_profile_id: Option<ProviderProfileId>,

    pub rules: serde_json::Value,

    #[serde(default = "default_version")]
    pub version: String,

    #[serde(default)]
    pub metadata: Metadata,
}

fn default_version() -> String {
    "1.0".to_string()
}
