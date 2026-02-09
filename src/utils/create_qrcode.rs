use image::{ImageBuffer, Rgb};
use qrcode::{types::QrError, EcLevel, QrCode};

pub fn new(data: String, width: u32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, QrError> {
    if width == 0 {
        return Err(QrError::InvalidVersion);
    }

    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::M)?;

    let module_count = code.width() as u32;
    let scale = (width / module_count).max(1);

    let qr_image = code
        .render::<Rgb<u8>>()
        .quiet_zone(false)
        .module_dimensions(scale, scale)
        .build();

    let qr_size: u32 = module_count * scale;

    let mut final_image = ImageBuffer::from_pixel(width, width, Rgb([255, 255, 255]));

    let offset = (width - qr_size) / 2;

    image::imageops::overlay(&mut final_image, &qr_image, offset as i64, offset as i64);

    Ok(final_image)
}
