# planner/builder ベースイメージ
FROM rust:1.96.1-trixie AS chef
WORKDIR /work

ENV SCCACHE_DIR=/sccache
ENV RECIPE_FILE=/work/recipe.json

RUN --mount=type=cache,target=/var/lib/apt/lists/ \
    --mount=type=cache,target=/var/cache/apt/ \
    apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        build-essential
RUN cargo install --locked cargo-chef sccache

# ステージ1: Planner - レシピを生成
FROM chef AS planner

COPY ./backend/ /work/backend/
RUN cd backend \
    && cargo chef prepare --recipe-path $RECIPE_FILE

# ステージ2: Builder - 依存関係をキャッシュ
FROM chef AS builder

ENV RUSTC_WRAPPER=sccache

COPY --from=planner $RECIPE_FILE $RECIPE_FILE
COPY ./backend/ /work/backend/
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path $RECIPE_FILE

RUN cd backend \
    && cargo build --locked --release

# ステージ3: Runtime - 最小限の本番イメージ
FROM debian:trixie-slim

COPY --from=builder /work/backend/target/release/energy-tracer /usr/local/bin/energy-tracer
RUN --mount=type=cache,target=/var/lib/apt/lists/ \
    --mount=type=cache,target=/var/cache/apt/ \
    apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates

CMD ["energy-tracer"]
