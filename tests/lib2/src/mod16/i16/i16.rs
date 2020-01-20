use wasm_bindgen::prelude::*;
use dtypei;

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sum_i16(n1: i16, n2: i16) -> i16 {
    n1 + n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sub_i16(n1: i16, n2: i16) -> i16 {
    n1 - n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn mul_i16(n1: i16, n2: i16) -> i16 {
    n1 * n2
}
