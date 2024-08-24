use std::{
    io::{BufWriter, Cursor, Error},
    path::Path,
};

use image::{io::Reader as imageReader, DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgb};
use tokio::{fs::File, io::AsyncWriteExt};

#[allow(dead_code)]
pub async fn save_svg(data: String) -> Result<(), Error> {
    let mut file = File::create("tmp/image.svg").await?;
    file.write_all(&data.as_bytes()).await?;
    Ok(())
}

#[allow(dead_code)]
pub fn open_image(path: &Path) -> Result<DynamicImage, ImageError> {
    let img = imageReader::open(&path)
        .ok()
        .expect("Image not found")
        .decode()?;
    Ok(img)
}

#[allow(dead_code)]
pub async fn open_file(path: &Path) -> Result<File, Error> {
    let file = File::open(path).await?;
    Ok(file)
}

pub fn reader_image(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(Vec<u8>, String), Error> {
    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));

    let _ = image.write_to(&mut buffer, ImageFormat::WebP);

    let reader = imageReader::new(Cursor::new(buffer.into_inner().unwrap().into_inner()))
        .with_guessed_format()
        .expect("load raw data failed");

    let format = reader.format().unwrap().to_mime_type();
    let bytes_image = reader.into_inner().into_inner();

    Ok((bytes_image, format.to_lowercase()))
}
