#!/bin/bash
set -euo pipefail

cargo run --package infra-db-migration -- up
cargo-watch \
  --delay 1 \
  --watch Cargo.toml \
  --watch src \
  --watch crates \
  -x 'run -p energy-tracer --features debug'
