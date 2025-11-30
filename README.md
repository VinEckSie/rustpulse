# 🚀 RustPulse — Secure, Real-Time Telemetry Engine
[![CI](https://github.com/vinecksie/rustpulse/actions/workflows/ci.yml/badge.svg)](https://github.com/vinecksie/rustpulse/actions)
[![CI](https://github.com/vinecksie/rustpulse/actions/workflows/audit.yml/badge.svg)](https://github.com/vinecksie/rustpulse/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Last Commit](https://img.shields.io/github/last-commit/vinecksie/rustpulse)](https://github.com/vinecksie/rustpulse)

Real-Time Telemetry Engine in Rust — built for 🛰️ edge devices, self-hosted metrics, and secure, offline-first operations.  
Powered by **Axum**, **gRPC (Tonic)**, and **PostgreSQL**.


## ✨ Overview

**RustPulse** is a modular, secure telemetry engine written in Rust for **real-time monitoring of distributed nodes**.  
It targets **offline-first**, **self-hosted** environments such as simulation clusters, defense systems, or autonomous edge deployments.

**Highlights**
- Hexagonal Architecture (Ports & Adapters)
- Domain-Driven + Test-Driven Development
- REST / gRPC APIs with SQLx & PostgreSQL persistence
- Offline-first design for edge & mission-critical ops
- CLI → Dashboard observability (Prometheus / Grafana)
- JWT-based auth with planned role separation
- MockTelemetrySource for simulation and test isolation


## 🧱 Architecture Overview

RustPulse follows a **Hexagonal Architecture** that isolates the domain layer from infrastructure and interfaces.

**Core Domains**
- `Node` → Agent identity and lifecycle  
- `NodeTelemetry` → Real-time metrics ingestion  
- `TelemetrySource` → Validation and data flow orchestration  

**Design Principles**
- **DDD** — domain-centric entities and use-cases  
- **TDD** — integration tests across async boundaries  
- **Security** — JWT authentication + role-based API/CLI separation  

<p align="center">
  <img src="https://raw.githubusercontent.com/VinEckSie/rustpulse/main/docs/architecture_overview.png" width="640">
</p>


## 🧰 Tech Stack

| Component | Tooling | Purpose |
|------------|----------|----------|
| **Backend Framework** | Axum | Fast async REST API building |
| **Storage** | PostgreSQL + SQLx / JSONL | Scalable telemetry persistence |
| **Transport** | Tonic (gRPC) | Binary protocol for distributed telemetry |
| **CQRS** | Axum + async executors | Clean command/query separation |
| **CLI Tool** | Clap | Native CLI with full telemetry control |
| **Auth** | JWT (jsonwebtoken) | Secure session & access management |
| **Observability** | Prometheus + Grafana | Metrics & visualization |
| **Logging** | Tracing | High-performance structured logs |
| **CI/CD** | GitHub Actions + Clippy + cargo-llvm-cov | Linting, testing, coverage |


## 🧩 Features

- RESTful endpoints via **Axum**
- Modular config with `.env` + **dotenvy**
- Centralized logging with **tracing** / **tracing-subscriber**
- Integration & unit tests using **tokio** + **reqwest**
- Outbound adapters for DB, mocks, and sources
- Async-safe operations with `tokio::sync::Mutex`
- Simplified error handling via **anyhow**
- Trait-based plug-and-play collectors (no core refactor needed)

## 📁 Project Structure

```plaintext
src/
├── adapters/          # Outbound adapters (DBs, mocks, sources)
│   ├── mock_repo.rs
│   ├── postgres_metrics_repo.rs
│   └── telemetry_source_repo.rs
│
├── app/               # Application orchestration
│   ├── errors.rs
│   └── metrics_service.rs
│
├── cli/               # Command-line interface
│   ├── args.rs
│   └── commands.rs
│
├── core/              # Domain logic (entities, ports, use-cases)
│   ├── domains/
│   │   ├── node.rs
│   │   └── telemetry.rs
│   ├── domains.rs
│   └── port.rs
│
├── handlers/          # HTTP handlers (Axum routes)
│   ├── health.rs
│   ├── metrics.rs
│   └── root.rs
│
├── infra/             # Infrastructure (DB, logging, startup)
│   ├── db.rs
│   ├── logging.rs
│   └── startup.rs
│
├── tests/             # Integration tests
│   ├── api.rs
│   └── common.rs
│
├── lib.rs             # Library entry point
└── main.rs            # Binary entry point
```

## 🧭 Planned Enhancements
- Prometheus + Grafana integration for observability
- SQLx-powered PostgreSQL persistence layer
- Structured alerting & configurable thresholds
- JWT-based auth with role-guarded API routes
- Cryptographic handshake (X25519 + HKDF)
- gRPC public API for distributed telemetry
- Containerized DevOps pipeline (Docker + GitHub Actions)
- UI dashboard & CLI client for control and live metrics


## 🧪 Development Notes

This repository is a personal development project.
This project is an educational but production-grade architecture showcase for Rust backend systems.
The goal is to showcase Rust architecture, testing, and systems design practices — not to provide a production-ready tool.


## 📚 Documentation

Documentation will be hosted on docs.rs￼ after the first crate release.
Detailed case studies and weekly changelogs are available on the RustPulse Landing Page￼.


## 📄 License

Dual-licensed under MIT OR Apache-2.0.
You may choose either license.

</br>

Thanks for checking out RustPulse!  
Follow the technical case study for deeper dives into its architecture, testing strategy, and telemetry runtime design in Rust.




