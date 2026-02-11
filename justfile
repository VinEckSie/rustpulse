
# RUN ALL CHECKS
check:
    just fix
    just test
    just audit
    just coverage
    just machete
    just doc

# Auto-fix clippy lints (when possible)
fix:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged
    cargo fmt
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



# LOCAL DEV
# Tracing with OpenTelemetry + Jaeger
jaeger:
    docker run --rm --name rustpulse-jaeger -p 16686:16686 -p 4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

backend:
    cargo run -p backend --bin rustpulse

jaeger-stop:
    docker stop rustpulse-jaeger || true

# Add a part for github BP with branches and CI
