use std::ops::{Div, Rem};
use std::fmt::{self, Debug, Display};

pub enum FileError {
    InvalidExtension(String),
    Read(std::io::Error),
    InsufficientLength(String),
}

impl From<std::io::Error> for FileError {
    fn from(value: std::io::Error) -> Self {
        FileError::Read(value)
    }
}

impl Debug for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::InvalidExtension(e) => {write!(f,"{}",e)},
            FileError::Read(e) => {write!(f,"{}", e)},
            FileError::InsufficientLength(e) => {write!(f,"{}", e)},
        }
    }
}

pub struct Division<T: Div<Output = T> + Rem<Output = T>> {
    pub quotient: T,
    pub remainder: T,
}

#[derive(Debug)]
pub struct Color(pub u8,pub u8,pub u8);

pub struct Position(pub i32,pub i32);

pub struct Pixel {
    pub color: Color,
    pub position: Position,
}
