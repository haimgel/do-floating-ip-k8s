FROM rust:1.57.0-alpine3.14 as builder

LABEL org.opencontainers.image.source=https://github.com/haimgel/digital-ocean-floating-ip

COPY . /src
WORKDIR /src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release

FROM alpine:3.14
RUN mkdir /app
COPY --from=builder /src/target/release/floating-ip-controller /app
ENTRYPOINT ["/app/floating-ip-controller"]
