//! # oap-types
//!
//! Core domain types, enums, and state machines for the Open Activity Protocol.
//!
//! This crate contains zero business logic — it defines the canonical data model
//! that all other OAP crates depend on.

pub mod actor;
pub mod activity;
pub mod attendance;
pub mod booking;
pub mod common;
pub mod ids;
pub mod location;
pub mod participant;
pub mod policy;
pub mod provider;
pub mod session;
pub mod webhook;
