use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::Cursor,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    utils::{
        create_qrcode,
        http::{response_bytes, response_error},
        img::reader_image,
    },
    AppState,
};
use axum_macros::debug_handler;

// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[debug_handler]
pub async fn generate_qrcode(
    State(state): State<AppState>,
    Json(body): Json<QRCodeBodyDefault>,
) -> impl IntoResponse {
    // Generate cache key
    let cache_key = generate_cache_key(&body.data, body.width, None, None, None);

    // Check cache first
    if let Some(cached_bytes) = state.qr_cache.get(&cache_key).await {
        return response_bytes(cached_bytes, Some("image/webp")).await;
    }

    // Generate QR code in blocking thread
    let data = body.data.clone();
    let width = body.width;
    let result = tokio::task::spawn_blocking(move || create_qrcode::new(data, width)).await;

    let image = match result {
        Ok(Ok(qr)) => qr,
        Ok(Err(qrcode::types::QrError::DataTooLong)) => {
            return response_error(StatusCode::BAD_REQUEST);
        }
        _ => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let (bytes, mime_type) = match reader_image(image) {
        Ok(img) => img,
        Err(_) => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Store in cache
    state.qr_cache.insert(cache_key, bytes.clone()).await;

    response_bytes(bytes, Some(&mime_type)).await
}

#[debug_handler]
pub async fn generate_qrcode_with_logo(
    State(state): State<AppState>,
    Json(body): Json<QRCodeBodyWithLogo>,
) -> impl IntoResponse {
    // Generate cache key
    let cache_key = generate_cache_key(
        &body.data,
        body.width,
        Some(&body.logoUrl),
        Some(body.logoWidth),
        body.logoHeight,
    );

    if let Some(cached_bytes) = state.qr_cache.get(&cache_key).await {
        return response_bytes(cached_bytes, Some("image/webp")).await;
    }

    // 1. FETCH LOGO ASYNC (AMAN)
    let logo_bytes = match state.http_client.get(&body.logoUrl).send().await {
        Ok(resp) => match resp.bytes().await {
            Ok(b) => b,
            Err(_) => return response_error(StatusCode::BAD_REQUEST),
        },
        Err(_) => return response_error(StatusCode::BAD_REQUEST),
    };

    let data = body.data.clone();
    let width = body.width;
    let logo_width = body.logoWidth;
    let logo_height = body.logoHeight;
    let logo_bytes = logo_bytes.to_vec();

    // 2. CPU WORK ONLY
    let result = tokio::task::spawn_blocking(move || {
        let mut image = create_qrcode::new(data, width).map_err(|_| StatusCode::BAD_REQUEST)?;

        let logo = image::io::Reader::new(Cursor::new(logo_bytes))
            .with_guessed_format()
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .decode()
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .into_rgb8();

        let target_height = logo_height.unwrap_or_else(|| {
            (logo.height() as f32 / logo.width() as f32 * logo_width as f32) as u32
        });

        let resized = image::imageops::resize(
            &logo,
            logo_width,
            target_height,
            image::imageops::FilterType::Nearest,
        );

        let x = (image.width() as i64 - resized.width() as i64) / 2;
        let y = (image.height() as i64 - resized.height() as i64) / 2;

        image::imageops::overlay(&mut image, &resized, x, y);

        reader_image(image).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    })
    .await;

    match result {
        Ok(Ok((bytes, mime))) => {
            state.qr_cache.insert(cache_key, bytes.clone()).await;
            response_bytes(bytes, Some(&mime)).await
        }
        Ok(Err(e)) => response_error(e),
        Err(_) => response_error(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn generate_cache_key(
    data: &str,
    width: u32,
    logo_url: Option<&str>,
    logo_width: Option<u32>,
    logo_height: Option<u32>,
) -> String {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    width.hash(&mut hasher);
    logo_url.hash(&mut hasher);
    logo_width.hash(&mut hasher);
    logo_height.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct QRCodeBodyDefault {
    pub data: String,
    pub width: u32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct QRCodeBodyWithLogo {
    pub data: String,
    pub width: u32,
    pub logoUrl: String,
    pub logoWidth: u32,
    pub logoHeight: Option<u32>,
}
