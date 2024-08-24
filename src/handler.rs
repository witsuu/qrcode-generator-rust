use std::io::Cursor;

use attohttpc::Method;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use image::Rgb;

use crate::utils::{
    create_qrcode,
    http::{response_bytes, response_error},
    img::reader_image,
};
use axum_macros::debug_handler;

#[debug_handler]
pub async fn generate_qrcode(Json(body): Json<QRCodeBodyDefault>) -> impl IntoResponse {
    let image = match create_qrcode::new(body.data, body.width) {
        Ok(qr) => qr,
        Err(_) => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let (bytes, mime_type) = match reader_image(image) {
        Ok(img) => img,
        Err(_) => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    return response_bytes(bytes, Some(&mime_type)).await;
}

#[debug_handler]
pub async fn generate_qrcode_with_logo(Json(body): Json<QRCodeBodyWithLogo>) -> impl IntoResponse {
    let mut image: image::ImageBuffer<Rgb<u8>, Vec<u8>> =
        create_qrcode::new(body.data, body.width).unwrap();

    let bytes_logo = match attohttpc::RequestBuilder::new(Method::GET, body.logoUrl).send() {
        Ok(r) => match r.is_success() {
            true => r.bytes().unwrap(),
            false => {
                return response_error(StatusCode::NOT_FOUND);
            }
        },
        _ => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let image2 = match image::io::Reader::new(Cursor::new(bytes_logo)).with_guessed_format() {
        Ok(img) => img.decode().unwrap().into_rgb8(),
        Err(_) => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let logo_height = match body.logoHeight {
        Some(h) => Some(h).unwrap(),
        _ => Some((image2.height() as f32 / image2.width() as f32) * body.logoWidth as f32)
            .unwrap()
            .round() as u32,
    };

    let resize_image2 = image::imageops::resize(
        &image2,
        body.logoWidth,
        logo_height,
        image::imageops::FilterType::Nearest,
    );

    let x = (image.width() as i64 / 2) - (resize_image2.width() as i64 / 2);
    let y = (image.height() as i64 / 2) - (resize_image2.height() as i64 / 2);

    image::imageops::overlay(&mut image, &resize_image2, x, y);

    let (bytes, mime_type) = match reader_image(image) {
        Ok(img) => img,
        Err(_) => {
            return response_error(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    return response_bytes(bytes, Some(&mime_type)).await;
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
