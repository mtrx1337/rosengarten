use camera;
extern crate image;

use rand::prelude::*;

use tools::matrix;
use tools::vector;

pub fn raytrace(height : i16,
                width : i16,
                trace_depth : u8,
                img : image::ImageBuffer) -> vec![u8; 3 as usize]{
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = clamp((255 * (x & y) != 0) as u64, 0, 255) as u8;
        let g = clamp((x ^ y) as u64             , 0, 255) as u8;
        let b = clamp((x | y) as u64             , 0, 255) as u8;
        *pixel = image::Rgb([r, g, b]);
    }
}

fn calculate_radiance(scene : Group, ray : Ray, trace_depth : u8, current_depth : u8) {
    // TODO
}

fn log_render_time(thread, timestamp : u32) {
    // TODO
}
