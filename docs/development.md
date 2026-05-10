# Development Documentation

This document provides information for developers who wish to contribute to or modify the QR Code Generator service.

## Project Structure
```
.
├── src/
│   ├── main.rs         # Application entry point, state initialization, and server setup.
│   ├── route.rs        # Route definitions and mapping to handlers.
│   ├── handler.rs      # Request handlers and business logic.
│   └── utils/          # Utility modules for core functionality.
│       ├── mod.rs      # Module declarations.
│       ├── create_qrcode.rs # QR code generation logic.
│       ├── http.rs     # HTTP response helpers.
│       └── img.rs      # Image processing and I/O utilities.
├── docs/               # Project documentation.
├── Dockerfile          # Multi-stage Docker build configuration.
├── docker-compose.yml  # Docker Compose orchestration.
└── Cargo.toml          # Rust dependencies and metadata.
```

## Setup for Development
1. **Install Rust:** Follow instructions at [rustup.rs](https://rustup.rs/).
2. **Install Dependencies:** (Optional) If running on Linux, you might need `pkg-config` and `libssl-dev`.
3. **IDE:** [VS Code](https://code.visualstudio.com/) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension is highly recommended.

## Common Commands
- **Check code:** `cargo check`
- **Build:** `cargo build`
- **Run (dev):** `cargo run`
- **Test:** `cargo test`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`

## Coding Standards
- **Immutability:** Favor immutable variables where possible.
- **Error Handling:** Use `Result` and `Option` for robust error handling. Avoid `unwrap()` unless in tests or where panic is intended and documented.
- **Async/Await:** Use async/await for I/O operations. Use `spawn_blocking` for CPU-bound tasks.
- **Documentation:** Use doc comments (`///`) for public functions and modules.

## Tech Stack Deep Dive
- **Axum:** Provides a clean, macro-free (mostly) way to define routes and extract request data.
- **Tokio:** The engine behind the service's high concurrency.
- **Moka:** A high-performance, concurrent caching library inspired by Caffeine (Java).
- **Image/QRCode:** Low-level libraries used for the core functionality of the app.
