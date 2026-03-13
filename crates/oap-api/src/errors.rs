//! HTTP error types following RFC 9457 (Problem Details for HTTP APIs).

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use oap_domain::errors::OapError;
use serde::Serialize;
use utoipa::ToSchema;

/// RFC 9457 Problem Details response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ProblemDetail {
    /// A URI reference that identifies the problem type.
    #[serde(rename = "type")]
    pub problem_type: String,
    /// A short, human-readable summary.
    pub title: String,
    /// The HTTP status code.
    pub status: u16,
    /// A human-readable explanation specific to this occurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A URI reference to the specific occurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ProblemDetail {
    pub fn new(status: StatusCode, title: impl Into<String>) -> Self {
        Self {
            problem_type: "about:blank".to_string(),
            title: title.into(),
            status: status.as_u16(),
            detail: None,
            instance: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// Application-level error type that converts to HTTP responses.
#[derive(Debug)]
pub struct ApiError(OapError);

impl From<OapError> for ApiError {
    fn from(err: OapError) -> Self {
        Self(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, problem) = match &self.0 {
            OapError::NotFound { resource, id } => (
                StatusCode::NOT_FOUND,
                ProblemDetail::new(StatusCode::NOT_FOUND, format!("{resource} not found"))
                    .with_detail(format!("{resource} with id {id} was not found")),
            ),
            OapError::InvalidTransition {
                resource,
                from,
                to,
            } => (
                StatusCode::CONFLICT,
                ProblemDetail::new(StatusCode::CONFLICT, "Invalid state transition")
                    .with_detail(format!(
                        "{resource} cannot transition from '{from}' to '{to}'"
                    )),
            ),
            OapError::CapacityExceeded { .. } => (
                StatusCode::CONFLICT,
                ProblemDetail::new(StatusCode::CONFLICT, "Capacity exceeded")
                    .with_detail(self.0.to_string()),
            ),
            OapError::ReservationExpired { .. } => (
                StatusCode::GONE,
                ProblemDetail::new(StatusCode::GONE, "Reservation expired")
                    .with_detail(self.0.to_string()),
            ),
            OapError::DuplicateRequest { key } => (
                StatusCode::CONFLICT,
                ProblemDetail::new(StatusCode::CONFLICT, "Duplicate request")
                    .with_detail(format!("Request with idempotency key '{key}' already processed")),
            ),
            OapError::Validation { message } => (
                StatusCode::BAD_REQUEST,
                ProblemDetail::new(StatusCode::BAD_REQUEST, "Validation error")
                    .with_detail(message.clone()),
            ),
            OapError::Conflict { message } => (
                StatusCode::CONFLICT,
                ProblemDetail::new(StatusCode::CONFLICT, "Conflict").with_detail(message.clone()),
            ),
            OapError::Unauthorized { message } => (
                StatusCode::UNAUTHORIZED,
                ProblemDetail::new(StatusCode::UNAUTHORIZED, "Unauthorized")
                    .with_detail(message.clone()),
            ),
            OapError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ProblemDetail::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                ),
            ),
        };

        (status, Json(problem)).into_response()
    }
}

/// Result type alias for API handlers.
pub type ApiResult<T> = Result<T, ApiError>;
