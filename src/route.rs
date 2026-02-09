use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{generate_qrcode, generate_qrcode_with_logo, health_check},
    AppState,
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to QRCode Generator API" }))
        .route("/health", get(health_check))
        .route(
            "/api/generate-qrcode",
            post(generate_qrcode).get(|| async { "QRCode Generator" }),
        )
        .route(
            "/api/generate-qrcode-with-logo",
            post(generate_qrcode_with_logo),
        )
        .with_state(state)
}
