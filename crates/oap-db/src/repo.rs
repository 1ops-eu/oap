//! Repository trait definitions.
//!
//! These traits define the data access interface for OAP resources.
//! PostgreSQL implementations use explicit SQL via SQLx — no ORM magic.
//!
//! Each repository method is designed to work within an explicit transaction
//! when needed (the caller controls transaction boundaries).

use oap_types::actor::{Actor, CreateActorRequest, UpdateActorRequest};
use oap_types::activity::{Activity, CreateActivityRequest, UpdateActivityRequest};
use oap_types::booking::{Booking, CreateBookingRequest};
use oap_types::common::PaginationParams;
use oap_types::ids::*;
use oap_types::location::{CreateLocationRequest, Location};
use oap_types::participant::{CreateParticipantProfileRequest, ParticipantProfile};
use oap_types::provider::{CreateProviderProfileRequest, ProviderProfile, UpdateProviderProfileRequest};
use oap_types::session::{CreateSessionRequest, Session, UpdateSessionRequest};
use oap_types::webhook::{CreateWebhookEndpointRequest, OutboxEvent, WebhookEndpoint};

use oap_domain::errors::OapResult;

// ---------------------------------------------------------------------------
// Actor Repository
// ---------------------------------------------------------------------------

/// Repository interface for Actor operations.
pub trait ActorRepository: Send + Sync {
    fn create(&self, req: CreateActorRequest) -> impl std::future::Future<Output = OapResult<Actor>> + Send;
    fn get_by_id(&self, id: ActorId) -> impl std::future::Future<Output = OapResult<Actor>> + Send;
    fn update(&self, id: ActorId, req: UpdateActorRequest) -> impl std::future::Future<Output = OapResult<Actor>> + Send;
}

// ---------------------------------------------------------------------------
// ProviderProfile Repository
// ---------------------------------------------------------------------------

/// Repository interface for ProviderProfile operations.
pub trait ProviderProfileRepository: Send + Sync {
    fn create(&self, req: CreateProviderProfileRequest) -> impl std::future::Future<Output = OapResult<ProviderProfile>> + Send;
    fn get_by_id(&self, id: ProviderProfileId) -> impl std::future::Future<Output = OapResult<ProviderProfile>> + Send;
    fn update(&self, id: ProviderProfileId, req: UpdateProviderProfileRequest) -> impl std::future::Future<Output = OapResult<ProviderProfile>> + Send;
}

// ---------------------------------------------------------------------------
// ParticipantProfile Repository
// ---------------------------------------------------------------------------

/// Repository interface for ParticipantProfile operations.
pub trait ParticipantProfileRepository: Send + Sync {
    fn create(&self, req: CreateParticipantProfileRequest) -> impl std::future::Future<Output = OapResult<ParticipantProfile>> + Send;
    fn get_by_id(&self, id: ParticipantProfileId) -> impl std::future::Future<Output = OapResult<ParticipantProfile>> + Send;
}

// ---------------------------------------------------------------------------
// Activity Repository
// ---------------------------------------------------------------------------

/// Repository interface for Activity operations.
pub trait ActivityRepository: Send + Sync {
    fn create(&self, req: CreateActivityRequest) -> impl std::future::Future<Output = OapResult<Activity>> + Send;
    fn get_by_id(&self, id: ActivityId) -> impl std::future::Future<Output = OapResult<Activity>> + Send;
    fn update(&self, id: ActivityId, req: UpdateActivityRequest) -> impl std::future::Future<Output = OapResult<Activity>> + Send;
    fn list(&self, params: PaginationParams) -> impl std::future::Future<Output = OapResult<Vec<Activity>>> + Send;
}

// ---------------------------------------------------------------------------
// Session Repository
// ---------------------------------------------------------------------------

/// Repository interface for Session operations.
pub trait SessionRepository: Send + Sync {
    fn create(&self, req: CreateSessionRequest) -> impl std::future::Future<Output = OapResult<Session>> + Send;
    fn get_by_id(&self, id: SessionId) -> impl std::future::Future<Output = OapResult<Session>> + Send;
    fn update(&self, id: SessionId, req: UpdateSessionRequest) -> impl std::future::Future<Output = OapResult<Session>> + Send;
    fn list(&self, params: PaginationParams) -> impl std::future::Future<Output = OapResult<Vec<Session>>> + Send;
    fn list_by_activity(&self, activity_id: ActivityId, params: PaginationParams) -> impl std::future::Future<Output = OapResult<Vec<Session>>> + Send;
}

// ---------------------------------------------------------------------------
// Booking Repository
// ---------------------------------------------------------------------------

/// Repository interface for Booking operations.
pub trait BookingRepository: Send + Sync {
    fn create(&self, req: CreateBookingRequest, initial_status: oap_types::booking::BookingStatus) -> impl std::future::Future<Output = OapResult<Booking>> + Send;
    fn get_by_id(&self, id: BookingId) -> impl std::future::Future<Output = OapResult<Booking>> + Send;
    fn update_status(&self, id: BookingId, status: oap_types::booking::BookingStatus) -> impl std::future::Future<Output = OapResult<Booking>> + Send;
}

// ---------------------------------------------------------------------------
// Location Repository
// ---------------------------------------------------------------------------

/// Repository interface for Location operations.
pub trait LocationRepository: Send + Sync {
    fn create(&self, req: CreateLocationRequest) -> impl std::future::Future<Output = OapResult<Location>> + Send;
    fn get_by_id(&self, id: LocationId) -> impl std::future::Future<Output = OapResult<Location>> + Send;
}

// ---------------------------------------------------------------------------
// WebhookEndpoint Repository
// ---------------------------------------------------------------------------

/// Repository interface for WebhookEndpoint operations.
pub trait WebhookEndpointRepository: Send + Sync {
    fn create(&self, req: CreateWebhookEndpointRequest) -> impl std::future::Future<Output = OapResult<WebhookEndpoint>> + Send;
    fn list_by_owner(&self, owner: ActorId) -> impl std::future::Future<Output = OapResult<Vec<WebhookEndpoint>>> + Send;
    fn list_active(&self) -> impl std::future::Future<Output = OapResult<Vec<WebhookEndpoint>>> + Send;
}

// ---------------------------------------------------------------------------
// Outbox Repository
// ---------------------------------------------------------------------------

/// Repository interface for the event outbox.
pub trait OutboxRepository: Send + Sync {
    fn insert(&self, event: OutboxEvent) -> impl std::future::Future<Output = OapResult<()>> + Send;
    fn fetch_pending(&self, limit: i64) -> impl std::future::Future<Output = OapResult<Vec<OutboxEvent>>> + Send;
    fn mark_delivered(&self, event_id: WebhookEventId) -> impl std::future::Future<Output = OapResult<()>> + Send;
    fn mark_failed(&self, event_id: WebhookEventId, error: String) -> impl std::future::Future<Output = OapResult<()>> + Send;
    fn mark_dead_letter(&self, event_id: WebhookEventId) -> impl std::future::Future<Output = OapResult<()>> + Send;
}
