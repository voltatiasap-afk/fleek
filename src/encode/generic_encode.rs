use anyhow::{Context, Result};
use colored::Colorize;
use flate2::{Compression, write::DeflateEncoder};
use image::{Rgb, imageops, open};
use std::fs;
use std::io::prelude::*;

// pub fn handle(args: EncodeArgs, source: Source) {
//     let bits = args.bits;
// }

pub fn text_encode(image: String, path: String, text: String, compression: u32) -> Result<()> {
    let text_bytes = text.as_bytes().to_vec();
    encode(image, path, text_bytes, compression)
}

pub fn file_encode(image: String, path: String, file: String, compression: u32) -> Result<()> {
    let file_bytes = fs::read(file)?;
    encode(image, path, file_bytes, compression)
}

fn encode(image: String, path: String, mut data: Vec<u8>, compression: u32) -> Result<()> {
    let mut image = open(image)?.to_rgb8();

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::new(compression));

    encoder.write_all(&data)?;

    data = encoder.finish()?;
    let data_len = (data.len() as u32).to_le_bytes();

    data.splice(0..0, data_len);

    let (width, height) = &image.dimensions();
    if (data.len() as u32 * 2) > (width * height - 2) {
        println!("{}", "Resizing image".blue());

        let target_pixels: f64 = (data.len() + 20) as f64 * 2.0;

        let aspect = (*width as f64 / *height as f64).ceil();
        let side = ((target_pixels / aspect).sqrt()).ceil();
        let target_height = side;
        let target_width = (side * aspect) as u32;
        image = imageops::resize(
            &image,
            target_width,
            target_height as u32,
            imageops::FilterType::Nearest,
        );
    }

    let mut curr_half = 0;
    let mut curr_byte: usize = 0;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if (x, y) == (0, 0) {
            *pixel = Rgb([10, 10, 10]);
            continue;
        }

        let Rgb([r, g, b]) = *pixel;

        if curr_byte >= data.len() {
            break;
        }

        let target_r: u8;
        let target_b: u8;

        if curr_half == 0 {
            let half = (data[curr_byte] & 0xF0) >> 4;
            let high = (half & 0b0000_1100) >> 2;
            let low = half & 0b0000_0011;

            target_r = (r & !0x03) | high;
            target_b = (b & !0x03) | low;

            curr_half = 1;
        } else {
            let half = data[curr_byte] & 0x0F;

            let high = (half & 0b0000_1100) >> 2;
            let low = half & 0b0000_0011;

            target_r = (r & !0x03) | high;
            target_b = (b & !0x03) | low;
            curr_half = 0;
            curr_byte += 1;
        }

        *pixel = Rgb([target_r, g, target_b]);
    }

    let output = image.save(&path).with_context(|| "Could not save image");
    println!("Saved to {}", path.blue());
    output
}
