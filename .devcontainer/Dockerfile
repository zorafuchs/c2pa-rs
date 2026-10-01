FROM rust:1-bookworm

ENV CARGO_HOME=/usr/local/cargo
ENV PATH=${CARGO_HOME}/bin:${PATH}

# System dependencies for c2pa-rs (OpenSSL, C toolchain, bindgen/clang, etc.)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    llvm \
    cmake \
    make \
    git \
    curl \
    wget \
    unzip \
    ca-certificates \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Rust components and targets
RUN rustup component add clippy rustfmt rust-src rust-analyzer && \
    rustup target add wasm32-unknown-unknown

# Handy cargo tooling (used by the repo's Makefile / CI); remove if you don't need it
RUN cargo install --locked cargo-nextest cargo-deny cargo-audit wasm-pack

WORKDIR /workspace
