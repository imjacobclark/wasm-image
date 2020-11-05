use std::io::{Cursor, Read, Seek, SeekFrom};
use std::panic;

use image::ImageFormat;
use image::DynamicImage;

use wasm_bindgen::prelude::*;

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let img = match image::load_from_memory(_array) {
        Ok(img) => img,
        Err(error) => {
            panic!("{:?}", error);
        }
    };

    return img;
}

fn get_image_as_array(image: DynamicImage) -> Vec<u8> {
    let mut c = Cursor::new(Vec::new());

    match image.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            panic!("{:?}", error);
        }
    };

    c.seek(SeekFrom::Start(0)).unwrap();

    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();

    return out;
}

#[wasm_bindgen]
pub fn rotate(_array: &[u8], _deg: u16) -> Vec<u8> {
    let mut img = load_image_from_array(_array);

    img = match _deg {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    };

    return get_image_as_array(img);
}