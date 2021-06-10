extern crate wasm_bindgen;

#[macro_use]
extern crate dtypei_derive;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use dtypei;

#[wasm_bindgen]
#[dtypei_enum()]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Material {
    Wood,
    Metal,
    Plastic,
}

// #[wasm_bindgen]
// #[dtypei_enum()]
// #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
// pub enum Material {
//     Custom(String),
//     Mix(i32, i32, i32),
//     Coop {frame: MaterialBase, body: MaterialBase},
// }

#[wasm_bindgen]
#[dtypei_enum()]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Color {
    Blue,
    Black,
    Brown,
}

#[wasm_bindgen]
#[dtypei_struct()]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WoodFinish {
    pub sanded: bool,
    pub varnished: bool,
    pub polished: bool,
}

#[wasm_bindgen]
#[dtypei_struct()]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub private_id: u64,
    pub legs: u8,
    // pub design_name: String,
    pub color: Color,
    pub material: Material,
    pub finish: WoodFinish,
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn new_table() -> Table {
    Table {
        private_id: 0,
        legs: 4,
        // design_name: String::from("name"),
        color: Color::Brown,
        material: Material::Wood,
        finish: WoodFinish {
            sanded: false,
            varnished: false,
            polished: false,
        }
    }
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn polish(mut table: Table) -> Table {
    table.finish.polished = true;
    table
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn sand(mut table: Table) -> Table {
    table.finish.sanded = true;
    table
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn varnish(mut table: Table) -> Table {
    table.finish.varnished = true;
    table
}

#[wasm_bindgen]
#[dtypei_attr()]
pub fn paint(mut table: Table, color: Color) -> Table {
    table.color = color;
    table
}

#[dtypei_attr()]
pub fn typedinterface() -> Vec<dtypei::Typei> {

}

#[wasm_bindgen]
pub fn typedinterface_js() -> JsValue {
    let interf: Vec<dtypei::Typei> = typedinterface();
    JsValue::from_serde(&interf).unwrap()
}
