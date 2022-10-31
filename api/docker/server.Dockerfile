# ====================================================================
# =                                                                  =
# =                     NodeForum Backend Server                     =
# =                                                                  =
# ====================================================================

# Create the NodeForum builder
FROM rust:latest

# Set rust to nightly
RUN rustup default nightly

# Add the packages
RUN mkdir -p /app /app/src
ADD Cargo.toml Cargo.lock /app/
ADD ./src/dummy.rs /app/src/main.rs
WORKDIR /app

# Pre-compile NodeForum (for caching)
RUN cargo build --release

# Add the source code
RUN rm /app/src/main.rs
ADD ./assets /app/
ADD ./src/* /app/src/

# Compile NodeForum
RUN mkdir -p /app/out && \
    cargo build --release -Z unstable-options --out-dir /app/out && \
    cp /app/out/nodeforum nodeforum
