extern crate num;
extern crate image;

use std::string::String;
use std::vec::Vec;
use num::clamp;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseArgsError {
    ParseInt(ParseIntError),
    NotEnoughArguments,
}

impl From<ParseIntError> for ParseArgsError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

pub fn parse_args() -> Result<(String, u32, u32), ParseArgsError> {
    let argv = std::env::args().collect::<Vec<_>>();
    match argv.get(1..4) {
        Some(argv) => Ok((String::from(&argv[0]), argv[1].parse()?, argv[2].parse()?)),
        None => Err(ParseArgsError::NotEnoughArguments)
    }
}

fn main() {
    let (filename, width, height) = match parse_args() {
        Ok(v) => v,
        Err(e) => { println!("{:?}", e); print_help(); return; }
    };

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = clamp((255 * (x & y) != 0) as u64, 0, 255) as u8;
        let g = clamp((x ^ y) as u64             , 0, 255) as u8;
        let b = clamp((x | y) as u64             , 0, 255) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    match img.save(filename) {
        Ok(v) => v,
        Err(e) => println!("Couldn't save image file. {}", e),
    };
}

fn print_help() {
    println!("rgn <filename> <width> <height>");
}
