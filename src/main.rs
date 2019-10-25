extern crate num;
extern crate image;

mod raytracer;

use std::string::String;
use std::vec::Vec;
use num::clamp;
use std::num::ParseIntError;
use core::f32::consts::PI;

const WIDTH : u16 = 1920;
const HEIGHT : u16 = 1080;
const SAMPLES : u16 = 20;
const TRACE_DEPTH : u16 = 10;
const FOV : f32 = PI / 2.0; // 90 degrees
const ORIGIN : Vec<u8> = vec!(0,0,0);

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

pub fn parse_args() -> Result<(String, f32), ParseArgsError> {
    let argv = std::env::args().collect::<Vec<_>>();
    match argv.get(1..3) {
        Some(argv) => Ok((String::from(&argv[0]), argv[1].parse()?)),
        None => Err(ParseArgsError::NotEnoughArguments)
    }
}

fn main() {
    let (filename, res_multiplier) = match parse_args() {
        Ok(v) => v,
        Err(e) => { println!("{:?}", e); print_help(); return; }
    };

    let width = clamp::<u32>
        ((WIDTH as f32 * res_multiplier).into(), 0, 8126);
    let height = clamp::<u32>
        ((HEIGHT as f32 * res_multiplier).into(), 0, 8126);

    //let camera = image::camera::new(width, height, FOV);

    let mut img = image::ImageBuffer::new(width as u32, height as u32);

    let scene : Group = init_scene();

    match img.save(filename) {
        Ok(v) => v,
        Err(e) => println!("Couldn't save image file. {}", e),
    };
}

fn init_scene() -> Group {
    // TODO
}

fn print_help() {
    println!("rgn <filename> <width> <height>");
}
