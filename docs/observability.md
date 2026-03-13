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

# Local ingest CRC check (POST /telemetry)

The ingest endpoint supports an optional `x-crc32` header (CRC-32/IEEE) to validate the request body bytes.

**Prereqs**
- Backend running (`just backend`)
- (optional) Jaeger running + OTEL env configured (follow the section above)

**0) (Optional) Inspect the example body**

Repo root has `body.json` which is a valid telemetry ingest payload.

**1) Ingest without CRC header**

Terminal:
- `just telemetry-ingest-no-crc`

Expected:
- `202 Accepted`

**2) Ingest with a valid CRC header**

Terminal:
- `just telemetry-ingest-crc-ok`

Expected:
- `202 Accepted`

**3) Ingest with a mismatched CRC header**

Terminal:
- `just telemetry-ingest-crc-bad`

Expected:
- `400 Bad Request`
- JSON error `code: "crc_mismatch"`

**4) Ingest with an invalid CRC header**

Terminal:
- `just telemetry-ingest-crc-invalid`

Expected:
- `400 Bad Request`
- JSON error `code: "invalid_crc"`

**5) Verify in Jaeger (optional)**

Jaeger UI:
- Service: `rustpulse-backend`
- Operation: `POST /telemetry`

For the mismatch case, the trace should include a log/event field:
- `crc_check = "fail"`

## Disable tracing

Remove/unset `OTEL_EXPORTER_OTLP_ENDPOINT` and restart the backend.
No exporter init log should appear, and the app should not attempt to send spans.

## Cleanup

- Stop Jaeger: `just jaeger-stop`
