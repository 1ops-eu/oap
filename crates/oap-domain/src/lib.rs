//! # oap-domain
//!
//! Domain logic for the Open Activity Protocol.
//!
//! This crate contains state transition validation, booking/capacity logic,
//! and domain error types. It depends on `oap-types` for data structures.

pub mod booking;
pub mod capacity;
pub mod errors;
pub mod transitions;
