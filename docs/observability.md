# Observability (OpenTelemetry + Jaeger)

This project uses `tracing` with an OpenTelemetry (OTLP/gRPC) exporter to visualize traces in Jaeger.

## Local tracing (Jaeger)

**Prereqs**
- Docker Desktop running (start it first)
- `just` installed

**0) Start Docker Desktop**

If it’s not already running, start Docker Desktop manually.
Optional (macOS): `open -a Docker`

**1) Configure env**

In repo root `.env`:
- `OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317`
- `OTEL_SERVICE_NAME=rustpulse-backend`
- (optional) `RUSTPULSE_ENV=local`

**2) Start Jaeger**

Terminal A:
- `just jaeger`

Jaeger UI:
- `http://localhost:16686`

**3) Start the backend**

Terminal B:
- `just backend`

Jaeger UI: you should see one boot log like:
- Service: `rustpulse-backend`
- Operation: `startup`

**4) Generate traces**

Open in your browser:
- OK: `http://127.0.0.1:3000/metrics`
- Error (simulates an app/usecase error): `http://127.0.0.1:3000/metrics?source_id=not-a-uuid`

**5) Verify in Jaeger**

Jaeger UI:
- Service: `rustpulse-backend`
- Operation: `GET /metrics`

Expected span shape:
- `fetch telemetry`
- `usecase.telemetry.fetch_all`
    - check value for tags `error.code`, `error.type`, `exception.message`

## Disable tracing

Remove/unset `OTEL_EXPORTER_OTLP_ENDPOINT` and restart the backend.
No exporter init log should appear, and the app should not attempt to send spans.

## Cleanup

- Stop Jaeger: `just jaeger-stop`
