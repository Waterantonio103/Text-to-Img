use std::ops::{Div, Rem};

#[derive(Debug)]
pub enum FileError {
    Extension,
    Read(std::io::Error),
    InsufficientLength,
}

pub struct Division<T: Div<Output = T> + Rem<Output = T>> {
    pub quotient: T,
    pub remainder: T,
}

pub struct Color(pub u8,pub u8,pub u8);

pub struct Position(pub i32,pub i32);

pub struct Pixel {
    pub color: Color,
    pub position: Position,
}
