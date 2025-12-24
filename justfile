# justfile

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Audit for security vulnerabilities
audit:
    cargo audit

# Code Coverage
coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Check for unused dependencies
machete:
    cargo machete || true

# Build documentation
doc:
    cargo doc --no-deps --open

# Run all checks
check:
    just fmt
    just lint
    just test
    just audit
    just coverage
    just machete
    just doc

# Automatically rerun all checks on file change
watch-dev:
	cargo watch -s "cargo test" --ignore coverage --ignore target --ignore docs

# Local dev (Jaeger + backend)
jaeger:
    docker run --rm --name rustpulse-jaeger -p 16686:16686 -p 4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

jaeger-stop:
    docker stop rustpulse-jaeger || true

backend:
    cargo run -p backend --bin rustpulse

dev:
    @echo "Run 'just jaeger' in one terminal, then 'just backend' in another."
