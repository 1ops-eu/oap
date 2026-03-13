# OAP — Open Activity Protocol

> An open protocol and reference runtime for **interoperable activity coordination**.

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Protocol Version](https://img.shields.io/badge/protocol-oap%2Fv0.1-green.svg)](#)

---

## What is OAP?

OAP defines a **canonical domain model**, **state machines**, **API semantics**, and **event contracts** for activities, sessions, bookings, attendance, and provider/peer scheduling — across sports, music, classes, lessons, and local group activities.

**OAP supports both:**

- **Provider-led activities** — tennis coaches, yoga studios, music schools, sports clubs
- **Peer-to-peer activities** — friends scheduling tennis, pickup games, jam sessions

**OAP is NOT** a monolithic event platform. It's a protocol layer that enables:

- Providers to run customized portals
- Users to carry a reusable identity across providers
- AI agents to generate custom frontends on stable primitives
- Activity data that is portable and interoperable

## Architecture

```text
┌─────────────────────────────────────────────────────┐
│                   Provider Portals                   │
│              (Next.js / React / Custom)              │
├─────────────────────────────────────────────────────┤
│                    OAP REST API                      │
│                  (Axum + OpenAPI)                     │
├──────────┬──────────┬───────────┬───────────────────┤
│ oap-api  │oap-events│ oap-domain│     oap-db        │
│ (HTTP)   │(webhooks)│  (logic)  │   (PostgreSQL)     │
├──────────┴──────────┴───────────┴───────────────────┤
│                    oap-types                         │
│           (domain model + state machines)            │
└─────────────────────────────────────────────────────┘
```

## Core Domain Model

```text
Actor
  ├─ ProviderProfile (optional)
  └─ ParticipantProfile (optional)

Activity
  └─ Session
       ├─ Booking
       ├─ WaitlistEntry (v0.2)
       └─ Attendance (v0.2)
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (stable, 1.85+) |
| Web Framework | Axum |
| Async Runtime | Tokio |
| Database | PostgreSQL 16+ |
| DB Driver | SQLx (compile-time checked queries) |
| Serialization | Serde |
| API Spec | OpenAPI 3.1 |
| Observability | tracing + OpenTelemetry |
| IDs | UUIDv7 |

## Repository Structure

```text
/oap
  /crates
    /oap-types        # Domain types, enums, state machines
    /oap-domain       # Domain logic, transitions, validation
    /oap-db           # SQLx queries, migrations, repositories
    /oap-events       # Webhook/event system, outbox pattern
    /oap-api          # Axum HTTP API server (reference runtime)
  /spec
    openapi.yaml      # OpenAPI 3.1 protocol spec
  /docs               # Protocol documentation
  /apps               # Provider/consumer portal apps (future)
  /sdk                # Generated SDKs (future)
```

## Quick Start

### Prerequisites

- Rust 1.85+ (`rustup update stable`)
- PostgreSQL 16+
- Docker & Docker Compose (optional, for local dev)

### Local Development

```bash
# Start PostgreSQL
docker compose up -d

# Copy environment config
cp .env.example .env

# Run database migrations
cargo run -p oap-api -- migrate

# Start the API server
cargo run -p oap-api

# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings
```

The API server starts at `http://localhost:8080`.

## Protocol Version

Current: **`oap/v0.1`**

Media type: `application/vnd.oap+json;version=0.1`

## v0.1 Scope

- Actor, ProviderProfile, ParticipantProfile
- Activity, Session, Booking
- Location, Policy (minimal)
- Capacity-aware booking with reservation windows
- State machines for Session, Booking, Payment
- Webhook registration + delivery (outbox pattern)
- Data export baseline
- Idempotency via `Idempotency-Key` header

## Contributing

See [AGENTS.md](AGENTS.md) for architecture, coding guidelines, and protocol rules.

## License

Apache-2.0 — see [LICENSE](LICENSE).
