//! Axum router definition.
//!
//! All v0.1 API routes are registered here.

use axum::{
    Router,
    routing::{get, patch, post},
};
use tower_http::trace::TraceLayer;

use crate::handlers;
use crate::state::AppState;

/// Create the application router with all routes and middleware.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        // Protocol info
        .route("/", get(handlers::health::protocol_info))
        // Actors
        .route("/actors", post(handlers::actors::create_actor))
        .route("/actors/{id}", get(handlers::actors::get_actor))
        .route("/actors/{id}", patch(handlers::actors::update_actor))
        // Provider Profiles
        .route(
            "/provider-profiles",
            post(handlers::provider_profiles::create_provider_profile),
        )
        .route(
            "/provider-profiles/{id}",
            get(handlers::provider_profiles::get_provider_profile),
        )
        .route(
            "/provider-profiles/{id}",
            patch(handlers::provider_profiles::update_provider_profile),
        )
        // Participant Profiles
        .route(
            "/participant-profiles",
            post(handlers::participant_profiles::create_participant_profile),
        )
        .route(
            "/participant-profiles/{id}",
            get(handlers::participant_profiles::get_participant_profile),
        )
        // Activities
        .route("/activities", post(handlers::activities::create_activity))
        .route("/activities", get(handlers::activities::list_activities))
        .route("/activities/{id}", get(handlers::activities::get_activity))
        .route(
            "/activities/{id}",
            patch(handlers::activities::update_activity),
        )
        // Sessions
        .route("/sessions", post(handlers::sessions::create_session))
        .route("/sessions", get(handlers::sessions::list_sessions))
        .route("/sessions/{id}", get(handlers::sessions::get_session))
        .route("/sessions/{id}", patch(handlers::sessions::update_session))
        .route(
            "/sessions/{id}/availability",
            get(handlers::sessions::get_session_availability),
        )
        // Bookings
        .route("/bookings", post(handlers::bookings::create_booking))
        .route("/bookings/{id}", get(handlers::bookings::get_booking))
        .route(
            "/bookings/{id}/cancel",
            post(handlers::bookings::cancel_booking),
        )
        // Webhook Endpoints
        .route(
            "/webhook-endpoints",
            post(handlers::webhook_endpoints::create_webhook_endpoint),
        )
        .route(
            "/webhook-endpoints",
            get(handlers::webhook_endpoints::list_webhook_endpoints),
        )
        // Exports
        .route(
            "/exports/actors/{id}",
            get(handlers::exports::export_actor),
        )
        .route(
            "/exports/provider-profiles/{id}",
            get(handlers::exports::export_provider_profile),
        )
        // Middleware
        .layer(TraceLayer::new_for_http())
        // State
        .with_state(state)
}
