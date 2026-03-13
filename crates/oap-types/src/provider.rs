//! ProviderProfile — an optional role/profile attached to an Actor.
//!
//! Allows an actor to function as a professional or structured organizer
//! (tennis coach, yoga studio, music school, etc.).

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::{Capabilities, ContactInfo, CurrencyCode, Metadata, PaymentIntentRef};
use crate::ids::{ActorId, PolicyId, ProviderProfileId};

/// The type of provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    /// An individual professional (coach, instructor, teacher).
    Individual,
    /// An organization (studio, school, club, academy).
    Organization,
}

/// A ProviderProfile in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ProviderProfile {
    pub provider_profile_id: ProviderProfileId,
    pub actor_id: ActorId,
    pub provider_type: ProviderType,

    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    /// URL-safe slug for the provider (e.g., "coach-maria-tennis").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactInfo>,

    /// Default currency for pricing (ISO 4217).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<CurrencyCode>,

    /// External payment account reference (Stripe Connect, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_account_ref: Option<PaymentIntentRef>,

    /// Reference to default policies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies_ref: Vec<PolicyId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Capabilities,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// Request body for creating a ProviderProfile.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateProviderProfileRequest {
    pub actor_id: ActorId,
    pub provider_type: ProviderType,

    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<CurrencyCode>,

    #[serde(default)]
    pub metadata: Metadata,
}

/// Request body for updating a ProviderProfile.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateProviderProfileRequest {
    #[validate(length(min = 1, max = 255))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<CurrencyCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
