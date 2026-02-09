# Build stage
FROM rust:slim as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl && \
    rm -rf /var/lib/apt/lists/*

# Create a dummy project to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# Build dependencies only
RUN cargo build --release

# Remove the dummy build artifacts for the application itself
# to ensure the actual source code is rebuilt
RUN rm -f target/release/deps/qrcode_generator*

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -ms /bin/bash appuser

WORKDIR /app
USER appuser

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/qrcode-generator /app/qrcode-generator

# Expose the application port
EXPOSE 3200

# Set the command to run the binary
CMD ["./qrcode-generator"]
