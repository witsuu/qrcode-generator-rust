use image::{ImageBuffer, Rgb};
use qrcode::{EcLevel, QrCode};

pub fn new(data: String, width: u32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, ()> {
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::H).unwrap();

    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = code
        .render::<Rgb<u8>>()
        .min_dimensions(width, width)
        .max_dimensions(width, width)
        .build();
    Ok(image)
}
