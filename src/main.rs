extern crate image;
extern crate num;

mod raytracer;

use raytracer::raytracer::raytracer::Raytracer;

use image::{ImageBuffer, Rgba};

use core::f32::consts::PI;
use num::clamp;
use std::num::ParseIntError;
use std::string::String;
use std::vec::Vec;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const SAMPLES: u16 = 20;
const TRACE_DEPTH: u32 = 10;
const FOV: f32 = PI / 2.0; // 90 degrees
const ORIGIN: [u8; 3] = [0, 0, 0];

fn main() {
    // get filename and quality multiplier from commandline arguments
    let (filename, res_multiplier) = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            print_help();
            std::process::exit(1);
        }
    };

    let width = clamp((WIDTH as f32 * res_multiplier) as u32, 0, 8126);
    let height = clamp((HEIGHT as f32 * res_multiplier) as u32, 0, 8126);
    let trace_depth = (TRACE_DEPTH as f32 * res_multiplier) as u32;

    //let camera = image::camera::new(width, height, FOV);

    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let tracer: Raytracer = Raytracer { width, height, trace_depth };
    img = tracer.raytrace(img);

    match img.save(filename) {
        Ok(v) => v,
        Err(e) => println!("Couldn't save image file. {}", e),
    };
}

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
        Some(argv) => Ok((String::from(&argv[0]), argv[1].parse().unwrap())),
        None => Err(ParseArgsError::NotEnoughArguments),
    }
}

//fn init_scene() -> Group<'static, Shape> {
//    todo!();
//}

fn print_help() {
    println!("rgn <filename> <quality>");
}
