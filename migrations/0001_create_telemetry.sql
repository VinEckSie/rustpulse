CREATE TABLE IF NOT EXISTS telemetry (
    source_id UUID NOT NULL,
    server_id UUID NOT NULL,
    "timestamp" TIMESTAMPTZ NOT NULL,
    cpu DOUBLE PRECISION NULL,
    memory DOUBLE PRECISION NULL,
    temperature REAL NULL,
    extras JSONB NOT NULL
);

CREATE INDEX IF NOT EXISTS telemetry_timestamp_idx ON telemetry ("timestamp");
