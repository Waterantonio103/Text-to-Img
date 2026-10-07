#![allow(unused)]

use std::fs;
use std::ops::Rem;
use std::path::Path;

fn main() {
    let path = Path::new("src/rand.txt");

    let rdr = fs::read(path);

    if let Ok(bytes) = rdr {
        for byte in bytes {
            println!("{byte:b}");
            let last_4 = byte << 4;
            println!("last: {last_4:b}");
            let first_4 = byte >> 4;
            println!("first: {first_4:b}");
        }
    }

}

fn range_wrap<T: Rem<Output = T>>(x: T, max: T) -> T {
    x % max
}

