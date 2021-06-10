extern crate wasm_bindgen;

#[macro_use]
extern crate dtypei_derive;

use wasm_bindgen::prelude::*;

pub mod amod;

// #[wasm_bindgen]
// #[dtypei_attr()]
// pub fn summ(n1: u8, n2: u8) -> u8 {
//     n1 + n2
// }

#[wasm_bindgen(start)]
pub fn main() {
    println!("Hello, world!");
}
