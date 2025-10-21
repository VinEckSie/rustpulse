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

fmt-check:
    cargo fmt --check || true

# Automatically rerun all checks on file change
watch-dev:
    cargo watch -s "just fmt-check && just test" --ignore coverage --ignore target --ignore docs

watch-ci:
    cargo watch -d 3 -s "just check" --ignore coverage --ignore target --ignore docs
