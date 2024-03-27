FROM rust:latest@sha256:00e330d2e2cdada2b75e9517c8359df208b3c880c5e34cb802c120083d50af35 AS builder
WORKDIR /arcalog
COPY . .
RUN cargo install --path .

FROM quay.io/centos/centos:stream9@sha256:7d20007bf2a2f0e23bc2049d8b12a84499701ebf1ddbe819a46c785f8f9b9aca
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