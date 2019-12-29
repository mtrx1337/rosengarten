pub mod raytracer {
    extern crate image;
    use image::{ImageBuffer, Rgba};
    use num::clamp;

    pub struct Raytracer {
        pub width: u32,
        pub height: u32,
        pub trace_depth: u32
    }

    impl Raytracer {
        pub fn raytrace(&self, mut img: ImageBuffer<Rgba<u8>, Vec<u8>>)
            -> ImageBuffer<Rgba<u8>, Vec<u8>> {
                for (x, y, pixel) in img.enumerate_pixels_mut() {
                    let r: u8 = clamp::<u64>((255 * (x & y) != 0) as u64, 0, 255) as u8;
                    let g: u8 = clamp::<u64>((x ^ y) as u64, 0, 255) as u8;
                    let b: u8 = clamp::<u64>((x | y) as u64, 0, 255) as u8;
                    *pixel = Rgba([r, g, b, 255]);
                }

                return img;
            }

        //fn calculate_radiance(scene : Group, ray : Ray, trace_depth : u8, current_depth : u8) {
        //    todo!();
        //}

        //fn log_render_time(thread, timestamp : u32) {
        //    todo!();
        //}
    }
}
