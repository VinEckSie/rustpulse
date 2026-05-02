# RustPulse: Real-Time Telemetry Engine in Rust

[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![CI](https://github.com/vinecksie/rustpulse/actions/workflows/ci.yml/badge.svg)](https://github.com/vinecksie/rustpulse/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)

> Production-style Rust telemetry system with real ingestion, CI/CD, and observability.

Telemetry engine designed for **distributed and edge environments**:

- handles intermittent connectivity  
- idempotent ingestion pipeline  
- PostgreSQL + fallback storage (JSONL)  
- structured logging + tracing  
- Dockerized + CI/CD ready  


Case study → https://vinecksie.super.site/rustpulse

---

## 🚀 What this product demonstrates

- Hexagonal architecture (clear separation of concerns)
- Real ingestion via Rust CLI agent (no mock-only flow)
- CI pipeline (fmt, clippy, deny, test, coverage)
- Observability with tracing + OpenTelemetry + Jaeger
- Production-style deployment for Staging and Prod (Docker + env config)

## ▶️ Run locally

```bash
just jaeger      # start observability (Jaeger)
just backend     # start RustPulse
```

Health check:
```bash
curl http://127.0.0.1:3000/health
```

Generate traces:
```bash
curl http://127.0.0.1:3000/metrics
```

Run the telemetry agent:
```bash
cargo run --bin agent
```

Run the Rust agent
```rust
cargo run --bin agent
```
👉 Full setup, environment variables, and observability guide:
[docs/observability.md](docs/observability.md)

## 🏗️ Hexagonal Architecture
<img width="1417" height="626" alt="image" src="https://github.com/user-attachments/assets/849f6021-0aab-462b-bc8a-e202a886015a" />


### Flow
Agent → HTTP API → Application Service → Domain → Repository → Storage

### Principles
- domain-driven design
- composable ports & adapters
- testable application services
- strict separation of concerns

### ⚙️ Operational Guarantees
- Reliability
- idempotent ingestion strategy
- fallback storage (JSONL)
- retry-ready design (extensible)

### Failure handling
- DB unavailable → fallback to file storage
- invalid payload → rejected early
- config errors → fail fast at startup
- async processing model

### Health
- /health endpoint
- readiness checks (extensible)

## 🔍 Observability
- structured logging with tracing
- request-level visibility
- OpenTelemetry integration (Jaeger)
- correlation-ready design (request IDs planned)

## 📦 Deployment
- Docker (multi-stage builds)
- Docker Compose orchestration
- environment-based configuration
- reproducible builds

## 🧪 CI Pipeline
- fmt
- clippy
- deny (dependency audit)
- tests
- coverage

## 🧠 Design Decisions & Trade-offs

**Hexagonal architecture**

Why:
- isolates domain logic
- improves testability
- enables multiple adapters

Trade-off:
- more boilerplate vs long-term maintainability

**PostgreSQL + JSONL fallback**

Why:
- PostgreSQL → strong querying capabilities
- JSONL → resilience in degraded environments

Trade-off:
- dual storage complexity

**REST + gRPC**

Why:
- REST → simplicity & compatibility
- gRPC → performance & internal communication

Trade-off:
- increased maintenance surface

## 🎯 Purpose
RustPulse is a production-oriented engineering case study.

It demonstrates:
- system design in Rust
- real-world backend constraints
- observability and reliability patterns
- clean architecture in practice

## 📄 License
MIT OR Apache-2.0
