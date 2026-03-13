# AGENTS.md — OAP (Open Activity Protocol)

> **Working name:** **OAP = Open Activity Protocol**
> **Scope:** Open protocol and reference runtime for **activities, sessions, bookings, attendance, and provider/peer scheduling** across sports, music, classes, lessons, and local group activities.
> **Core principle:** **Protocol-first, API-first, federation-later.**
> **Primary implementation language:** **Rust**

---

## 1. Mission

OAP is an open protocol and reference runtime for **interoperable activity coordination**.

It supports both:

1. **Provider-led activities** — tennis coaches, music teachers, yoga studios, sports clubs, local organizers
2. **User-led / peer-to-peer activities** — friends scheduling tennis, pickup games, jam sessions, private practice sessions

The goal is to define a **canonical domain model**, **state machines**, **API semantics**, and **event contracts** so that:

- Providers can run highly customized portals
- Users carry a reusable identity across many providers
- Activity data becomes portable and interoperable
- AI agents can generate or adapt custom frontends on top of stable protocol primitives

---

## 2. Design Principles

**OAP is:**
- A domain protocol for **activities and bookings**
- A stable **resource model** with explicit **state machines**
- A set of **API contracts** and **webhook/event contracts**
- A foundation for **portability** and **custom provider portals**

**OAP is NOT:**
- A social network, messaging, or payment protocol
- A full identity protocol or website builder
- A federated system (day one)

**Core philosophy:**
- Small core, strong semantics
- Protocol over product lock-in
- Canonical data model first
- Payments and identity are adapters, not core
- Federation is optional and later
- AI-friendly, machine-consumable, stable APIs

---

## 3. Core Domain Model

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

Supporting: Location, Policy, Package (v0.2), Membership (v0.2), WebhookEndpoint.

---

## 4. Implementation Language: Rust

### Why Rust

OAP is not a typical CRUD SaaS — it's a **protocol + reference runtime**. Rust provides:

- **Compile-time correctness** for state machines, transitions, and domain invariants
- **Safe concurrency** for booking/capacity race conditions
- **Strong enum modeling** — canonical state machines as algebraic types
- **Crate modularity** — clean separation of protocol types, domain logic, DB, events, and API
- **Future-proofing** — signed artifacts, federation, embeddable engines, CLI tools
- **Protocol credibility** — modern infra projects (Matrix/Conduit, payment systems) converge on Rust

### When Rust is hard

- Slower initial velocity than Go/TypeScript
- Steeper learning curve for AI-generated code
- Risk of over-engineering

**Mitigation:** keep the core small, ship early, iterate. Use TypeScript for frontend portals.

---

## 5. Recommended Technical Stack

### Core Runtime
| Component | Choice |
|-----------|--------|
| Language | Rust stable (1.85+) |
| Web Framework | **Axum** |
| Async Runtime | **Tokio** |
| Database | **PostgreSQL 16+** |
| DB Driver | **SQLx** (compile-time checked, async) |
| Serialization | **Serde** |
| API Spec | **OpenAPI 3.1** (utoipa) |
| Observability | **tracing** + OpenTelemetry |
| IDs | **UUIDv7** |
| Webhook Signing | HMAC-SHA256 |
| Error Model | RFC 9457 Problem Details |

### Why SQLx over Diesel/ORM

OAP is **transaction-heavy, concurrency-sensitive**. ORMs:
- Hide locking behavior
- Obscure transaction boundaries
- Make capacity/reservation logic harder to reason about

SQLx provides explicit SQL with compile-time checking — closest to Go's `sqlc` philosophy.

### Frontend / Portals (Not Core)
- Next.js, TypeScript, React, Tailwind, shadcn/ui, TanStack Query, Zod
- Generated OAP SDKs

### Identity (Composable)
- ZITADEL or Keycloak (self-hostable, pluggable)

### Payments (Adapters)
- Stripe, Mollie, Adyen, PayPal — behind adapter interfaces

---

## 6. Crate Structure

```text
/oap
  /crates
    /oap-types        # Domain types, enums, state machines (zero logic)
    /oap-domain       # Domain logic, state transitions, capacity, validation
    /oap-db           # SQLx queries, migrations, repository traits
    /oap-events       # Webhook/event system, outbox pattern, HMAC signing
    /oap-api          # Axum HTTP API server (reference runtime binary)
```

