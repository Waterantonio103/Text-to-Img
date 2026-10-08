#![allow(unused)]

mod header;

use crate::header::*;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() -> Result<(), FileError> {
    let path = Path::new("src/rand.txt");

    if let Some(extension) = path.extension() {
        if extension != OsStr::new("txt") {return Err(FileError::Extension);}
    }

    let rdr = fs::read(path);

    match rdr {
        Ok(bytes) => {
            if bytes.is_empty() {
                return Err(FileError::InsufficientLength);
            }
            let nbytes = bytes.len();
            colors(&bytes, nbytes as i32);
        },
        Err(e) => {return Err(FileError::Read(e))}
    }

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
