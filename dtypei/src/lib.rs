extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubTypes {
    pub name: String,
    pub label: String,
    pub dimensions: Vec<i32>, // String[],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Typei {
    pub type_choice: String,  // TODO: enum
    pub name: String,
    pub inputs: Vec<SubTypes>,
    pub outputs: Vec<SubTypes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleInterface {
    pub interface: Vec<Typei>,
}

impl ModuleInterface {
    pub fn new() -> ModuleInterface {
        ModuleInterface {
            interface: Vec::new(),
        }
    }

    pub fn get(&self) -> Vec<Typei> {
        self.interface.to_vec()
    }

    pub fn add(&mut self, export: Typei) {
        self.interface.push(export);
    }
}

static mut INTERFACE_STR: Vec<String> = Vec::new();
pub fn istrget() -> Vec<String> {
    unsafe {
        INTERFACE_STR.to_vec()
    }
}
pub fn istradd(export: String) {
    unsafe {
        INTERFACE_STR.push(export);
    }
}
