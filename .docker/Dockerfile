# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/zix

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new directory for the source code
RUN mkdir src

# Copy the source code
COPY src ./src

# Install required target (for example, x86_64-unknown-linux-gnu and x86_64-pc-windows-gnu)
RUN rustup target add x86_64-unknown-linux-gnu x86_64-pc-windows-gnu

# Build the application for Linux
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Build the application for Windows
RUN cargo build --release --target x86_64-pc-windows-gnu

# Use a smaller image to run the application
FROM debian:bullseye-slim AS runtime

# Copy the built Linux binary from the builder stage
COPY --from=builder /usr/src/zix/target/x86_64-unknown-linux-gnu/release/zix /usr/local/bin/zix-linux

# Copy the built Windows binary from the builder stage (if needed)
COPY --from=builder /usr/src/zix/target/x86_64-pc-windows-gnu/release/zix.exe /usr/local/bin/zix.exe

# Set the entry point for the Linux binary
ENTRYPOINT ["zix-linux"]