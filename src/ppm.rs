use crate::image::Image;
use std::string::String;
use std::fs::File;
use std::io::prelude::*;

pub fn write_rgb_ppm(img: &mut Image) {
    let mut file = match File::create(&img.filename) {
        Ok(f) => f,
        Err(e) => panic!("Error when trying to create the file \"{}\", error: \"{}\"", img.filename, e),
    };

    /* example header:
     * P3
     * 1920 1080
     * 255
     */
    let header = String::from(
        format!("P3\n{} {}\n255\n", &img.width, &img.height));

    match file.write(header.as_bytes()) {
        Ok(f) => f,
        Err(e) => { println!("Error when trying to write to the file \"{}\", error: \"{}\"", img.filename, e); std::process::exit(0) },
    };

    let mut pixel_address: u64;

    // write actual image data
    for y in 0..img.height {
        for x in 0..img.width {
            pixel_address = (4 * img.width * y  + 4 * x) as u64;
            match file.write(&[img.image[pixel_address as usize]]) {
                Ok(f) => f,
                Err(e) => panic!("Error when trying to write to the file \"{}\", error: \"{}\"", img.filename, e),
            };
        }
    }
}
