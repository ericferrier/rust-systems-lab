use image::{DynamicImage, Rgba};
use image::GenericImageView;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

// Draws a visible text watermark in the bottom-right corner
pub fn add_text_like_watermark(img: DynamicImage, text: &str) -> DynamicImage {
    let (w, h) = img.dimensions();
    let mut img = img.to_rgba8();

    // Use a bundled font file
    let font_data: &[u8] = include_bytes!("../fonts/DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data).expect("Font not found");

    let scale = Scale { x: 40.0, y: 40.0 };
    let margin = 20;
    let text_width = (text.len() as u32) * 24; // rough estimate
    let x = w.saturating_sub(text_width + margin);
    let y = h.saturating_sub(40 + margin);

    draw_text_mut(
        &mut img,
        Rgba([255, 255, 255, 128]), // semi-transparent white
        x as i32,
        y as i32,
        scale,
        &font,
        text,
    );

    DynamicImage::ImageRgba8(img)
}