#WATCH FOR CHANGES
watch:
    cargo watch -c -x "check"

# RUN ALL CHECKS
check:
    just fix
    just deny
    just test
    just coverage
    just machete
    just doc

# Non-destructive local validation (good before pushing)
prepush:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test
    cargo deny check

# Auto-fix clippy lints (when possible)
fix:
    cargo fmt
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged
    cargo fmt
    cargo clippy --all-targets --all-features -- -D warnings

# compliance + supply-chain security checks
deny:
    cargo deny check

# Run tests
test:
    cargo test

# Code Coverage
coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Check for unused dependencies
machete:
    cargo machete || true
# Build documentation
doc:
    cargo doc --no-deps --open

# Scripts
#docker-build:
#    /scripts/docker_build.sh
#deploy-staging:
#    /scripts/deploy_staging.sh
#smoke:
#    /scripts/smoke_test.sh

# LOCAL DEV
# Tracing with OpenTelemetry + Jaeger
jaeger:
    open -a Docker || true
    docker run --rm --name rustpulse-jaeger -p 16686:16686 -p 4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

backend:
    open -g -a Docker || true
    sh -c 'i=0; while [ "$i" -lt 60 ]; do docker info >/dev/null 2>&1 && exit 0; i=$((i+1)); sleep 1; done; echo "Docker not ready" >&2; exit 1'
    docker compose -f compose.yml up -d postgres
    docker compose -f compose.yml ps
    cargo run --bin rustpulse

jaeger-stop:
    docker stop rustpulse-jaeger || true

# DOCKER (macOS)
# Start Docker Desktop: `just docker-start`
docker-start:
    open -a Docker || true

# Postgres via compose.yml (needs POSTGRES_PASSWORD in `.env`)
docker-up:
    docker compose -f compose.yml up -d

postgres-up:
    docker compose -f compose.yml up -d postgres

docker-ps:
    docker compose -f compose.yml ps

docker-logs service="postgres":
    docker compose -f compose.yml logs -f {{service}}

docker-down:
    docker compose -f compose.yml down

docker-reset:
    docker compose -f compose.yml down -v


# LOCAL DEV
# CRC-32 (X-CRC32 header) testing for POST /telemetry
crc32 file="body.json":
    python3 -c 'import zlib; data=open("{{file}}","rb").read(); print("{:08x}".format(zlib.crc32(data) & 0xffffffff))'

telemetry-ingest-no-crc file="body.json":
    curl -sS -i -X POST http://127.0.0.1:3000/telemetry -H "content-type: application/json" --data-binary @"{{file}}"

telemetry-ingest-crc-ok file="body.json":
    curl -sS -i -X POST http://127.0.0.1:3000/telemetry -H 'content-type: application/json' -H "x-crc32: $(just crc32 {{file}})" --data-binary @"{{file}}"

telemetry-ingest-crc-bad file="body.json":
    curl -sS -i -X POST http://127.0.0.1:3000/telemetry -H "content-type: application/json" -H "x-crc32: 00000000" --data-binary @"{{file}}"

telemetry-ingest-crc-invalid file="body.json":
    curl -sS -i -X POST http://127.0.0.1:3000/telemetry -H "content-type: application/json" -H "x-crc32: not-hex" --data-binary @"{{file}}"
