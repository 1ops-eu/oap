//! # oap-events
//!
//! Webhook and event delivery system for the Open Activity Protocol.
//!
//! Implements HMAC-SHA256 signed webhook delivery with retry logic,
//! using the outbox pattern for reliable at-least-once delivery.

pub mod delivery;
pub mod signing;
