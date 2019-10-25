extern crate image;

use rand::prelude::*;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

mod Raytracer {
use num::clamp;

    pub fn raytrace(height: u16,
                    width: u16,
                    trace_depth: u8,
                    img: image::ImageBuffer<image::Pixel, image::Pixel::Subpixel>) -> image::ImageBuffer<image::Pixel, image::Pixel::Subpixel> {

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = clamp::<u64>
                ((255 * (x & y) != 0) as u64, 0, 255) as u8;
            let g = clamp::<u64>
                ((x ^ y) as u64             , 0, 255) as u8;
            let b = clamp::<u64>
                ((x | y) as u64             , 0, 255) as u8;
            *pixel = image::Rgb([r, g, b]);
        }

        return img;
    }

    /*
    fn calculate_radiance(scene : Group, ray : Ray, trace_depth : u8, current_depth : u8) {
        // TODO
    }

    fn log_render_time(thread, timestamp : u32) {
        // TODO
    }
    */
}
