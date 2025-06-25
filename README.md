# 🚀 RustPulse — Real-Time Monitoring for Mission-Critical Systems 
[![CI](https://github.com/yourusername/rustpulse/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/rustpulse/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Last Commit](https://img.shields.io/github/last-commit/yourusername/rustpulse)](https://github.com/yourusername/rustpulse)


*A Rust-native system for local, secure, and high-performance telemetry monitoring — built with Hexagonal Architecture, DDD, and TDD.*
> Fast, modular, and secure real-time telemetry system written in Rust for mission-critical environments.


---

## ❓ Why RustPulse?

RustPulse was built to showcase what modern Rust can offer in terms of:

- Deterministic performance
- Fully native backend + frontend
- True test-first, modular architecture (Hexagonal + DDD)
- Aerospace-inspired system reliability, minus the overhead

---

## 📖 Full Case Study

→ [Read it on Notion](https://your.notion.site/project-page)  
→ [Or view the Markdown version](./docs/case-study.md)

---

## 📡 Overview

RustPulse is a modular, production-ready telemetry monitoring system written entirely in Rust. It is built for high-reliability environments such as simulation clusters, internal infrastructure, and latency-sensitive operations.

✅ Hexagonal Architecture (Ports & Adapters)  
✅ Domain-Driven Design + Test-Driven Development  
✅ Fast REST/gRPC APIs with PostgreSQL persistence  
✅ Tauri desktop dashboard with real-time visualizations  
✅ Dockerized, secure, and CI-integrated

---

## 🌐 Live Demo

🚧 Coming soon – Will be available at:  
👉 [https://demo.rustpulse.io](https://demo.rustpulse.io)

---

## 🧱 Tech Stack

| Component                | Technology (Rust-Only)           | Purpose                                         |
|--------------------------|----------------------------------|-------------------------------------------------|
| Backend Framework        | Actix Web                        | High-performance REST API framework             |
| API Communication        | Tonic (gRPC)                     | Efficient, low-latency data exchange            |
| Database                 | PostgreSQL + Diesel ORM          | Storing server performance logs & history       |
| Async Runtime            | Tokio                            | Handling real-time data efficiently             |
| Telemetry & Logging      | Tracing                          | Capturing API logs & debugging information      |
| Security & Authentication| JWT (jsonwebtoken crate)         | Secure login & user session management          |
| Frontend UI              | Tauri                            | Interactive native desktop dashboard            |
| Data Visualization       | Plotters                         | Real-time graphs & analytics                    |
| Error Handling           | thiserror + anyhow               | Reliable & structured error management          |
| Testing & CI/CD          | Cargo test, Clippy, GitHub Actions| Unit testing, static analysis, CI/CD pipeline   |
| Deployment & Hosting     | Docker + DigitalOcean            | Containerized deployment for easy setup         |
| Zero-Downtime Deployment | Rolling Updates (Docker)         | Seamless system updates with no downtime        |


---

## 🔧 Key Features

### Backend API
- ✅ Real-time telemetry collection (CPU, Memory, Disk, Network)
- ✅ REST & gRPC API for data access
- ✅ Custom alert thresholds and notifications
- ✅ Historical metrics storage

---

### Desktop Dashboard
- ✅ Native Tauri-based UI (no browser needed)
- ✅ Live charts with Plotters
- ✅ Dark/light mode switch
- ✅ Local-only, secure access

---

## 🧪 Example Use Case

> A simulation team runs CPU-heavy processes. With RustPulse:  
> – Engineers track live system metrics through the dashboard  
> – Alerts trigger when thresholds are exceeded  
> – Teams respond immediately to prevent outages  
> – Logs and trends support diagnostics

---

## 🚀 How to Run

### 🐳 Docker (Recommended)
```bash
git clone https://github.com/yourname/rustpulse.git
cd rustpulse
docker-compose up --build
```

### 🦀 Local (Dev)
```bash
cargo build
cargo run
```

---

### 🧪 Testing & CI
RustPulse is built using TDD principles:

Unit and integration tests (cargo test)

Linting with Clippy (cargo clippy)

GitHub Actions for CI/CD

---

## 📁 Project Structure

```text
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

### 📌 Roadmap
 Phase 2: Auth + Role Guards

 Phase 3: Observability + CI/CD

 Phase 4: Tauri UI buildout

 Phase 5: Testing + Public Demo

 ---

### 📸 Demo & Screenshots
🚧 Coming soon after Phase 4

---

## 🤝 Contributing

Contributions are welcome!  
If you're learning Rust, curious about system monitoring, or want to explore Hexagonal Architecture in practice — feel free to fork, open issues, or create pull requests.

> RustPulse follows TDD + clean layering. It’s great for clean collaboration.

---

## 📚 Documentation

```bash
cargo doc --open
```

Full hosted documentation will be available on docs.rs after the first crate release.

---

### ✅ Final Section Order (top to bottom)

| Section | Placement |
|---------|-----------|
| ❓ Why RustPulse | Right before `📖 Full Case Study` |
| 📖 Full Case Study | Already in place |
| 📡 Overview → 📁 Project Structure | As-is |
| 📌 Roadmap | Keep here |
| 📸 Demo & Screenshots | Keep here |
| 📚 Documentation | Add before License |
| 🤝 Contributing | Add before Documentation |
| 📄 License | Final section (always last) |

---

Let me know when you publish — this `README.md` is looking **world-class**. 🦀🔥
---

### 📄 License
MIT OR Apache-2.0
