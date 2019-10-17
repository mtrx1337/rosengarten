extern crate num;
mod ppm;
mod image;
use std::string::String;
use num::clamp;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let width : u16 = args[2].parse::<u16>().expect("Not enough arguments provided.");
    let height : u16 = args[3].parse::<u16>().expect("Not enough arguments provided.");

    let mut color : [u8; 4] = [0, 0, 0, 0];
    let mut x : u16 = 0;
    let mut y : u16 = 0;
    let mut pixel_address : u64 = 0;

    let mut img = image::Image {
        filename : String::from(&args[1]),
        width    : width,
        height   : height,
        gamma    : 2.2,
        image    : Vec::<u8>::new(),
    };

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
}
