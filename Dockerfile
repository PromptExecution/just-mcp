# syntax=docker/dockerfile:1

# Multi-stage build for just-mcp with multi-platform support
# Builds static musl binaries for minimal final image size

# NOTE: ARGs `BUILDPLATFORM` + `TARGETARCH` are implicitly defined by BuildKit:
# https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope

# Builder stage - compile just-mcp from source
# Need nightly for edition2024 support
FROM docker.io/rustlang/rust:nightly-alpine AS builder
ARG TARGETARCH

# Install build dependencies
# Note: musl provides static linking by default, no need for separate static libs
RUN apk add --no-cache \
    musl-dev \
    pkgconfig

# Set up cross-compilation targets based on target architecture
RUN case "${TARGETARCH}" in \
    amd64) echo "x86_64-unknown-linux-musl" > /tmp/rust-target ;; \
    arm64) echo "aarch64-unknown-linux-musl" > /tmp/rust-target ;; \
    *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    RUST_TARGET=$(cat /tmp/rust-target) && \
    rustup target add "${RUST_TARGET}"

# Set working directory
WORKDIR /build

# Copy dependency manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY just-mcp-lib/Cargo.toml just-mcp-lib/

# Create dummy source files to cache dependencies
RUN mkdir -p src just-mcp-lib/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > just-mcp-lib/src/lib.rs

# Build dependencies only (cached layer)
RUN RUST_TARGET=$(cat /tmp/rust-target) && \
    cargo build --release --target "${RUST_TARGET}"

# Remove dummy files and copy actual source code
RUN rm -rf src just-mcp-lib/src
COPY src ./src
COPY just-mcp-lib/src ./just-mcp-lib/src

# Build the actual binary
# Using RUSTFLAGS to ensure fully static linking
RUN RUST_TARGET=$(cat /tmp/rust-target) && \
    RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release --target "${RUST_TARGET}" && \
    cp "target/${RUST_TARGET}/release/just-mcp" /tmp/just-mcp && \
    strip /tmp/just-mcp

# Final stage - minimal runtime image
FROM scratch

# MCP registry label ensures OCI metadata verifies package ownership
LABEL io.modelcontextprotocol.server.name="io.github.PromptExecution/just-mcp"

# Copy the static binary from builder
COPY --from=builder /tmp/just-mcp /usr/local/bin/just-mcp

# Set the binary as entrypoint
ENTRYPOINT ["/usr/local/bin/just-mcp"]

# Default to showing help if no arguments provided
CMD ["--help"]
