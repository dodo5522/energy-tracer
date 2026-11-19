FROM base-dev:latest
WORKDIR /work
USER root

ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install cargo-watch
COPY --chmod=755 ./backend/watch.sh /work/backend/watch.sh
CMD ["/work/backend/watch.sh"]
