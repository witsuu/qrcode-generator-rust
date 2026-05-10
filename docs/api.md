# API Documentation

This document describes the available API endpoints for the QR Code Generator service.

## Base URL
The service runs by default on `http://localhost:3200`.

## Endpoints

### 1. Welcome
Returns a simple welcome message.

- **URL:** `/`
- **Method:** `GET`
- **Success Response:**
  - **Code:** 200 OK
  - **Content:** `Welcome to QRCode Generator API`

---

### 2. Health Check
Returns the health status of the service.

- **URL:** `/health`
- **Method:** `GET`
- **Success Response:**
  - **Code:** 200 OK
  - **Content:** `OK`

---

### 3. Metrics
Returns Prometheus metrics for the service.

- **URL:** `/metrics`
- **Method:** `GET`
- **Success Response:**
  - **Code:** 200 OK
  - **Content:** Prometheus formatted metrics data.

---

### 4. Generate Standard QR Code
Generates a standard QR code based on the provided data and width.

- **URL:** `/api/generate-qrcode`
- **Method:** `POST`
- **Content-Type:** `application/json`
- **Request Body:**
  ```json
  {
    "data": "https://example.com",
    "width": 500
  }
  ```
  - `data` (string, required): The text or URL to be encoded in the QR code.
  - `width` (integer, required): The width and height of the resulting image in pixels.
- **Success Response:**
  - **Code:** 200 OK
  - **Content-Type:** `image/webp`
  - **Content:** Binary image data.
  - **Headers:**
    - `Cache-Control: public, max-age=31536000`
    - `Content-Disposition: attachment; filename="qrcode.webp"`
- **Error Responses:**
  - **Code:** 400 Bad Request (if data is too long or width is 0)
  - **Code:** 500 Internal Server Error (if image generation fails)

---

### 5. Generate QR Code with Logo
Generates a QR code with an embedded logo in the center.

- **URL:** `/api/generate-qrcode-with-logo`
- **Method:** `POST`
- **Content-Type:** `application/json`
- **Request Body:**
  ```json
  {
    "data": "https://example.com",
    "width": 500,
    "logoUrl": "https://example.com/logo.png",
    "logoWidth": 100,
    "logoHeight": 100
  }
  ```
  - `data` (string, required): The text or URL to be encoded.
  - `width` (integer, required): The width and height of the QR code image.
  - `logoUrl` (string, required): The URL of the logo image to be embedded.
  - `logoWidth` (integer, required): The desired width of the logo in the QR code.
  - `logoHeight` (integer, optional): The desired height of the logo. If omitted, the aspect ratio is preserved.
- **Success Response:**
  - **Code:** 200 OK
  - **Content-Type:** `image/webp`
  - **Content:** Binary image data.
  - **Headers:**
    - `Cache-Control: public, max-age=31536000`
    - `Content-Disposition: attachment; filename="qrcode.webp"`
- **Error Responses:**
  - **Code:** 400 Bad Request (if data is too long, width is 0, or logo cannot be fetched/decoded)
  - **Code:** 500 Internal Server Error (if image processing fails)

## Error Handling
The API uses standard HTTP status codes:
- `200 OK`: Request succeeded.
- `400 Bad Request`: Invalid parameters or input.
- `404 Not Found`: Endpoint does not exist.
- `500 Internal Server Error`: An unexpected error occurred on the server.
