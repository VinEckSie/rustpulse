# justfile

# Auto-fix clippy lints (when possible)
fix:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged
    cargo fmt
    cargo clippy -- -D warnings

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
    just fix
    just lint
    just test
    just audit
    just coverage
    just machete
    just doc



# Local dev (Jaeger + backend)
dev:
    @echo "Otel + Jaeger: \n- Start Docker Desktop \n- Run 'just jaeger' in one terminal \n- Run 'just backend' in another \n- Request an endpoint from Rustpulse \n- Check Jaeger UI at localhost:16686/search"

jaeger:
    docker run --rm --name rustpulse-jaeger -p 16686:16686 -p 4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

backend:
    cargo run -p backend --bin rustpulse

jaeger-stop:
    docker stop rustpulse-jaeger || true
