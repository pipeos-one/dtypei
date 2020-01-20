use wasm_bindgen::prelude::*;
use dtypei;

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sum_u8(n1: u8, n2: u8) -> u8 {
    n1 + n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sub_u8(n1: u8, n2: u8) -> u8 {
    n1 - n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn mul_u8(n1: u8, n2: u8) -> u8 {
    n1 * n2
}
