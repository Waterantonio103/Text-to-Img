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
    let nbytes = bytes.len();

    if nbytes < 3 {
        return Err(FileError::InsufficientLength(format!("Insufficient Bytes {{ Current -> '{}' Minimum -> '{}' }}", nbytes, 3)));
    }
    colors(bytes, nbytes as i32);

    Ok(())
}

//Quotient = #pixels we will have
fn colors(mut bytes: Vec<u8>, nbytes: i32) -> Result<(), String> {
    //Careful, result from divide_by never used
    let mut fill_in_bytes = fill_bytes(nbytes)?;
    if !fill_in_bytes.is_empty() {
        bytes.append(&mut fill_in_bytes);
    }

    Ok(())
}

fn divide_by(ntr: i32, dtr: i32) -> Result<Division<i32>, String> {
    if dtr == 0 {
        return Err(String::from("Division by zero"))
    }
    Ok(Division{quotient: ntr/dtr, remainder: ntr%dtr})
}

fn set_colors(bytes: &[u8]) -> Vec<Color> {
    bytes.as_chunks::<3>().0.iter()
        .map(|&[r,g,b]| Color(r,g,b))
        .collect()
}

//fill bytes : while loop that waits until divide_by(len, 3).remainder + count == 0

fn fill_bytes(nbytes: i32) -> Result<Vec<u8>, String> {
    let mut offset = 0;
    let mut values: Vec<u8> = Vec::new();
    let div_result = divide_by(nbytes, 3)?;
    while div_result.remainder + offset != 3 {
        let byte: u8 = 0;
        values.push(byte);
        offset += 1;
    }
    Ok(values)
}
