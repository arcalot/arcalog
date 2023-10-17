FROM rust:latest@sha256:73af736ea21c14181c257bf674c7095a8bad6343a1eadd327a8bf1ce1c5209b4 AS builder
WORKDIR /arcalog
COPY . .
RUN cargo install --path .

FROM quay.io/centos/centos:stream9@sha256:aeca8aee6df1e62c25f306396dc6631520dc350fe90d48cf30f45272ffeb2c61
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