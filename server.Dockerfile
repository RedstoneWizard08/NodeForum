# ====================================================================
# =                                                                  =
# =                     NodeForum Backend Server                     =
# =                                                                  =
# ====================================================================

# Create the NodeForum builder
FROM rust:latest as builder

# Install musl tools
RUN apt-get update && \
    apt-get -y install \
    musl musl-dev musl-tools

# Install the Rust toolchain
RUN rustup target add $(uname -m)-unknown-linux-musl

# Add the code
ADD ./packages/backend /app
WORKDIR /app

# Compile NodeForum
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build \
        --release --target $(uname -m)-unknown-linux-musl

# Copy the binary
RUN cp /app/target/$(uname -m)-unknown-linux-musl/release/nodeforum /app/nodeforum

# Create the binary-only image
FROM scratch

# Copy the binary
COPY --from=builder /app/nodeforum /nodeforum

# Set the start command
ENTRYPOINT [ "/nodeforum" ]
CMD [ "/nodeforum" ]
