use num::clamp;
use std::string::String;
use std::vec::Vec;

pub struct Image {
    pub width:    u16,
    pub height:   u16,
    pub gamma:    f32,
    pub filename: String,
    pub image:    Vec<u8>,
}

// array with chunks of 4 elements. r/g/b/a
pub fn set_pixel(pixel_address: u64, mut color: [u8; 4], img: &mut Image) {
    color = gamma_correct_color(color, img.gamma);
    // red
    img.image[pixel_address as usize]       = color[0];
    // green
    img.image[(pixel_address + 1) as usize] = color[1];
    // blue
    img.image[(pixel_address + 2) as usize] = color[2];
    // alpha
    img.image[(pixel_address + 3) as usize] = color[3];
}

pub fn gamma_correct_color(mut color: [u8; 4], gamma: f32) -> [u8; 4] {
    // value ^ (1/gamma)
    for col in color.iter_mut() {
        *col = clamp(col.pow((1.0 as f32 / gamma) as u32), 0, 255);
    }
    return color;
}
