#![allow(unused)]

mod header;

use crate::header::*;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use rand::{Rng, RngExt, seq::IndexedRandom};

fn main() -> Result<(), FileError> {
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
    colors(bytes, nbytes);

    Ok(())
}

//Quotient = #pixels we will have
fn colors(mut bytes: Vec<u8>, nbytes: usize) -> Result<(), String> {
    //Careful, result from divide_by never used
    let mut fill_in_bytes = fill_bytes(&bytes, nbytes)?;
    if !fill_in_bytes.is_empty() {
        bytes.append(&mut fill_in_bytes);
    }
    let colors = set_colors(&bytes);
    dbg!(&colors);
    Ok(())
}

fn set_colors(bytes: &[u8]) -> Vec<Color> {
    bytes.as_chunks::<3>().0.iter()
    .map(|&[r,g,b]| Color(r,g,b))
    .collect()
}

fn fill_bytes(bytes: &[u8], nbytes: usize) -> Result<Vec<u8>, String> {
    let std_dev = std_deviation(bytes);
    let mut offset = 0;
    let mut values: Vec<u8> = Vec::new();
    let div_result = divide_by(nbytes as i32, 3)?;
    if div_result.remainder == 0 {return Ok(values);}
    while div_result.remainder + offset != 3 {
        let mut rng = rand::rng();
        let rand_byte = bytes.choose(&mut rng).unwrap_or(&0);
        let byte: u8 = set_and_clamp(*rand_byte, std_dev);
        values.push(byte);
        offset += 1;
    }
    Ok(values)
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
        (byte - std_dev) % u8::MAX
    } else {
        (byte + std_dev) % u8::MAX
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
        assert_eq!(fill_bytes(&bytes, nbytes).unwrap().len(), 1)
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