use crate::image::Image;
use std::fs::File;

pub fn write_rgb_ppm(img : Image) {
    let out_fd = File::open(img.filename);
}
