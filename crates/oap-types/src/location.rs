//! Location — where a session occurs.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::Metadata;
use crate::ids::LocationId;

/// The type of location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    Physical,
    Virtual,
    Hybrid,
}

/// Geographic coordinates.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
}

/// Physical address.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Resource constraints for a location (e.g., max occupancy, equipment).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResourceConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_occupancy: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub equipment: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub amenities: Vec<String>,
}

/// A Location in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct Location {
    pub location_id: LocationId,
    pub location_type: LocationType,

    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_constraints: Option<ResourceConstraints>,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// Request body for creating a Location.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateLocationRequest {
    pub location_type: LocationType,

    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_constraints: Option<ResourceConstraints>,

    #[serde(default)]
    pub metadata: Metadata,
}
