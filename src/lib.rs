// #![feature(stdsimd)]

pub mod array;
pub mod images;
pub mod math;
pub mod crypto;
pub mod byte;
pub mod json;
pub mod string;

use wasm_bindgen::prelude::*;

use console_error_panic_hook::set_once;

#[wasm_bindgen(start)]
pub fn main() {
    set_once(); // Set panic hook for better error messages
}