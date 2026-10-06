#![allow(unused)]

use std::fs;
use std::ops::Rem;
use std::path::Path;

fn main() {
    let path = Path::new("src/rand.txt");

    let rdr = fs::read(path);

    if let Ok(bytes) = rdr {
        
    }

    let num: i32 = 256;
    let res = range_wrap(num, 256);
    println!("{res}");
}

fn range_wrap<T: Rem<Output = T>>(x: T, max: T) -> T {
    x % max
}

