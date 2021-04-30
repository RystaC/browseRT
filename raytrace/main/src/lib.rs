use wasm_bindgen::prelude::*;

use random::Random;
use vector::Color;

#[wasm_bindgen]
pub fn noise_image(width: usize, height: usize) -> Vec<u8> {
    let size = width * height;
    let mut color = Vec::<Color>::with_capacity(size);
    color.resize(size, Color::from_val(1.0, 0.0, 0.0));

    let mut buffer = Vec::<u8>::with_capacity(size * 4);
    for c in color {
        let byte = c.as_bytes(255);
        buffer.push(byte[0]);
        buffer.push(byte[1]);
        buffer.push(byte[2]);
        buffer.push(byte[3]);
    };

    buffer
}