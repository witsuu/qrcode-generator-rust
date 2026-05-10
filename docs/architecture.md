# Architecture Documentation

This document outlines the high-level architecture and technical decisions of the QR Code Generator service.

## Tech Stack
- **Language:** [Rust](https://www.rust-lang.org/) (2021 edition)
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
- **Image Processing:** [image](https://github.com/image-rs/image)
- **QR Code Generation:** [qrcode-rust](https://github.com/kennytm/qrcode-rust)
- **Caching:** [Moka](https://github.com/moka-rs/moka)
- **HTTP Client:** [reqwest](https://github.com/seanmonstar/reqwest)
- **Metrics:** [axum-prometheus](https://github.com/clux/axum-prometheus)

## High-Level Design
The service is built as a stateless REST API (with in-memory caching) designed for high performance and reliability.

### 1. Request Handling
The application uses **Axum**, a web framework that leverages **Tokio** and **Tower**. Requests are handled asynchronously.

### 2. Caching Strategy
To minimize redundant CPU-intensive operations (QR generation and image processing), the service implements an in-memory cache using **Moka**.
- **Cache Key:** A hexadecimal hash (DefaultHasher) of the request parameters (data, width, logo URL, logo dimensions).
- **Configuration:**
  - `max_capacity`: 1000 entries.
  - `time_to_live` (TTL): 300 seconds (5 minutes).
- **Behavior:** On each request, the service checks the cache. If a hit occurs, the cached binary data (WebP) is returned immediately.

### 3. Concurrency & Blocking Tasks
Generating QR codes and performing image operations (resizing, overlaying) are CPU-bound tasks. To prevent blocking the async executor's threads, these operations are wrapped in `tokio::task::spawn_blocking`. This ensures that the service can continue to handle incoming I/O requests while heavy computation happens in a dedicated thread pool.

### 4. Image Processing
- **Format:** The service serves images in **WebP** format for optimal compression and quality.
- **Logo Embedding:** Logos are fetched via `reqwest`, decoded, resized using `Nearest` filter, and overlaid on top of the generated QR code.

### 5. Error Correction
The QR codes are generated with Error Correction Level **M** (Medium - 15% recovery), which provides a good balance between data density and robustness, especially when embedding logos.

### 6. Observability
Integration with **Prometheus** allows for monitoring request rates, latencies, and other vital signs. Metrics are exposed via the `/metrics` endpoint.

### 7. Reliability
- **Connection Pooling:** `reqwest` client is configured with connection pooling (`pool_max_idle_per_host`).
- **Graceful Shutdown:** The server listens for `SIGINT` and `SIGTERM` signals to perform a graceful shutdown, ensuring in-flight requests are completed.
