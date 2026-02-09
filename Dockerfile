# =========================
# Build stage
# =========================
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

# Build dependencies
RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      libssl-dev \
      ca-certificates \
      curl \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source
COPY . .

# Build application
RUN cargo build --release


# =========================
# Runtime stage
# =========================
FROM debian:bookworm-slim

# Runtime dependencies only
RUN apt-get update && \
    apt-get install -y \
      ca-certificates \
      libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Non-root user
RUN useradd -m appuser
USER appuser

WORKDIR /app

# Copy binary
COPY --from=builder /usr/src/app/target/release/qrcode-generator /app/qrcode-generator

EXPOSE 3200

CMD ["./qrcode-generator"]
