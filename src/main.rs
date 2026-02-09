mod handler;
mod route;
mod utils;

use std::time::Duration;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    response::IntoResponse,
};
use moka::future::Cache;

use route::create_route;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub http_client: reqwest::Client,
    pub qr_cache: Cache<String, Vec<u8>>,
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT_FOUND")
}

#[tokio::main]
async fn main() {
    // Initialize HTTP client with connection pooling
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to create HTTP client");

    // Initialize cache with max capacity and TTL to prevent memory leaks
    let qr_cache = Cache::builder()
        .max_capacity(1000) // Max 1000 entries
        .time_to_live(Duration::from_secs(300)) // 5 minutes TTL
        .build();

    let state = AppState {
        http_client,
        qr_cache,
    };

    // Setup CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(false)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Build route with state
    let app = create_route(state).layer(cors).fallback(handler_404);

    // Run server with graceful shutdown
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3200").await.unwrap();

    println!("Server running on http://0.0.0.0:3200");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received, starting graceful shutdown");
}
