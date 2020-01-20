use wasm_bindgen::prelude::*;
use dtypei;

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sum_u16(n1: u16, n2: u16) -> u16 {
    n1 + n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sub_u16(n1: u16, n2: u16) -> u16 {
    n1 - n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn mul_u16(n1: u16, n2: u16) -> u16 {
    n1 * n2
}
