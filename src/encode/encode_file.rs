use anyhow::{Context, Result};
use colored::Colorize;
use image::{Rgb, imageops, open};
use std::fs;

// pub fn handle(args: EncodeArgs, source: Source) {
//     let bits = args.bits;
// }

pub fn encode(image: String, path: String, file: String) -> Result<()> {
    let mut image = open(image)?.to_rgb8();

    let file = fs::read(file)?;

    let (width, height) = &image.dimensions();
    if (file.len() as u32 * 2) > (width * height - 2) {
        println!(
            "{}",
            "Resizing image in order to be able to fit the file. Still very buggy so might not work".blue()
        );

        let target_pixels: f64 = (file.len() + 5) as f64 * 2.0;

        let aspect = *width as f64 / *height as f64;
        let side = target_pixels.sqrt() as u32;
        let target_height = side;
        let target_width = side * aspect as u32;
        println!(
            "Debug: aspect: {}, height: {}, width: {}",
            aspect, target_height, target_width
        );
        image = imageops::resize(
            &image,
            target_width,
            target_height,
            imageops::FilterType::Triangle,
        );
    }

    let mut curr_half = 0;
    let mut curr_byte: usize = 0;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if (x, y) == (0, 0) {
            *pixel = Rgb([10, 10, 10]);
            continue;
        }

        if curr_byte >= file.len() {
            *pixel = Rgb([33, 33, 33]);
            break;
        }

        let Rgb([r, g, b]) = *pixel;

        let target_r: u8;
        let target_b: u8;

        if curr_half == 0 {
            let half = (file[curr_byte] & 0xF0) >> 4;
            let high = (half & 0b0000_1100) >> 2;
            let low = half & 0b0000_0011;

            target_r = (r & !0x03) | high;
            target_b = (b & !0x03) | low;

            curr_half = 1;
        } else {
            let half = file[curr_byte] & 0x0F;

            let high = (half & 0b0000_1100) >> 2;
            let low = half & 0b0000_0011;

            target_r = (r & !0x03) | high;
            target_b = (b & !0x03) | low;
            curr_half = 0;
            curr_byte += 1;
        }

        *pixel = Rgb([target_r, g, target_b]);
    }

    println!("Saved to {}", path.blue());
    image.save(path).with_context(|| "Could not save image")
}
