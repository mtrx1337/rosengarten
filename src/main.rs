extern crate num;
extern crate image;

use raytracer::camera;
use std::string::String;
use std::vec::Vec;
use num::clamp;
use std::num::ParseIntError;
use f64::consts::PI;
use raytracer::shapes::*;

const WIDTH = 1920;
const HEIGHT = 1080;
const SAMPLES = 20;
const TRACE_DEPTH = 10;
const FOV = PI / 2; // 90 degrees
const ORIGIN = vec![0; 3 as usize];

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
    let (filename, res_multiplier) = match parse_args() {
        Ok(v) => v,
        Err(e) => { println!("{:?}", e); print_help(); return; }
    };

    let width = WIDTH * res_multiplier;
    let height = HEIGHT * res_multiplier;

    //let camera = image::camera::new(width, height, FOV);

    let mut img = image::ImageBuffer::new(width, height);

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
