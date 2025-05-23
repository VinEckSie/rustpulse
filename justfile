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

# RUN MANUALLY:
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


#cargo watch -s "just check" --ignore coverage --ignore target --ignore docs





