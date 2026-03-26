# Persistence with Postgres via sqlx

## Dev SQL tutorial (Postgres)

Goal: start Postgres locally, boot RustPulse (which initializes the schema), then verify the table/rows using `psql`.

### 1) Start Postgres

- docker compose -f docker-compose.yml up -d postgres
- docker compose -f docker-compose.yml ps

### 2) Enable Postgres storage

Set these environment variables:

RUSTPULSE_STORAGE=postgres
DATABASE_URL=postgres://rustpulse:<password>@127.0.0.1:5432/rustpulse

### 3) Boot the backend (creates/updates schema)

- cargo run --bin rustpulse

Expected boot logs in Postgres mode:
- `db.migrate`
- `db.ready`

### 4) Open a SQL shell (psql)

- docker compose -f docker-compose.yml exec postgres psql -U rustpulse -d rustpulse

### 5) Verify schema + data

In `psql`:
```sql
\dt
\d telemetry
select count(*) from telemetry;
select source_id, server_id, timestamp from telemetry order by timestamp desc limit 5;
```

### 6) Insert data via the app, then re-check

In another terminal:

- just telemetry-ingest-no-crc

Back in `psql`:

```sql
select count(*) from telemetry;
```

### 7) Cleanup (optional)

In `psql`:

```sql
truncate telemetry;
```

Stop Postgres:

- Container and network: docker compose -f docker-compose.yml down
- To wipe data too: docker compose -f docker-compose.yml down -v

## Test commands (fast)

These tests skip the “real DB” cases unless `DATABASE_URL` is set in your environment.

- Pool + ping: cargo test postgres_db
- Repository behavior: cargo test postgres_telemetry_repo
- Boot wiring + schema init: cargo test infra::startup::tests

## INFO: Where the SQL lives

- Boot-time schema init: `src/infra/startup.rs`
- Repo queries (INSERT/SELECT): `src/adapters/output/postgres_telemetry_repo.rs`
