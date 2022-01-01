FROM debian:11-slim

LABEL org.opencontainers.image.source=https://github.com/haimgel/digital-ocean-floating-ip

RUN mkdir /app
COPY target/release/anchor-ip-annotator /app
ENTRYPOINT ["/app/anchor-ip-annotator"]
