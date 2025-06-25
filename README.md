# 📡 RustPulse

A CLI tool to monitor server resources in real-time — written in idiomatic Rust.

## 📖 Full Case Study

→ [Read it on Notion](https://your.notion.site/project-page)  
→ [Or view the Markdown version](./docs/case-study.md)


## 🔧 Features
- ⏱ Live CPU/memory usage
- 📊 Disk & process stats
- 🐳 Docker-ready
- ⚙️ Configurable thresholds

## 🚀 Usage
```bash
cargo install rustpulse
rustpulse --help
```

## 📸 Demo
![CLI gif]

## 🧠 Architecture
Built with:

tokio

sysinfo

clap

termion

## 🧪 Tests & CI


## 📄 License
MIT OR Apache-2.0

markdown
Copy
Edit

---

# 🚀 RustPulse: Mission-Critical Server Monitoring in Rust

> Originally inspired by aerospace-grade reliability, **RustPulse** is a fast, secure, and production-ready backend system for monitoring server health in real-time. It brings the discipline of fault-tolerant, deterministic systems to modern infrastructure — from cloud servers to critical operations.

---

## 🎯 Project Goal
Create a fully modular, production-grade Rust backend that provides real-time system metrics, secure API access, and a robust DevOps pipeline — all implemented using Hexagonal Architecture and Test-Driven Development.

---

## 🧱 Tech Stack Overview
| Layer              | Tech                                        | Purpose                                 |
|-------------------|---------------------------------------------|----------------------------------------------|
| Web API           | Axum + Tokio                                | Async REST framework                         |
| GraphQL API       | async-graphql                               | Flexible frontend queries                    |
| Database          | PostgreSQL + SQLx                           | Compile-time safe SQL and real-world backend |
| Auth              | JWT + OAuth2                                | Secure login and RBAC                        |
| Observability     | tracing + OpenTelemetry + Prometheus        | Logs and metrics                             |
| DevOps            | GitHub Actions, Docker, Kubernetes          | CI/CD and production deployment              |
| Architecture      | Hexagonal (Ports & Adapters)                | Clean layering and maintainability           |
| Frontend (optional)| Tauri + Plotters                           | Optional graphical UI for metrics            |

---


## 📁 Directory Layout (Hexagonal Architecture)
```
rustpulse/
├── src/
│   ├── main.rs              # Entrypoint
│   ├── config.rs            # Load .env + init settings
│   ├── routes.rs            # Central Axum router
│   ├── telemetry.rs         # Tracing + Prometheus setup
│   ├── middleware.rs        # Logging and auth middleware
│   ├── errors.rs            # Error handling
│   ├── domain/
│   │   ├── model.rs         # Entities: Server, Metric
│   │   ├── port.rs          # Traits: Storage, Auth, Notifier
│   │   └── service.rs       # Business logic (use cases)
│   ├── infra/
│   │   ├── db.rs            # Postgres impl
│   │   ├── auth.rs          # JWT/OAuth impl
│   │   └── notifier.rs      # Alerts/log integrations
├── tests/                   # Integration tests
├── .env                     # Environment variables
├── Dockerfile
├── docker-compose.yml
├── kubernetes/              # Helm charts or raw YAML
├── .github/workflows/       # CI/CD config
└── Cargo.toml
```

---

## 🧱 Week 1 – Project Bootstrapping & Architecture Overview

**🎯 Goal:** Set up the foundational RustPulse project structure with initial routes, logging, and observability baseline.

### ✅ Features Implemented
- Project initialized using `cargo`
- Logging to file with `tracing_appender`
- Optional JSON log format via `.env` (`LOG_JSON=1`)
- `/health` endpoint (liveness probe)
- Root route `/` with startup log message
- `.env` includes `DATABASE_URL`, `RUST_LOG`, and `LOG_JSON`

### 🧰 Tooling
| Tool                | Purpose                      |
|---------------------|------------------------------|
| Axum                | Web framework                |
| Tokio               | Async runtime                |
| tracing             | Structured logging           |
| tracing-subscriber  | Log filtering & formatting   |
| dotenvy             | Load .env config             |
| tower-http          | HTTP middleware              |

### 🌐 Router Design
- `routes.rs` mounts all routes
- Scalable structure for REST, GraphQL, and auth

### 📆 Next: Week 2
- Create domain models: `Server`, `Metric`
- Initialize `AppState`
- Load `.env` into `config.rs`

📌 *This README updates weekly to reflect live project development.*
