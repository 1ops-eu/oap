//! Common types shared across the OAP domain model.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Metadata is an open key-value map for vendor-specific extensions.
///
/// Keys MUST be namespaced for vendor-specific fields, e.g.:
/// - `com.example.tennis.surface`
/// - `org.club.court_number`
/// - `dev.portal.level_band`
pub type Metadata = HashMap<String, serde_json::Value>;

/// Capabilities advertised by a resource.
///
/// Used to declare optional features supported by a provider, activity, etc.
pub type Capabilities = Vec<String>;

/// Pagination parameters for list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    /// Maximum number of items to return (default: 20, max: 100).
    #[serde(default = "default_limit")]
    pub limit: i64,

    /// Cursor-based offset for pagination.
    #[serde(default)]
    pub cursor: Option<String>,
}

fn default_limit() -> i64 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: default_limit(),
            cursor: None,
        }
    }
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T: ToSchema> {
    pub data: Vec<T>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub total_count: Option<i64>,
}

/// OAP protocol version identifier.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProtocolVersion {
    pub version: String,
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self {
            version: "oap/v0.1".to_string(),
        }
    }
}

/// An external reference to an identity provider.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IdentityRef {
    /// The identity provider identifier (e.g., "oidc", "passkey").
    pub provider: String,
    /// The external subject identifier.
    pub subject: String,
    /// Optional issuer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

/// An external reference to a payment intent.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentIntentRef {
    /// The payment provider identifier (e.g., "stripe", "mollie").
    pub provider: String,
    /// The external payment intent ID.
    pub external_id: String,
}

/// An external reference to a conversation/messaging thread.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConversationRef {
    /// The messaging system identifier (e.g., "matrix", "email").
    pub system: String,
    /// The external reference identifier (e.g., room ID, thread ID).
    pub external_id: String,
}

/// ISO 4217 currency code.
pub type CurrencyCode = String;

/// Price amount in minor units (e.g., cents).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Price {
    /// Amount in minor units (e.g., 1000 = €10.00).
    pub amount: i64,
    /// ISO 4217 currency code (e.g., "EUR", "USD").
    pub currency: CurrencyCode,
}

/// Contact information for a provider.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContactInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
