FROM rust:latest@sha256:6de6071df133f8be44dd4538c74e93590941c6d2b9c98853e05011714fbcf57d AS builder
WORKDIR /arcalog
COPY . .
RUN cargo install --path .

FROM quay.io/centos/centos:stream9@sha256:30178f32547587d82188bb3ffa27f394da32d311e590f4d7cd9a66e2953c1d68
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