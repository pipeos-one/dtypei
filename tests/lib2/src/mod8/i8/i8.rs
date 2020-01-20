use wasm_bindgen::prelude::*;
use dtypei;

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sum_i8(n1: i8, n2: i8) -> i8 {
    n1 + n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sub_i8(n1: i8, n2: i8) -> i8 {
    n1 - n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn mul_i8(n1: i8, n2: i8) -> i8 {
    n1 * n2
}
