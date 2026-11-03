#!/bin/bash
set -euo pipefail

cd $(dirname $0)

export $(grep -v '^#' ./.env | xargs)
cargo run --package infra-db-migration -- up
cargo-watch \
  --delay 1 \
  --watch Cargo.toml \
  --watch src \
  --watch crates \
  -x 'run -p energy-tracer --features debug'
