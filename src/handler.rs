use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::Cursor,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use image::Rgb;

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

    // Check cache first
    if let Some(cached_bytes) = state.qr_cache.get(&cache_key).await {
        return response_bytes(cached_bytes, Some("image/webp")).await;
    }

    // Clone values for async block
    let http_client = state.http_client.clone();
    let logo_url = body.logoUrl.clone();
    let data = body.data.clone();
    let width = body.width;
    let logo_width = body.logoWidth;
    let logo_height = body.logoHeight;

    let result = tokio::task::spawn_blocking(move || {
        // This needs to be in a blocking context because we're doing CPU-intensive work
        let mut image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = create_qrcode::new(data, width)
            .map_err(|e| match e {
                qrcode::types::QrError::DataTooLong => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })?;

        // Fetch logo - we need to use blocking reqwest call here
        let bytes_logo = tokio::runtime::Handle::current().block_on(async {
            http_client
                .get(&logo_url)
                .send()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .bytes()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        })?;

        let image2 = match image::io::Reader::new(Cursor::new(bytes_logo)).with_guessed_format() {
            Ok(img) => img
                .decode()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .into_rgb8(),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let logo_height = match logo_height {
            Some(h) => h,
            _ => {
                (image2.height() as f32 / image2.width() as f32 * logo_width as f32).round() as u32
            }
        };

        let resize_image2 = image::imageops::resize(
            &image2,
            logo_width,
            logo_height,
            image::imageops::FilterType::Nearest,
        );

        let x = (image.width() as i64 / 2) - (resize_image2.width() as i64 / 2);
        let y = (image.height() as i64 / 2) - (resize_image2.height() as i64 / 2);

        image::imageops::overlay(&mut image, &resize_image2, x, y);

        let (bytes, mime_type) =
            reader_image(image).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok((bytes, mime_type))
    })
    .await;

    match result {
        Ok(Ok((bytes, mime_type))) => {
            // Store in cache
            state.qr_cache.insert(cache_key, bytes.clone()).await;
            response_bytes(bytes, Some(&mime_type)).await
        }
        Ok(Err(err)) => response_error(err),
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
