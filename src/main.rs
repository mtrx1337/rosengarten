extern crate num;

mod ppm;
mod image;

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

pub fn parse_args() -> Result<(String, u16, u16), ParseArgsError> {
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

    let mut img = image::Image {
        filename: filename,
        width: width,
        height: height,
        gamma: 2.2,
        image: vec![0; (width * height * 4) as usize]
    };

    let mut color: [u8; 4] = [0, 0, 0, 0];
    let mut pixel_address: u64;

    for y in 0..height {
        for x in 0..width {
            pixel_address = (4 * width * y + 4 * x) as u64;

            color[0] = clamp((255 * !(x & y)) as u64, 0, 255) as u8;
            color[1] = clamp((x ^ y) as u64         , 0, 255) as u8;
            color[2] = clamp((x | y) as u64         , 0, 255) as u8;
            color[3] = clamp(255                    , 0, 255);

            image::set_pixel(pixel_address, color, &mut img);
        }
    }

    ppm::write_rgb_ppm(&mut img);
}

fn print_help() {
    println!("rgn <filename> <width> <height>");
}