**Future crates:**
- `oap-export` — import/export bundles
- `oap-signing` — signed manifests, verified exports
- `oap-state-machines` — standalone state machine library

---

## 7. AI Agent Coding Rules (Rust-Specific)

### Architecture
- Treat `Activity`, `Session`, `Booking` as the protocol core
- Keep booking state, payment state, attendance state, and identity state **separated**
- Use additive schema evolution — avoid removing fields
- Do NOT introduce federation into v0.1

### Rust Style
- Model state machines as **Rust enums** with explicit `transition_to()` methods
- Use `thiserror` for domain errors, `anyhow` only at boundaries
- Keep transactions **visible** — never hide them inside repository methods
- Prefer small modules with explicit `pub` boundaries
- Use typed ID wrappers (newtypes over UUID) for compile-time safety
- Write explicit SQL — no ORM magic
- Use the **outbox pattern** for webhook/event delivery
- Design all APIs for **idempotent retries**

### Anti-Patterns
- ❌ Embedding chat/messaging into core objects
- ❌ Overloading `metadata` with business-critical logic
- ❌ Inventing vendor-specific states in canonical enums
- ❌ Hiding concurrency logic in abstractions
- ❌ Creating giant "Event" objects that collapse multiple concerns
- ❌ Coupling provider-led and peer-led flows into one brittle path
- ❌ Assuming marketplace discovery is core

---

## 8. State Machines (Critical, Non-Negotiable)

State machines must be **explicit Rust enums** with validated transitions.

### Session Status
```
draft → scheduled → open → full → in_progress → completed → archived
                      ↓                                        ↑
                      → in_progress → completed ───────────────┘
Any state → cancelled (subject to business rules)
```

### Booking Status
```
pending → requires_payment → reserved → confirmed → attended
    ↓           ↓                          ↓         → no_show
    → waitlisted                           → cancelled_by_participant
    → expired                              → cancelled_by_provider
                                           → refunded
```

### Payment Status
```
not_required | pending | authorized | captured | failed | refunded | partially_refunded
```

### Attendance Status
```
attended | late | no_show | excused_absence
```

---

## 9. Core Technical Requirements

1. **Canonical Resource Model** — stable semantics for AI agents, vendors, exports
2. **UUIDv7** — time-sortable, distributed generation
3. **Idempotency** — `Idempotency-Key` header on all mutating endpoints
4. **Capacity & Concurrency** — transactional seat reservation, time-bounded holds, no overselling
5. **Webhooks/Events** — outbox pattern, at-least-once delivery, HMAC-signed, replay support
6. **Versioning** — `oap/v0.1`, versioned media types, additive evolution
7. **Extensibility** — `kind`, `domain`, `subcategory`, `capabilities[]`, `metadata{}` (namespaced)
8. **Portability** — export/import bundles from day one
9. **RBAC** — owner, admin, coach, staff, participant, organizer, guest

---

## 10. v0.1 Scope

**Resources:** Actor, ProviderProfile, ParticipantProfile, Activity, Session, Booking, Location, Policy, WebhookEndpoint

**Features:** CRUD for all resources, capacity-aware booking, optional reservation window, minimal payment state refs, visibility rules, peer + provider sessions, webhook registration + delivery, export baseline

**Exclusions:** Full federation, complex marketplace, recurrence engine, Package/Membership, built-in chat, deep social graph, protocol-native payments

---

## 11. Guiding Decisions

| Decision | Choice |
|----------|--------|
| Top-level entity | Activity (not generic Event) |
| Bookable unit | Session (concrete scheduled instance) |
| Booking scope | Separate from payment |
| Attendance scope | Separate from booking |
| Identity anchor | Actor |
| Provider concept | ProviderProfile (optional role) |
| Language | Rust + Axum + SQLx + PostgreSQL |
| API style | REST + OpenAPI + webhooks |
| Event delivery | Outbox pattern |
| IDs | UUIDv7 |
| Repo structure | Monorepo first |
| Federation | Later |
| Identity/Payments | Adapters |
| Portability | Day one |
