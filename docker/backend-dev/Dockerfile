FROM rust:1.96.1-slim-trixie
WORKDIR /var/tmp/backend

RUN --mount=type=cache,target=/var/lib/apt/lists/ \
    --mount=type=cache,target=/var/cache/apt/ \
    apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates
RUN cargo install cargo-watch

COPY --chmod=755 ./docker/backend-dev/cmd.sh /var/tmp/cmd.sh
CMD ["/var/tmp/cmd.sh"]
