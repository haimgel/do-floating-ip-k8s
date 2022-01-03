FROM rust:1.57.0-alpine3.14 as builder

LABEL org.opencontainers.image.source=https://github.com/haimgel/digital-ocean-floating-ip

# C compiler is needed for Ring, etc.
RUN apk add build-base openssl-dev && \
    adduser -u 1000 app -D && \
    mkdir -p /app /src && \
    chown app /src /app

USER app
COPY --chown=app . /src
WORKDIR /src
RUN --mount=type=cache,target=/usr/local/cargo/registry,uid=1000 \
    --mount=type=cache,target=/src/target,uid=1000 \
    cargo build && \
    cargo build --release && \
    cp /src/target/debug/floating-ip-controller /app && \
    cp /src/target/debug/anchor-ip-annotator /app && \
    cp /src/target/release/floating-ip-controller /app/floating-ip-controller.release && \
    cp /src/target/release/anchor-ip-annotator /app/anchor-ip-annotator.release

FROM alpine:3.14
RUN apk add openssl && \
    adduser -u 1000 app -D && \
    mkdir /app

COPY --from=builder /app/* /app
USER app
ENTRYPOINT ["/app/floating-ip-controller"]