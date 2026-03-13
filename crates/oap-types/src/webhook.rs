//! Webhook — endpoint registration and event delivery types.
//!
//! Webhooks are core infrastructure in OAP (like Stripe).
//! Events use an outbox pattern for reliable delivery.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::Metadata;
use crate::ids::{ActorId, WebhookEndpointId, WebhookEventId};

/// Event types emitted by the OAP protocol.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub enum OapEventType {
    // Actor events
    #[serde(rename = "actor.created")]
    ActorCreated,
    #[serde(rename = "actor.updated")]
    ActorUpdated,

    // ProviderProfile events
    #[serde(rename = "provider_profile.created")]
    ProviderProfileCreated,
    #[serde(rename = "provider_profile.updated")]
    ProviderProfileUpdated,

    // Activity events
    #[serde(rename = "activity.created")]
    ActivityCreated,
    #[serde(rename = "activity.updated")]
    ActivityUpdated,

    // Session events
    #[serde(rename = "session.created")]
    SessionCreated,
    #[serde(rename = "session.updated")]
    SessionUpdated,
    #[serde(rename = "session.cancelled")]
    SessionCancelled,

    // Booking events
    #[serde(rename = "booking.created")]
    BookingCreated,
    #[serde(rename = "booking.reserved")]
    BookingReserved,
    #[serde(rename = "booking.requires_payment")]
    BookingRequiresPayment,
    #[serde(rename = "booking.confirmed")]
    BookingConfirmed,
    #[serde(rename = "booking.waitlisted")]
    BookingWaitlisted,
    #[serde(rename = "booking.cancelled")]
    BookingCancelled,
    #[serde(rename = "booking.expired")]
    BookingExpired,

    // Attendance events
    #[serde(rename = "attendance.recorded")]
    AttendanceRecorded,
}

impl std::fmt::Display for OapEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| format!("{self:?}"));
        write!(f, "{s}")
    }
}

/// Webhook delivery status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
    DeadLetter,
}

/// A webhook event envelope.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebhookEvent {
    /// Unique event identifier for deduplication.
    pub event_id: WebhookEventId,

    /// The event type (e.g., "booking.confirmed").
    pub event_type: OapEventType,

    /// OAP protocol version.
    pub protocol_version: String,

    /// Event timestamp.
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    /// The event payload (resource snapshot).
    pub data: serde_json::Value,

    /// Correlation metadata.
    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// A registered webhook endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebhookEndpoint {
    pub webhook_endpoint_id: WebhookEndpointId,
    pub owner_actor_id: ActorId,

    /// The HTTPS URL to deliver events to.
    #[validate(url)]
    pub url: String,

    /// Event types this endpoint subscribes to. Empty = all events.
    #[serde(default)]
    pub subscribed_events: Vec<OapEventType>,

    /// Whether this endpoint is active.
    #[serde(default = "default_true")]
    pub active: bool,

    /// Signing secret for HMAC-SHA256 verification.
    #[serde(skip_serializing)]
    pub secret: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

fn default_true() -> bool {
    true
}

/// Request body for creating a WebhookEndpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateWebhookEndpointRequest {
    pub owner_actor_id: ActorId,

    #[validate(url)]
    pub url: String,

    #[serde(default)]
    pub subscribed_events: Vec<OapEventType>,

    #[serde(default)]
    pub metadata: Metadata,
}

/// Outbox event record for reliable delivery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxEvent {
    pub event_id: WebhookEventId,
    pub event_type: OapEventType,
    pub payload: serde_json::Value,
    pub delivery_status: DeliveryStatus,
    pub attempt_count: i32,
    pub max_attempts: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        with = "time::serde::rfc3339::option"
    )]
    pub last_attempted_at: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        with = "time::serde::rfc3339::option"
    )]
    pub delivered_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
}
