# Deployment Documentation

This document provides instructions for deploying the QR Code Generator service.

## Local Deployment

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Steps
1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd qrcode-generator-rust
   ```

2. **Run the application:**
   ```bash
   cargo run --release
   ```
   The server will start on `http://0.0.0.0:3200`.

---

## Docker Deployment

The service includes a multi-stage `Dockerfile` and `docker-compose.yml` for easy containerized deployment.

### Build and Run with Docker Compose
```bash
docker-compose up -d --build
```

### Manual Docker Build
```bash
docker build -t qrcode-generator .
docker run -p 3200:3200 qrcode-generator
```

### Dockerfile Details
The `Dockerfile` uses a multi-stage build process to ensure a small final image:
1. **Builder Stage:** Uses `rust:1.93-slim-bookworm` to compile the application with its dependencies. It uses a "dummy" build strategy to cache dependencies, significantly speeding up subsequent builds.
2. **Runtime Stage:** Uses `debian:bookworm-slim`. It installs only necessary runtime libraries (`libssl3`, `ca-certificates`).
3. **Security:** The application runs as a non-root user (`appuser`) for enhanced security.

### Health Check
The Docker Compose configuration includes a health check:
- **Test:** `curl -f http://localhost:3200/health`
- **Interval:** 30 seconds
- **Retries:** 3

---

## Environment Variables
Currently, the application uses hardcoded defaults (Port 3200). In future iterations, these can be moved to environment variables.

## Resource Requirements
- **Memory:** Minimum 128MB (Can vary depending on traffic and cache size).
- **CPU:** Dependent on QR generation frequency. The application scales well with multiple cores thanks to Tokio's multithreaded runtime.
