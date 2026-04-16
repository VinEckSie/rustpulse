# RustPulse Deployment Architecture

RustPulse uses the same deployment architecture in staging and production:

- Linux VM
- Docker containers
- Docker Compose orchestration
- systemd manages the Compose stack
- environment-specific behavior controlled via injected environment variables

Local development uses `.env` for convenience.

# Architecture Overview

GitHub Actions
│
▼
GHCR (container registry)
│
▼
Linux VM
├── systemd
├── docker compose
├── postgres container
└── rustpulse container

## Key ideas

- immutable container image per commit
- environment-specific config injected at deploy time
- service managed via systemd
- health endpoint used for deployment validation
