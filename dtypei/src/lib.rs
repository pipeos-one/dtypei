extern crate wasm_bindgen;

// use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TypeRelation { Has, Link, Bytes }

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LangChoices { Solidity, JavaScript, Python }

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TypeChoices {
    BaseType,
    PayableFunction,
    StateFunction,
    ViewFunction,
    PureFunction,
    Event,
    MappingType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubTypes {
    pub name: String,
    pub label: String,
    // relation: TypeRelation,
    pub dimensions: Vec<i32>, // String[],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Typei {
    // lang: LangChoices,  // required
    // pub type_choice: TypeChoices,  // required
    // contractAddress: address,
    // source: bytes32,
    pub name: String,  // required
    // required types
    pub inputs: Vec<SubTypes>,  // required
    // optional types
    // optionals: SubTypes[],
    // function outputs
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
        // self.interface.into_iter().collect()
    }

    pub fn add(&mut self, export: Typei) {
        self.interface.push(export);
    }
}

// static mut INTERFACES: Vec<Typei> = Vec::new();
//
// pub fn get() -> Vec<Typei> {
//     unsafe {
//         INTERFACES.to_vec()
//     }
// }
//
// pub fn add(export: Typei) {
//     unsafe {
//         INTERFACES.push(export);
//     }
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
