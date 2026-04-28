use image::{DynamicImage};
use std::io::Cursor;

pub fn compress_size(img: &DynamicImage, quality: u8) -> u64 {
    let mut buffer = Cursor::new(Vec::new());

    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
        &mut buffer,
        quality,
    );

    encoder.encode_image(img).unwrap();

    buffer.get_ref().len() as u64
}