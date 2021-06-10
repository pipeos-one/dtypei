extern crate wasm_bindgen;

// #[macro_use]
// extern crate lazy_static;

// use std::cell::RefCell;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleInterfaceString {
    pub interface: Vec<String>,
}

impl ModuleInterfaceString {
    pub fn new() -> ModuleInterfaceString {
        ModuleInterfaceString {
            interface: Vec::new(),
        }
    }

    pub fn get(&self) -> Vec<String> {
        self.interface.to_vec()
    }

    pub fn add(&mut self, export: String) {
        self.interface.push(export);
    }
}

// thread_local! {
//     // pub static INTERFACE_STR: RefCell<ModuleInterfaceString> = RefCell::new();
//
//     pub static INTERFACE_STR2: ModuleInterfaceString = ModuleInterfaceString::new();
//
//     // INTERFACE_STR2::add(String::from("hello"));
//
//     // #[allow(unused)]
//     // static BAR: RefCell<f32> = RefCell::new(1.0);
// }

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

// lazy_static! {
//     pub static ref INTERFACE_STR: ModuleInterfaceString = {
//         let mut m = ModuleInterfaceString::new();
//         m
//     };
// }
