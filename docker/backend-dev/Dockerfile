FROM ubuntu:26.04
WORKDIR /work

RUN --mount=type=cache,target=/var/lib/apt/lists/ \
    --mount=type=cache,target=/var/cache/apt/ \
    apt-get update \
    && apt-get install -y --no-install-recommends build-essential curl git ca-certificates

ENV MISE_INSTALL_PATH=/usr/local/bin/mise
RUN curl -fsSL https://mise.run | bash
RUN echo 'eval "$(/usr/local/bin/mise activate bash)"' >> /root/.bashrc

COPY mise.toml /etc/mise/config.toml
RUN mise install --system \
    && mise reshim --system

RUN mise exec -- cargo install cargo-watch
COPY --chmod=755 ./backend/watch.sh /work/watch.sh
CMD ["/work/watch.sh"]
