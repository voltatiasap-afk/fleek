use anyhow::Result;
use colored::Colorize;
use image::{Rgb, open};

use crate::cli::DecodeArgs;

pub fn handle(args: DecodeArgs) -> Result<()> {
    let image = args.image;

    decode(image)
}

fn decode(image: String) -> Result<()> {
    let image = open(image)?.to_rgb8();

    if *image.get_pixel(0, 0) == Rgb([10, 10, 10]) {
        let mut curr_half = 0;
        let mut bytes: Vec<u8> = Vec::new();
        let mut byte = 0u8;
        for (_, _, pixel) in image.enumerate_pixels() {
            let Rgb([r, g, b]) = *pixel;

            if (r, g, b) == (10, 10, 10) {
                continue;
            }

            if (r, g, b) == (33, 33, 33) {
                break;
            }

            if curr_half == 0 {
                let high = (r & 0b00000011) << 6;
                let low = (b & 0b00000011) << 4;

                curr_half = 1;
                byte = high | low;
            } else {
                let high = (r & 0x03) << 2;
                let low = b & 0x03;

                curr_half = 0;
                byte |= high | low;

                bytes.push(byte);
                byte = 0u8;
            }
        }
        let file_type = infer::get(&bytes);
        // if bytes.len() % 2 == 1 {
        //     bytes.pop();
        // }
        match file_type {
            Some(p) => {
                std::fs::write(format!("output.{}", p.extension()), bytes)?;
                println!("Saved to {}.{}", "output".blue(), p.extension().blue());
            }
            None => {
                let text = String::from_utf8_lossy(&bytes);
                println!("Decoded text: {}", text.blue());
            }
        }
    } else {
        println!("This image is not from fleek");
    }

    Ok(())
}
