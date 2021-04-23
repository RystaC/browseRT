use wasm_bindgen::prelude::*;

use random::Random;

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
pub fn noise_image(width: usize, height: usize) -> Vec<u8> {
    let size = width * height;
    let mut rand = Random::new();
    let mut color = Vec::<Color>::with_capacity(size);
    for _ in 0..size { color.push(Color { r: rand.next_1byte(), g: rand.next_1byte(), b: rand.next_1byte(), a: 255 })};
    let mut result = Vec::with_capacity(size * 4);
    for c in color {
        result.push(c.r);
        result.push(c.g);
        result.push(c.b);
        result.push(c.a);
    }
    result
}