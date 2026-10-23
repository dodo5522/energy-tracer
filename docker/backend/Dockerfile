FROM rust:1.96.1-trixie as builder
WORKDIR /tmp/backend
COPY ./crates ./crates
COPY ./src ./src
COPY ./Cargo.toml ./
COPY ./rust-toolchain.toml ./
RUN cargo build --release

FROM debian:trixie-slim
RUN --mount=type=cache,target=/var/lib/apt/lists/ \
    --mount=type=cache,target=/var/cache/apt/ \
    apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates
COPY --from=builder /tmp/backend/target/release/energy-tracer /usr/local/bin/energy-tracer
CMD ["energy-tracer"]
