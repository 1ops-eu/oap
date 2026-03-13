//! Webhook delivery logic.
//!
//! Delivers webhook events to registered endpoints with:
//! - HMAC-SHA256 signing
//! - Retry logic with exponential backoff
//! - Timeout handling

use std::time::Duration;

use oap_types::webhook::WebhookEvent;
use tracing::{info, warn};

use crate::signing::build_signature_header;

/// Configuration for webhook delivery.
#[derive(Debug, Clone)]
pub struct DeliveryConfig {
    /// HTTP timeout per delivery attempt.
    pub timeout: Duration,
    /// Maximum number of delivery attempts.
    pub max_attempts: u32,
    /// Base delay for exponential backoff (doubles each retry).
    pub backoff_base: Duration,
    /// User-Agent header value.
    pub user_agent: String,
}

impl Default for DeliveryConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            max_attempts: 5,
            backoff_base: Duration::from_secs(1),
            user_agent: "OAP-Webhook/0.1".to_string(),
        }
    }
}

/// Result of a delivery attempt.
#[derive(Debug)]
pub enum DeliveryResult {
    /// Successfully delivered (2xx response).
    Success { status_code: u16 },
    /// Delivery failed after all retry attempts.
    Failed { last_error: String, attempts: u32 },
}

/// Deliver a webhook event to a URL with signing.
///
/// This performs a single delivery attempt (without retries).
/// The caller (outbox worker) is responsible for retry orchestration.
pub async fn deliver_event(
    client: &reqwest::Client,
    url: &str,
    event: &WebhookEvent,
    secret: &[u8],
    config: &DeliveryConfig,
) -> DeliveryResult {
    let payload = match serde_json::to_vec(event) {
        Ok(p) => p,
        Err(e) => {
            return DeliveryResult::Failed {
                last_error: format!("serialization error: {e}"),
                attempts: 1,
            };
        }
    };

    let signature_header = build_signature_header(secret, &payload);

    let result = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", &config.user_agent)
        .header("X-OAP-Signature", &signature_header)
        .header("X-OAP-Event-Type", event.event_type.to_string())
        .header("X-OAP-Event-ID", event.event_id.to_string())
        .header("X-OAP-Protocol-Version", &event.protocol_version)
        .timeout(config.timeout)
        .body(payload)
        .send()
        .await;

    match result {
        Ok(response) => {
            let status = response.status().as_u16();
            if response.status().is_success() {
                info!(
                    event_id = %event.event_id,
                    url = url,
                    status = status,
                    "webhook delivered successfully"
                );
                DeliveryResult::Success {
                    status_code: status,
                }
            } else {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "no body".to_string());
                warn!(
                    event_id = %event.event_id,
                    url = url,
                    status = status,
                    body = %body,
                    "webhook delivery received non-success response"
                );
                DeliveryResult::Failed {
                    last_error: format!("HTTP {status}: {body}"),
                    attempts: 1,
                }
            }
        }
        Err(e) => {
            warn!(
                event_id = %event.event_id,
                url = url,
                error = %e,
                "webhook delivery failed"
            );
            DeliveryResult::Failed {
                last_error: e.to_string(),
                attempts: 1,
            }
        }
    }
}

/// Calculate the backoff delay for a given attempt number.
pub fn backoff_delay(config: &DeliveryConfig, attempt: u32) -> Duration {
    config.backoff_base * 2u32.saturating_pow(attempt.saturating_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backoff_delay() {
        let config = DeliveryConfig {
            backoff_base: Duration::from_secs(1),
            ..Default::default()
        };

        assert_eq!(backoff_delay(&config, 1), Duration::from_secs(1));
        assert_eq!(backoff_delay(&config, 2), Duration::from_secs(2));
        assert_eq!(backoff_delay(&config, 3), Duration::from_secs(4));
        assert_eq!(backoff_delay(&config, 4), Duration::from_secs(8));
    }
}
