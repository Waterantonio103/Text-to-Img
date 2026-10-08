#![allow(unused)]

mod header;

use crate::header::*;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() -> Result<(), FileError> {
    let file = "src/rand.txt";
    let path = Path::new(file);
    let target_extension = OsStr::new("txt");

    if let Some(extension) = path.extension() {
        if extension != target_extension {return Err(FileError::InvalidExtension(format!("Invalid file extension {{ Current -> '.{}' Required -> '.{}' }}", extension.display(), target_extension.display())));}
    }

    let bytes = fs::read(path)?;

    if bytes.len() < 3 {
        return Err(FileError::InsufficientLength(format!("Insufficient Bytes {{ Current -> '{}' Minimum -> '{}' }}", bytes.len(), 3)));
    }
    let nbytes = bytes.len();
    colors(&bytes, nbytes as i32);

    Ok(())
}

//Quotient = #pixels we will have
fn colors(bytes: &[u8], nbytes: i32) {
    if let Ok(div_result) = divide_by(nbytes, 3) {

    }
}

fn divide_by(numerator: i32, denominator: i32) -> Result<Division<i32>, String> {
    if denominator == 0 {
        return Err(String::from("Division by zero"))
    }
    Ok(Division{quotient: numerator/denominator, remainder: numerator%denominator})
}

fn set_colors(bytes: &[u8]) -> Vec<Color> {
    bytes.as_chunks::<3>().0.iter()
        .map(|&[r,g,b]| Color(r,g,b))
        .collect()
}
