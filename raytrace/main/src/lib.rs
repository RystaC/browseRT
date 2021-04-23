use wasm_bindgen::prelude::*;

use random::Random;

#[wasm_bindgen]
pub fn noise_image(width: usize, height: usize) -> Vec<u8> {
    let size = width * height;
    let mut rand = Random::new();
    let mut result = Vec::with_capacity(size * 3);
    for _ in 0..(size * 3) { result.push(rand.next_1byte()); };
    result
}