# QR Code Generator (Rust)

A high-performance QR Code generator REST API built with Rust, Axum, and Tokio. This application allows you to generate standard QR codes and QR codes with embedded logos.

## Features

- 🚀 **High Performance**: Built with Rust and Axum for speed and efficiency.
- 🖼️ **Customizable**: Generate standard QR codes or embed logos in the center.
- 🐳 **Docker Ready**: Includes Dockerfile and Docker Compose for easy deployment.
- 💓 **Health Check**: Built-in health check endpoint.

## Tech Stack

- **Language**: Rust 2021
- **Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Libraries**:
  - `qrcode`: For QR code generation logic.
  - `image`: For image processing (overlaying logos).
  - `attohttpc`: For fetching remote logo images.

## Getting Started

### Prerequisites

- Rust (cargo) installed (for local development)
- Docker & Docker Compose (for containerized deployment)

### Running Locally

1. **Clone the repository:**

   ```bash
   git clone https://github.com/witsuu/qrcode-generator-rust.git
   cd qrcode-generator-rust
   ```

2. **Run with Cargo:**
   ```bash
   cargo run
   ```
   The server will start on `http://0.0.0.0:3200`.

### Running with Docker

1. **Build and start the container:**

   ```bash
   docker-compose up -d --build
   ```

2. **Verify it's running:**
   ```bash
   curl http://localhost:3200/
   # Output: Welcome to QRCode Generator API
   ```

## API Documentation

The server listens on port `3200` by default.

### 1. Health Check / Welcome

- **URL**: `/`
- **Method**: `GET`
- **Response**: `Welcome to QRCode Generator API`

### 2. Generate Standard QR Code

Generates a simple QR code image (PNG).

- **URL**: `/api/generate-qrcode`
- **Method**: `POST`
- **Content-Type**: `application/json`
- **Body Parameters**:
  - `data` (string, required): The text or URL to encode.
  - `width` (number, required): The width/height of the generated image (pixels).

**Example Request:**

```bash
curl -X POST http://localhost:3200/api/generate-qrcode \
  -H "Content-Type: application/json" \
  -d '{
    "data": "https://github.com/witsuu/qrcode-generator-rust",
    "width": 500
  }' --output qrcode.png
```

### 3. Generate QR Code with Logo

Generates a QR code with an image overlay (logo) in the center.

- **URL**: `/api/generate-qrcode-with-logo`
- **Method**: `POST`
- **Content-Type**: `application/json`
- **Body Parameters**:
  - `data` (string, required): The text or URL to encode.
  - `width` (number, required): The width/height of the generated image (pixels).
  - `logoUrl` (string, required): URL of the logo image to embed.
  - `logoWidth` (number, required): Desired width of the logo.
  - `logoHeight` (number, optional): Desired height of the logo. If omitted, it preserves aspect ratio based on `logoWidth`.

**Example Request:**

```bash
curl -X POST http://localhost:3200/api/generate-qrcode-with-logo \
  -H "Content-Type: application/json" \
  -d '{
    "data": "https://rust-lang.org",
    "width": 500,
    "logoUrl": "https://www.rust-lang.org/logos/rust-logo-512x512.png",
    "logoWidth": 100
  }' --output qrcode_logo.png
```

## Project Structure

```
.
├── src/
│   ├── main.rs       # Entry point, CORS setup, Server start
│   ├── route.rs      # API Route definitions
│   ├── handler.rs    # Request handlers (Business logic)
│   └── utils/        # Utility functions (QR generation, Image processing)
├── Dockerfile        # Multi-stage Docker build
├── docker-compose.yml # Docker composition
└── Cargo.toml        # Dependencies
```

## License

[MIT](LICENSE)
