FROM rust:latest@sha256:ac8c4cb82e317512260fbcf54e80039d9083605e3b8ea3b9fd4c39e1472c6215 AS builder
WORKDIR /arcalog
COPY . .
RUN cargo install --path .

FROM quay.io/centos/centos:stream9@sha256:a4f9d5e8da914b40c8153b7e927c1de83143a9f2e280b41fb3553522aa8c3ecb
RUN dnf install -y compat-openssl11
WORKDIR /
VOLUME ["/data"]
COPY --from=builder /arcalog/target/release/arcalog /arcalog
COPY --from=builder /arcalog/config.yaml /config.yaml
ENTRYPOINT ["/arcalog"]
CMD []

LABEL org.opencontainers.image.source="https://github.com/arcalot/arcalog"
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL org.opencontainers.image.vendor="Arcalot project"
LABEL org.opencontainers.image.authors="Arcalot contributors"
LABEL org.opencontainers.image.title="Arcalog"