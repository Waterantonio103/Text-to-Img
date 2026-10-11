#![allow(unused)]

mod header;
mod args;

use crate::header::*;
use crate::args::Args;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use rand::{Rng, RngExt, seq::IndexedRandom};
use image::{ImageBuffer, RgbImage};
use clap::Parser;

fn main() -> Result<(), FileError> {
    // let args = Args::parse();

    let file = "src/rand.txt";
    let path = Path::new(file);
    let target_extension = OsStr::new("txt");

    if let Some(extension) = path.extension() {
        if extension != target_extension {return Err(FileError::InvalidExtension(format!("Invalid file extension {{ Current -> '.{}' Required -> '.{}' }}", extension.display(), target_extension.display())));}
    }

    let bytes = fs::read(path)?;
    let nbytes = bytes.len();

    if nbytes < 3 {
        return Err(FileError::InsufficientLength(format!("Insufficient Bytes {{ Current -> '{}' Minimum -> '{}' }}", nbytes, 3)));
    }

    let colors = colors(bytes, nbytes);
    let (width, height) = dimensions(colors.len() as f64);

    let pixels = construct_pixels(colors, width, height);

    let mut img = RgbImage::new(width, height);

    for pixel in pixels {
        //careful with width and height, panics here if out of bounds, double check
        img.put_pixel(
            pixel.position.0, 
            pixel.position.1,
            image::Rgb([pixel.color.0, pixel.color.1, pixel.color.2]));
    }

    //temporary placeholder to control flow, replace temp with actual file, possible created using fs?
    img.save(Path::new("src/test3.png"));

    Ok(())
}

fn construct_pixels(mut colors: Vec<Color>, width: u32, height: u32) -> Vec<Pixel> {
    let lngt = colors.len();
    let difference = (lngt).saturating_sub((width as usize) * (height as usize));
    if difference != 0 {
        for _ in 0..difference {
            colors.pop();
        }
    }
    let mut pixels: Vec<Pixel> = Vec::new();
    for (i, color) in colors.iter().copied().enumerate() {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;

        pixels.push(Pixel {
                color,
                position: Position(x, y),
            }
        );
    }
    pixels
}

fn dimensions(size: f64) -> (u32, u32) {
    const LIMIT: f64 = 100_000_000.0;
    const PERCENTAGE: f64 = 0.0001;
    let p: f64 = if size > LIMIT {
        LIMIT
    } else {
        size
    };
    let depth = (p * PERCENTAGE).ceil() as u32;
    let mut best_score = f64::MAX;
    let mut dimensions = (0,0);
    for outer_step in 0..depth {
        let inner_p = p - (outer_step as f64);
        let starter: f64 = inner_p.sqrt().floor();
        let mut inner_step: f64 = 0.0;
        let mut check_div = inner_p % (starter - inner_step);
        while check_div != 0.0 {
            inner_step += 1.0;
            check_div = inner_p % (starter - inner_step);
        }
        let width = starter - inner_step;
        let height = inner_p / width;
        let score = height.max(width) / height.min(width);
        if score < best_score {
            best_score=score; 
            dimensions=(width as u32, height as u32);
        }
        if score == 1.0 { break; }
    }

    return dimensions;
}

fn colors(mut bytes: Vec<u8>, nbytes: usize) -> Vec<Color> {
    let mut fill_in_bytes = fill_bytes(&bytes, nbytes);
    if !fill_in_bytes.is_empty() {
        bytes.append(&mut fill_in_bytes);
    }
    set_colors(&bytes)
}

fn set_colors(bytes: &[u8]) -> Vec<Color> {
    bytes.as_chunks::<3>().0.iter()
    .map(|&[r,g,b]| Color(r,g,b))
    .collect()
}

fn fill_bytes(bytes: &[u8], nbytes: usize) -> Vec<u8> {
    let std_dev = std_deviation(bytes);
    let mut offset = 0;
    let mut values: Vec<u8> = Vec::new();
    let div_result = divide_by(nbytes as i32, 3).unwrap();
    if div_result.remainder == 0 {return values;}
    while div_result.remainder + offset != 3 {
        let mut rng = rand::rng();
        let rand_byte = bytes.choose(&mut rng).unwrap_or(&0);
        let byte: u8 = set_and_clamp(*rand_byte, std_dev);
        values.push(byte);
        offset += 1;
    }
    values
}

fn divide_by(ntr: i32, dtr: i32) -> Result<Division<i32>, String> {
    if dtr == 0 {
        return Err(String::from("Division by zero"))
    }
    Ok(Division{quotient: ntr/dtr, remainder: ntr%dtr})
}

fn std_deviation(set: &[u8]) -> u8 {
    let width = set.len() as f64;
    let mean = set.iter().copied().map(f64::from).sum::<f64>() / width;
    let distances: Vec<f64> = set.iter().map(|&val| {
        (val as f64 - mean).powf(2.0)
    }).collect();
    (distances.iter().sum::<f64>() / width).sqrt().floor() as u8
}

fn set_and_clamp(byte: u8, std_dev: u8) -> u8 {
    if byte >= u8::MAX/2 {
        byte.saturating_sub(std_dev)
    } else {
        byte.saturating_add(std_dev)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

use crate::{colors, fill_bytes};

    #[test]
    fn test_byte_logic() {
        let bytes = vec![];
        let nbytes = 1046;
        assert_eq!(fill_bytes(&bytes, nbytes).len(), 1)
    }

    #[test]
    fn colors_output() {
        let bytes = vec![
            12, 255, 34, 87, 190, 3, 144, 61, 222, 18,
            99, 201, 76, 45, 167, 8, 231, 54, 119, 250,
            31, 173, 92, 6, 214, 136, 58, 199, 24, 111,
            240, 69, 153, 42, 188, 15, 225, 81, 130, 247,
        ];
        let nbytes = bytes.len();
        colors(bytes, nbytes);
    }
}