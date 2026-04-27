use std::env;
use std::time::Duration;
use std::thread::sleep;

use ffmpeg_next as ffmpeg;
use image::{DynamicImage, ImageBuffer, Luma};

const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- <video.mp4> [--width 80] [--fps 24]");
        return;
    }

    let video_path = &args[1];

    let mut width: u32 = 120;
    let mut fps: u32 = 24;

    // CLI flags
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--width" => {
                if i + 1 < args.len() {
                    width = args[i + 1].parse().unwrap_or(120);
                    i += 1;
                }
            }
            "--fps" => {
                if i + 1 < args.len() {
                    fps = args[i + 1].parse().unwrap_or(24);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    ffmpeg::init().unwrap();

    let mut ictx = ffmpeg::format::input(video_path).expect("Cannot open video");
    let input = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .expect("No video stream");

    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())
        .unwrap()
        .decoder()
        .video()
        .unwrap();

    let mut decoder = context_decoder;

    let time_per_frame = Duration::from_secs_f32(1.0 / fps as f32);

    println!("🎥 Playing ASCII video @ {} FPS, width={}", fps, width);

    for (stream, packet) in ictx.packets() {
        if stream.index() != video_stream_index {
            continue;
        }

        decoder.send_packet(&packet).unwrap();

        let mut frame = ffmpeg::util::frame::Video::empty();

        while decoder.receive_frame(&mut frame).is_ok() {
            let ascii_frame = frame_to_ascii(&frame, width);

            print!("\x1B[2J\x1B[H"); // clear terminal
            println!("{}", ascii_frame);

            sleep(time_per_frame);
        }
    }
}

fn to_ascii(img: DynamicImage, width: u32) -> String {
    let gray = img.to_luma8();

    let (w, h) = gray.dimensions();

    let scale = width as f32 / w as f32;
    let new_h = (h as f32 * scale * 0.55) as u32; // adjust aspect ratio for terminal

    let resized = image::imageops::resize(
        &gray,
        width,
        new_h,
        image::imageops::FilterType::Nearest,
    );

    let mut output = String::new();

    for y in 0..resized.height() {
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y)[0];

            let idx = map_to_ascii(pixel);
            output.push(ASCII_CHARS[idx] as char);
        }
        output.push('\n');
    }

    output
}

fn map_to_ascii(value: u8) -> usize {
    let len = ASCII_CHARS.len() - 1;

    let scaled = (value as f32 / 255.0) * len as f32;

    len - scaled as usize // invert so dark = dense characters
}


fn frame_to_ascii(frame: &ffmpeg::util::frame::Video, width: u32) -> String {
    let (w, h) = (frame.width(), frame.height());

    let data = frame.data(0);

    let img = ImageBuffer::<Luma<u8>, Vec<u8>>::from_raw(w, h, data.to_vec())
        .expect("Frame conversion failed");

    let dyn_img = DynamicImage::ImageLuma8(img);

    to_ascii(dyn_img, width)
}

fn map_to_ascii(v: u8) -> usize {
    let len = ASCII_CHARS.len() - 1;

    let scaled = (v as f32 / 255.0) * len as f32;

    len - scaled as usize
}