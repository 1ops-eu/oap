-- OAP v0.1 Initial Schema
-- PostgreSQL 16+
-- All IDs are UUIDv7 (time-sortable)

-- ============================================================================
-- Custom ENUM types
-- ============================================================================

CREATE TYPE actor_type AS ENUM ('person', 'organization', 'system');
CREATE TYPE verification_status AS ENUM ('unverified', 'pending', 'verified', 'rejected');
CREATE TYPE provider_type AS ENUM ('individual', 'organization');
CREATE TYPE activity_kind AS ENUM ('class', 'private_lesson', 'open_play', 'workshop', 'course_series', 'rehearsal', 'meetup', 'peer_activity', 'custom');
CREATE TYPE activity_domain AS ENUM ('sports', 'music', 'fitness', 'education', 'social', 'custom');
CREATE TYPE visibility AS ENUM ('private', 'invite_only', 'unlisted', 'public');
CREATE TYPE participation_mode AS ENUM ('provider_led', 'peer_led', 'mixed');
CREATE TYPE session_status AS ENUM ('draft', 'scheduled', 'open', 'full', 'in_progress', 'completed', 'cancelled', 'archived');
CREATE TYPE booking_status AS ENUM ('pending', 'requires_payment', 'reserved', 'confirmed', 'waitlisted', 'cancelled_by_participant', 'cancelled_by_provider', 'expired', 'attended', 'no_show', 'refunded');
CREATE TYPE payment_requirement AS ENUM ('not_required', 'required_before_confirmation', 'required_deferred', 'package_credit');
CREATE TYPE payment_status AS ENUM ('not_required', 'pending', 'authorized', 'captured', 'failed', 'refunded', 'partially_refunded');
CREATE TYPE attendance_status AS ENUM ('attended', 'late', 'no_show', 'excused_absence');
CREATE TYPE location_type AS ENUM ('physical', 'virtual', 'hybrid');
CREATE TYPE policy_type AS ENUM ('cancellation', 'no_show', 'booking_window', 'late_payment', 'waitlist_promotion', 'custom');
CREATE TYPE delivery_status AS ENUM ('pending', 'delivered', 'failed', 'dead_letter');

-- ============================================================================
-- Actors
-- ============================================================================

