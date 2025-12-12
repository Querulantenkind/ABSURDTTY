# Requires: just (optional)
default:
  @just -l

fmt:
  cargo fmt

lint:
  cargo clippy --all-targets --all-features -- -D warnings

test:
  cargo test --workspace

build:
  cargo build --release

run-noise:
  cargo run -p noise -- --help

run-mood:
  cargo run -p tty-mood -- --help
