#![allow(unused)]

use std::fs;
use std::ops::Rem;
use std::path::Path;

struct Color(u8,u8,u8);

struct Position(i32,i32);

struct Pixel {
    color: Color,
    position: Position,
}

fn main() {
    let path = Path::new("src/rand.txt");

    let rdr = fs::read(path);

    if let Ok(bytes) = rdr {
        let nbytes = bytes.len();
        if divide_by_3(nbytes as i32).1 == 0 {
            //function
        } else {
            //other function
        }
    }

}

fn range_wrap<T: Rem<Output = T>>(x: T, max: T) -> T {
    x % max
}

fn divisible_by_3(num: usize) -> bool {
    num % 3 == 0
}

fn divide_by_3(num: i32) -> (i32, i32) {
    (num/3, num%3)
}

fn select_per_color(len: i32, bytes: &Vec<u8>) {
    
}
