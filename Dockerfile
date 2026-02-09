# =========================
# Build stage
# =========================
FROM rust:1.93-slim-bookworm AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser
USER appuser
WORKDIR /app

COPY --from=builder /usr/src/app/target/release/qrcode-generator ./

EXPOSE 3200
CMD ["./qrcode-generator"]
