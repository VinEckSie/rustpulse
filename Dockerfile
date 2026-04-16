# syntax=docker/dockerfile:1

FROM rust:1.86-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --bin rustpulse --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rustpulse /usr/local/bin/rustpulse

EXPOSE 8080

ENV APP_ENV=prod
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUSTPULSE_STORAGE=jsonl

CMD ["rustpulse"]
