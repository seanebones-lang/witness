# Witness container image: build the API server from the locked workspace,
# run it as an unprivileged user with the database on a dedicated volume.
# See docs/CONTAINERS.md for the deployment recipe and its boundaries.
FROM rust:1.95.0-slim-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY . .
RUN cargo build --release --locked --package witness-api

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --uid 10001 --create-home witness \
    && mkdir -p /data && chown witness:witness /data
COPY --from=builder /build/target/release/witness-api /usr/local/bin/witness-api
USER witness
# HOST is the bind address *inside* the container's network namespace.
# The compose recipe publishes the port on the host's loopback only, so the
# API stays loopback-reachable from the host and unreachable from the LAN.
# Run bare (no compose), keep the default HOST=127.0.0.1 instead.
ENV HOST=0.0.0.0 PORT=8080 DATABASE_URL=sqlite:///data/witness.db
EXPOSE 8080
VOLUME ["/data"]
ENTRYPOINT ["witness-api"]
