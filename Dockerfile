FROM rust:1.57.0-slim-bullseye as builder

LABEL org.opencontainers.image.source=https://github.com/haimgel/digital-ocean-floating-ip

# C compiler is needed for Ring, etc.
RUN apt-get update && \
    apt-get install -y build-essential libssl-dev pkg-config && \
    adduser -u 1000 app --disabled-password && \
    mkdir -p /app /src && \
    chown app /src /app

USER app
COPY --chown=app . /src
WORKDIR /src
RUN --mount=type=cache,target=/usr/local/cargo/registry,uid=1000 \
    --mount=type=cache,target=/src/target,uid=1000 \
    cargo build && \
    cp /src/target/debug/floating-ip-controller /app && \
    cp /src/target/debug/anchor-ip-annotator /app

FROM debian:bullseye-slim
RUN \
    adduser -u 1000 app --disabled-password && \
    mkdir /app

COPY --from=builder /app/* /app
USER app
ENTRYPOINT ["/app/floating-ip-controller"]