CREATE TABLE actors (
    actor_id        UUID PRIMARY KEY,
    actor_type      actor_type NOT NULL,
    display_name    VARCHAR(255) NOT NULL,
    handle          VARCHAR(255) UNIQUE,
    identity_ref    JSONB,
    verification_status verification_status NOT NULL DEFAULT 'unverified',
    capabilities    JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata        JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_actors_handle ON actors (handle) WHERE handle IS NOT NULL;
CREATE INDEX idx_actors_type ON actors (actor_type);
CREATE INDEX idx_actors_created_at ON actors (created_at);

-- ============================================================================
-- Provider Profiles
-- ============================================================================

CREATE TABLE provider_profiles (
    provider_profile_id UUID PRIMARY KEY,
    actor_id            UUID NOT NULL REFERENCES actors(actor_id),
    provider_type       provider_type NOT NULL,
    display_name        VARCHAR(255) NOT NULL,
    slug                VARCHAR(255) UNIQUE,
    contact             JSONB,
    default_currency    VARCHAR(3),
    payment_account_ref JSONB,
    policies_ref        UUID[] NOT NULL DEFAULT '{}',
    capabilities        JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata            JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_provider_profiles_actor ON provider_profiles (actor_id);
CREATE INDEX idx_provider_profiles_slug ON provider_profiles (slug) WHERE slug IS NOT NULL;

-- ============================================================================
-- Participant Profiles
-- ============================================================================

CREATE TABLE participant_profiles (
    participant_profile_id UUID PRIMARY KEY,
    actor_id               UUID NOT NULL REFERENCES actors(actor_id),
    preferences            JSONB NOT NULL DEFAULT '{}'::jsonb,
    capabilities           JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata               JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_participant_profiles_actor ON participant_profiles (actor_id);

-- ============================================================================
-- Activities
-- ============================================================================

CREATE TABLE activities (
    activity_id          UUID PRIMARY KEY,
    owner_actor_id       UUID NOT NULL REFERENCES actors(actor_id),
    provider_profile_id  UUID REFERENCES provider_profiles(provider_profile_id),
    kind                 activity_kind NOT NULL,
    title                VARCHAR(500) NOT NULL,
    description          TEXT,
    domain               activity_domain NOT NULL,
    subcategory          VARCHAR(100),
    visibility           visibility NOT NULL DEFAULT 'public',
    participation_mode   participation_mode NOT NULL DEFAULT 'provider_led',
    default_capacity     INTEGER,
    booking_rules_ref    UUID,
    cancellation_policy_ref UUID,
    pricing_model        JSONB,
    capabilities         JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata             JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_activities_owner ON activities (owner_actor_id);
CREATE INDEX idx_activities_provider ON activities (provider_profile_id) WHERE provider_profile_id IS NOT NULL;
CREATE INDEX idx_activities_domain ON activities (domain);
CREATE INDEX idx_activities_kind ON activities (kind);
CREATE INDEX idx_activities_visibility ON activities (visibility);
CREATE INDEX idx_activities_created_at ON activities (created_at);

-- ============================================================================
-- Locations
-- ============================================================================

CREATE TABLE locations (
    location_id          UUID PRIMARY KEY,
    location_type        location_type NOT NULL,
    name                 VARCHAR(255) NOT NULL,
    address              JSONB,
    geo                  JSONB,
    resource_constraints JSONB,
    metadata             JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================================
-- Sessions
-- ============================================================================

CREATE TABLE sessions (
    session_id           UUID PRIMARY KEY,
    activity_id          UUID NOT NULL REFERENCES activities(activity_id),
    owner_actor_id       UUID NOT NULL REFERENCES actors(actor_id),
    provider_profile_id  UUID REFERENCES provider_profiles(provider_profile_id),
    starts_at            TIMESTAMPTZ NOT NULL,
    ends_at              TIMESTAMPTZ NOT NULL,
    timezone             VARCHAR(50) NOT NULL DEFAULT 'UTC',
    location_ref         UUID REFERENCES locations(location_id),
    capacity             INTEGER,
    booked_count         INTEGER NOT NULL DEFAULT 0,
    reserved_count       INTEGER NOT NULL DEFAULT 0,
    waitlist_enabled     BOOLEAN NOT NULL DEFAULT false,
    status               session_status NOT NULL DEFAULT 'draft',
    visibility           visibility NOT NULL DEFAULT 'public',
    price_override       JSONB,
    conversation_ref     JSONB,
    metadata             JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT check_session_times CHECK (ends_at > starts_at),
    CONSTRAINT check_capacity_positive CHECK (capacity IS NULL OR capacity > 0),
    CONSTRAINT check_booked_count CHECK (booked_count >= 0),
    CONSTRAINT check_reserved_count CHECK (reserved_count >= 0)
);

CREATE INDEX idx_sessions_activity ON sessions (activity_id);
CREATE INDEX idx_sessions_owner ON sessions (owner_actor_id);
CREATE INDEX idx_sessions_status ON sessions (status);
CREATE INDEX idx_sessions_starts_at ON sessions (starts_at);
CREATE INDEX idx_sessions_provider ON sessions (provider_profile_id) WHERE provider_profile_id IS NOT NULL;

-- ============================================================================
-- Bookings
-- ============================================================================

CREATE TABLE bookings (
    booking_id             UUID PRIMARY KEY,
    session_id             UUID NOT NULL REFERENCES sessions(session_id),
    activity_id            UUID NOT NULL REFERENCES activities(activity_id),
    participant_actor_id   UUID NOT NULL REFERENCES actors(actor_id),
    participant_profile_id UUID REFERENCES participant_profiles(participant_profile_id),
    created_by_actor_id    UUID NOT NULL REFERENCES actors(actor_id),
    status                 booking_status NOT NULL DEFAULT 'pending',
    seat_count             INTEGER NOT NULL DEFAULT 1,
    reservation_expires_at TIMESTAMPTZ,
    payment_requirement    payment_requirement NOT NULL DEFAULT 'not_required',
    payment_status         payment_status NOT NULL DEFAULT 'not_required',
    payment_intent_ref     JSONB,
    package_consumption_ref VARCHAR(255),
    confirmed_at           TIMESTAMPTZ,
    cancelled_at           TIMESTAMPTZ,
    cancellation_reason    TEXT,
    metadata               JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT check_seat_count CHECK (seat_count > 0)
);

CREATE INDEX idx_bookings_session ON bookings (session_id);
CREATE INDEX idx_bookings_activity ON bookings (activity_id);
CREATE INDEX idx_bookings_participant ON bookings (participant_actor_id);
CREATE INDEX idx_bookings_status ON bookings (status);
CREATE INDEX idx_bookings_reservation_expiry ON bookings (reservation_expires_at)
    WHERE reservation_expires_at IS NOT NULL AND status IN ('reserved', 'requires_payment');
CREATE INDEX idx_bookings_created_at ON bookings (created_at);

-- ============================================================================
-- Attendance (v0.2 resource, table created early for forward compatibility)
-- ============================================================================

CREATE TABLE attendance (
    attendance_id         UUID PRIMARY KEY,
    booking_id            UUID NOT NULL REFERENCES bookings(booking_id),
    session_id            UUID NOT NULL REFERENCES sessions(session_id),
    participant_actor_id  UUID NOT NULL REFERENCES actors(actor_id),
    status                attendance_status NOT NULL,
    recorded_by_actor_id  UUID NOT NULL REFERENCES actors(actor_id),
    recorded_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    metadata              JSONB NOT NULL DEFAULT '{}'::jsonb,

    CONSTRAINT uq_attendance_booking UNIQUE (booking_id)
);

CREATE INDEX idx_attendance_session ON attendance (session_id);
CREATE INDEX idx_attendance_participant ON attendance (participant_actor_id);

-- ============================================================================
-- Policies
-- ============================================================================

CREATE TABLE policies (
    policy_id            UUID PRIMARY KEY,
    policy_type          policy_type NOT NULL,
    owner_actor_id       UUID NOT NULL REFERENCES actors(actor_id),
    provider_profile_id  UUID REFERENCES provider_profiles(provider_profile_id),
    rules                JSONB NOT NULL,
    version              VARCHAR(50) NOT NULL DEFAULT '1.0',
    metadata             JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_policies_owner ON policies (owner_actor_id);

-- ============================================================================
-- Webhook Endpoints
-- ============================================================================

CREATE TABLE webhook_endpoints (
    webhook_endpoint_id UUID PRIMARY KEY,
    owner_actor_id      UUID NOT NULL REFERENCES actors(actor_id),
    url                 TEXT NOT NULL,
    subscribed_events   JSONB NOT NULL DEFAULT '[]'::jsonb,
    active              BOOLEAN NOT NULL DEFAULT true,
    secret              TEXT NOT NULL,
    metadata            JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_webhook_endpoints_owner ON webhook_endpoints (owner_actor_id);
CREATE INDEX idx_webhook_endpoints_active ON webhook_endpoints (active) WHERE active = true;

-- ============================================================================
-- Event Outbox (for reliable webhook delivery)
-- ============================================================================

CREATE TABLE event_outbox (
    event_id          UUID PRIMARY KEY,
    event_type        VARCHAR(100) NOT NULL,
    payload           JSONB NOT NULL,
    delivery_status   delivery_status NOT NULL DEFAULT 'pending',
    attempt_count     INTEGER NOT NULL DEFAULT 0,
    max_attempts      INTEGER NOT NULL DEFAULT 5,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_attempted_at TIMESTAMPTZ,
    delivered_at      TIMESTAMPTZ,
    last_error        TEXT
);

CREATE INDEX idx_outbox_pending ON event_outbox (created_at)
    WHERE delivery_status = 'pending';
CREATE INDEX idx_outbox_failed ON event_outbox (delivery_status)
    WHERE delivery_status = 'failed';

-- ============================================================================
-- Idempotency Keys
-- ============================================================================

CREATE TABLE idempotency_keys (
    idempotency_key VARCHAR(255) NOT NULL,
    owner_actor_id  UUID NOT NULL,
    endpoint        VARCHAR(100) NOT NULL,
    response_status INTEGER NOT NULL,
    response_body   JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at      TIMESTAMPTZ NOT NULL DEFAULT (now() + interval '24 hours'),

    PRIMARY KEY (idempotency_key, owner_actor_id, endpoint)
);

CREATE INDEX idx_idempotency_expires ON idempotency_keys (expires_at);

-- ============================================================================
-- Updated-at trigger function
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply updated_at triggers to all mutable tables
CREATE TRIGGER trg_actors_updated_at BEFORE UPDATE ON actors FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trg_provider_profiles_updated_at BEFORE UPDATE ON provider_profiles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trg_participant_profiles_updated_at BEFORE UPDATE ON participant_profiles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trg_activities_updated_at BEFORE UPDATE ON activities FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trg_sessions_updated_at BEFORE UPDATE ON sessions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trg_bookings_updated_at BEFORE UPDATE ON bookings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trg_policies_updated_at BEFORE UPDATE ON policies FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trg_webhook_endpoints_updated_at BEFORE UPDATE ON webhook_endpoints FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
