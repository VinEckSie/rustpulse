# 🚀 RustPulse — Secure, Real-Time Telemetry Engine
[![CI](https://github.com/vinecksie/rustpulse/actions/workflows/ci.yml/badge.svg)](https://github.com/vinecksie/rustpulse/actions)
[![CI](https://github.com/vinecksie/rustpulse/actions/workflows/audit.yml/badge.svg)](https://github.com/vinecksie/rustpulse/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Last Commit](https://img.shields.io/github/last-commit/vinecksie/rustpulse)](https://github.com/vinecksie/rustpulse)


*A Rust-native system for local, secure, and high-performance telemetry monitoring — built with Hexagonal Architecture, DDD, and TDD.*

## 📖 Full Case Study

> 🔍 Read the full technical case study on [Notion](https://vinecksie.notion.site/RustPulse-Secure-Real-Time-Telemetry-Engine-in-Rust-21e066ddb92f8091a561c1f3b710cc0e) or browse the [weekly changelog](https://github.com/VinEckSie/rust-weekly-changelog).


## 📡 Overview

RustPulse is a modular, production-ready telemetry monitoring system written entirely in Rust. It is built for high-reliability environments such as simulation clusters, internal infrastructure, and latency-sensitive operations.

- ✅ Hexagonal Architecture (Ports & Adapters)
- ✅ Domain-Driven Design + Test-Driven Development
- ✅ Fast REST/gRPC APIs with PostgreSQL persistence
- ✅ CLI to Dashboard workflow with Prometheus/Grafana
- ✅ Offline-first design for edge/mission-critical ops
- ✅ Pluggable source architecture (trait-based)
- ✅ Currently uses `MockTelemetrySource` for simulation
- ✅ Real collectors can be added without touching core logic

<!--
## 🌐 Live Demo

🚧 Coming soon – Will be available at:  
👉 [https://demo.rustpulse.io](https://demo.rustpulse.io)
-->

## 🧱 Architecture Overview

- **Hexagonal Architecture**
    - Modular structure separating core logic and external interfaces

- **Domain-Driven Design (DDD)**
    - `Node`: agent identity and lifecycle
    - `NodeTelemetry`: real-time metrics from nodes
    - `TelemetrySource`: ingestion and validation layer

- **Test-Driven Development (TDD)**
    - Integration-driven workflows for async flows & interfaces

- **Security & Auth**
    - JWT for all API layers
    - Role-based access (planned)

## ⚙️ Tech Stack

| Component | Tooling | Purpose |
|----------|---------|---------|
| Backend Framework | Axum (Rust) | Fast and ergonomic async API building |
| Storage | PostgreSQL + SQLx / JSONL | Persistent or append-only telemetry logging |
| Transport | Tonic (gRPC) | Binary protocol for scalable services |
| CQRS | Axum + Async Executors | Clean separation of command/query |
| CLI Tool | Clap + Rust | Native CLI with full telemetry control |
| Auth | JWT (jsonwebtoken) | Secure session management |
| Observability | Prometheus + Grafana | Metrics & dashboards |
| Logging | Tracing | High-performance structured logs |
| CI/CD | GitHub Actions + Clippy | Linting, testing, quality gates |


## 🔧 Features

### ✅ Backend API

- RESTful endpoints via Axum
- Modular `.env` support with `dotenvy`
- Centralized logging via `tracing`
- Mock repo for data simulation
- Fully async architecture

## 📁 Project Structure

```
src/
├── adapters/                 # Outbound adapters (DBs, mocks, sources)
│   ├── mock_repo.rs
│   ├── postgres_metrics_repo.rs
│   └── telemetry_source_repo.rs
│
├── app/                      # Application services and orchestration
│   ├── errors.rs
│   └── metrics_service.rs
│
├── cli/                      # Command-line interface logic
│   ├── args.rs
│   └── commands.rs
│
├── core/                     # Domain logic: entities, ports, use cases
│   ├── domains/
│   │   ├── node.rs
│   │   └── telemetry.rs
│   ├── domains.rs            # Central re-exports (flat module style)
│   └── port.rs               # Domain ports (interfaces for adapters)
│
├── handlers/                 # Inbound interfaces: HTTP handlers (Axum)
│   ├── health.rs
│   ├── metrics.rs
│   └── root.rs
│
├── infra/                    # Infrastructure layer: DB, logging, startup
│   ├── db.rs
│   ├── logging.rs
│   └── startup.rs
│
├── tests/                    # Integration tests
│   ├── api.rs
│   └── common.rs
│
├── lib.rs                    # Library entry point (used for tests or crates)
└── main.rs                   # Binary entry point
```

## 🚀 How to Run

🚧 Coming soon

## 📸 Demo & Screenshots

🚧 Coming soon

## 🤝 Contributing

🚧 Project in early development. No contributions accepted yet. 🚧

## 📚 Documentation

🚧 Processing

Hosted docs will be available on [docs.rs](https://docs.rs/rustpulse) after first crate release.

## 📄 License

MIT OR Apache-2.0


<br>

Thanks for checking out RustPulse! Follow the full case study for deep dives into architecture, design, and async telemetry in Rust.



