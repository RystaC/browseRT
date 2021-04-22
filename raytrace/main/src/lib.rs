use wasm_bindgen::prelude::*;

use random::Random;

#[wasm_bindgen]
pub fn take_number() -> u64 {
    let mut rand = Random::new();
    rand.next_1byte()
}