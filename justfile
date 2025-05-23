#!/usr/bin/env just --justfile


fmt:
   cargo fmt

lint:
  cargo clippy -- -D warnings

test:
  cargo test

check:
    just fmt
    just lint
    just test

release:
  cargo build --release    

bin:
  cargo run --bin bin -- arg1

#cargo watch -s "just check"





