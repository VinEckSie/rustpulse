# ⚡ RustPulse — Real-Time Telemetry Engine in Rust

[![CI](https://github.com/vinecksie/rustpulse/actions/workflows/ci.yml/badge.svg)](https://github.com/vinecksie/rustpulse/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Last Commit](https://img.shields.io/github/last-commit/vinecksie/rustpulse)](https://github.com/vinecksie/rustpulse)

RustPulse is an **async telemetry engine** exploring how to build reliable data pipelines and well-structured systems in Rust.

Designed as a **case study in systems design**, observability, and production-oriented engineering practices.

📘 Case study → https://vinecksie.super.site/rustpulse


# Overview

RustPulse ingests and processes telemetry from distributed nodes using a modular architecture that isolates domain logic from infrastructure concerns.

Focus areas:

• async processing with Axum + Tonic (gRPC)  
• PostgreSQL persistence with JSONL fallback  
• structured tracing and observability  
• clean hexagonal architecture  
• production-style CI validation  


# Key Capabilities

• REST + gRPC ingestion endpoints  
• PostgreSQL storage via SQLx with idempotent schema init  
• JSONL fallback for offline-first scenarios  
• structured logging with tracing  
• OpenTelemetry tracing with Jaeger (local)  
• CRC32 request validation for data integrity  
• configurable environment-based runtime behaviour  
• CI pipeline with clippy, tests, coverage, cargo-deny  


# Architecture

RustPulse follows a **hexagonal architecture** separating:

domain logic • application services • infrastructure adapters

Core domains:

• Node identity lifecycle  
• telemetry ingestion pipeline  
• validation and transformation flow  

Design principles:

DDD-inspired boundaries  
TDD-oriented workflow  
composable ports and adapters  

# Tech Stack

Rust · Axum · Tonic (gRPC) · SQLx · PostgreSQL · JSONL  
Tracing · OpenTelemetry · Jaeger  
Docker · GitHub Actions · cargo-deny  


# Recent Improvements

• PostgreSQL wiring with environment-driven configuration  
• SQLx setup documentation and schema initialization  
• CRC32 validation for /telemetry endpoint  
• OpenTelemetry tracing with Jaeger spans  
• improved CI pipeline with cargo-deny  
• concurrency configuration in CI workflow  


# Documentation

[Observability (OpenTelemetry + Jaeger)](https://github.com/VinEckSie/rustpulse/blob/main/docs/observability.md)  
Local distributed tracing configuration and span instrumentation.

[CRC32 validation](https://github.com/VinEckSie/rustpulse/blob/main/docs/crc32.md)  
Integrity validation for `/telemetry` ingestion requests.

[Persistence](https://github.com/VinEckSie/rustpulse/blob/main/docs/persistence.md)  
PostgreSQL wiring, JSONL fallback strategy, and storage decisions.

[Deployment runbook](https://github.com/VinEckSie/rustpulse/blob/main/docs/deployment_runbook.md)  
Staging and production setup using Docker Compose and systemd.

# Purpose

RustPulse explores how to design maintainable Rust services with emphasis on:

• clear architecture boundaries  
• async and concurrent data pipelines  
• observability and runtime introspection  
• reliable persistence strategies  
• production-oriented engineering practices  

It serves as a practical reference for system-oriented Rust development.

# License

MIT OR Apache-2.0
