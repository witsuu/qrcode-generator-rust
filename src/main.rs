mod handler;
mod route;
mod utils;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    response::IntoResponse,
};

use route::create_route;
use tower_http::cors::{Any, CorsLayer};

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT_FOUND")
}

#[tokio::main]
async fn main() {
    // setup cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(false)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // build route
    let app = create_route().layer(cors);

    let app = app.fallback(handler_404);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3200").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
