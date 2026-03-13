# OAP Protocol Overview

## What is OAP?

The **Open Activity Protocol (OAP)** is an open protocol for interoperable activity coordination. It defines a canonical domain model, state machines, API semantics, and event contracts for managing activities, sessions, bookings, and attendance across diverse domains.

## Protocol Version

Current: `oap/v0.1`

Media type: `application/vnd.oap+json;version=0.1`

## Core Concepts

### Actor

An **Actor** is the protocol-level identity anchor. It represents a person, organization, or system entity. Actors can optionally have:

- **ProviderProfile** — enables the actor to offer activities (coach, studio, school)
- **ParticipantProfile** — represents participation in activities

### Activity

An **Activity** is the reusable, top-level domain object. It can be:

- A class, lesson, workshop, or course series
- An open play session or meetup
- A peer-created activity between friends
- A rehearsal, camp, or custom activity type

Activities have a `kind`, `domain`, `subcategory`, and support both provider-led and peer-led participation modes.

### Session

A **Session** is the concrete, scheduled instance of an Activity — the primary bookable unit. Sessions have:

- Time range (`starts_at`, `ends_at`, `timezone`)
- Location reference
- Capacity limits with booking/reservation tracking
- A well-defined status state machine

### Booking

A **Booking** represents a participant's reservation for a Session. It is explicitly separate from:

- Payment (tracked as `payment_status` + external refs)
- Attendance (separate resource in v0.2+)
- Identity provider details

Bookings follow a detailed state machine supporting pending, reserved, confirmed, waitlisted, cancelled, and terminal states.

## State Machines

OAP uses explicit, stable state machines — they are non-negotiable for interoperability.

### Session Lifecycle

```
draft → scheduled → open → full → in_progress → completed → archived
```

### Booking Lifecycle

```
pending → requires_payment → reserved → confirmed → attended/no_show
     → waitlisted → (promoted) → confirmed
     → expired / cancelled
```

## API Style

- REST-first, JSON over HTTPS
- OpenAPI 3.1 defined
- Machine-readable, SDK-generatable
- Idempotent via `Idempotency-Key` header
- RFC 9457 Problem Details for errors
- HMAC-SHA256 signed webhooks

## Getting Started

See the [README](../README.md) for setup instructions and the [OpenAPI spec](../spec/openapi.yaml) for the full API definition.
