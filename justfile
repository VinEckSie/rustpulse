
#WATCH FOR CHANGES
watch:
    cargo watch -c -x "check -p backend"

# RUN ALL CHECKS
check:
    just fix
    just deny
    just test
    just coverage
    just machete
    just doc

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


# LOCAL DEV
# Tracing with OpenTelemetry + Jaeger
jaeger:
    docker run --rm --name rustpulse-jaeger -p 16686:16686 -p 4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

backend:
    cargo run -p backend --bin rustpulse

jaeger-stop:
    docker stop rustpulse-jaeger || true

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

# Add a part for github BP with branches and CI
