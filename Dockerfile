FROM rust:latest@sha256:911acdfd39276ead0dfb583a833f1db7d787ad0d5333848378d88f19e5fc158c AS builder
WORKDIR /arcalog
COPY . .
RUN cargo install --path .

FROM quay.io/centos/centos:stream9@sha256:2e2a9f4f7171ccda195f3935192f1150155daab4da305f8c273e9fe6134fe7fa
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