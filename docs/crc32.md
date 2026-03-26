# CRC-32 (X-CRC32 / `x-crc32`) testing (POST `/telemetry`)

The ingest endpoint supports an optional `x-crc32` header (CRC-32/IEEE) to validate the request body bytes.

**Prereqs**
- Backend running (just backend)
- `just` installed

## Commands (from `justfile`)

Compute CRC-32 for a JSON body file (prints 8 hex digits):
- just crc32
- just crc32 body.json
- just crc32 file=path/to/body.json

Ingest examples:
- No CRC header: just telemetry-ingest-no-crc
- CRC OK (computed from file bytes): just telemetry-ingest-crc-ok
- CRC mismatch: just telemetry-ingest-crc-bad
- CRC invalid (non-hex): just telemetry-ingest-crc-invalid

## Expected responses

- No CRC header: `202 Accepted`
- Valid CRC header: `202 Accepted`
- CRC mismatch: `400 Bad Request` with JSON error `code: "crc_mismatch"`
- Invalid CRC: `400 Bad Request` with JSON error `code: "invalid_crc"`
