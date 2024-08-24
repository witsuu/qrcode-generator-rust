use std::path::Path;

use axum::{
    body::Body,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use tokio_util::io::ReaderStream;

use super::img::open_file;

const YEAR_TO_SECONDS: u32 = 31536000;

pub fn get_header(age: u32, mime_type: Option<&str>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let cache_age = if age <= 0 {
        "no-cache".to_string()
    } else {
        format!("public, max-age={}", age)
    };

    let content_type = match mime_type {
        Some(mime) => Some(mime).unwrap(),
        _ => "text/plain",
    };

    headers.insert(header::CACHE_CONTROL, cache_age.parse().unwrap());
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"qrcode.webp\"".parse().unwrap(),
    );

    headers
}

pub fn response_error(status_code: StatusCode) -> Response {
    (get_header(YEAR_TO_SECONDS, None), status_code).into_response()
}

pub async fn response_bytes(bytes: Vec<u8>, mime_type: Option<&str>) -> Response {
    (get_header(YEAR_TO_SECONDS, mime_type), bytes).into_response()
}

#[allow(dead_code)]
pub async fn response_file(path: &Path, mime_type: Option<&str>) -> Response {
    let file = match open_file(path).await {
        Ok(file) => file,
        Err(_) => {
            return response_error(StatusCode::NOT_FOUND);
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    (get_header(YEAR_TO_SECONDS, mime_type), body).into_response()
}
