//! Custom Axum extractors.

use axum::{
    extract::Query,
    http::HeaderMap,
};
use oap_types::common::PaginationParams;
use serde::Deserialize;

/// Extract pagination parameters from query string.
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub cursor: Option<String>,
}

fn default_limit() -> i64 {
    20
}

impl From<Pagination> for PaginationParams {
    fn from(p: Pagination) -> Self {
        PaginationParams {
            limit: p.limit.clamp(1, 100),
            cursor: p.cursor,
        }
    }
}

/// Extract the Idempotency-Key header.
pub fn extract_idempotency_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("idempotency-key")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}
