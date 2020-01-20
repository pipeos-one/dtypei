extern crate wasm_bindgen;

#[macro_use]
extern crate dtypei_derive;

use wasm_bindgen::prelude::*;
use dtypei;

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sum(n1: u16, n2: u16) -> u16 {
    n1 + n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sub(n1: u16, n2: u16) -> u16 {
    n1 - n2
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn mul(n1: u16, n2: u16) -> u16 {
    n1 * n2
}

#[dtypei_attr()]
pub fn typedinterface() -> Vec<dtypei::Typei> {

}

#[wasm_bindgen]
pub fn typedinterface_js() -> JsValue {
    let interf: Vec<dtypei::Typei> = typedinterface();
    JsValue::from_serde(&interf).unwrap()
}
