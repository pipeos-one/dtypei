#![allow(unused_variables)]

use std::collections::HashMap;
use js_sys::{Function, Object, Reflect, WebAssembly, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
// use serde::{Deserialize, Serialize};
use serde_json;

use reqwest;
use dtypei;

use shorthand::ShortHand;
use std::cell::RefCell;
use std::thread_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

thread_local! {
    static WRAPPED_MODULES: RefCell<HashMap<String, WrappedModule>> = RefCell::new(HashMap::new());
}

type ModuleInterface = Vec<dtypei::Typei>;

#[derive(ShortHand)]
pub struct WrappedModule {
    instance: WebAssembly::Instance,
    interface: ModuleInterface,
}

async fn get_module_buffer(url: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    Ok(resp.to_vec())
}

async fn load_module_async(url: &String) -> Result<WebAssembly::Instance, JsValue> {
    let buffer = get_module_buffer(url).await.unwrap();

    let a = JsFuture::from(WebAssembly::instantiate_buffer(&buffer, &Object::new())).await?;

    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    Ok(b)
}

fn run_func(b: &WebAssembly::Instance, fname: &String, inputs: &serde_json::Value) -> Result<serde_json::Value, JsValue> {
    let inputs_vec = inputs.as_array().unwrap();
    let c = b.exports();
    console_log!("c: {:?}", c);
    console_log!("callF : {:?}", fname);

    let finstance = Reflect::get(c.as_ref(), &fname.into())?
        .dyn_into::<Function>()
        .expect(&format!("{} wasn't a function", &fname).to_string());

    console_log!("finstance = {:?}", finstance);
    let length: usize = inputs_vec.len();
    console_log!("Inputs lenght: {}", length);
    let result = match &length {
        0 => finstance.call0(&JsValue::undefined()),
        1 => {
            let input = &inputs_vec[0];
            let inputjs: i32 = serde_json::from_value(input.to_owned()).unwrap();
            finstance.call1(&JsValue::undefined(), &inputjs.into())
        },
        2 => {
            let input1 = &inputs_vec[0];
            let input2 = &inputs_vec[1];
            let inputjs1: i32 = serde_json::from_value(input1.to_owned()).unwrap();
            let inputjs2: i32 = serde_json::from_value(input2.to_owned()).unwrap();
            console_log!("Inputs {:?}, {:?}", inputjs1, inputjs2);
            finstance.call2(&JsValue::undefined(), &inputjs1.into(), &inputjs2.into())
        },
        3 => {
            let input1 = &inputs_vec[0];
            let input2 = &inputs_vec[1];
            let input3 = &inputs_vec[2];
            let inputjs1: i32 = serde_json::from_value(input1.to_owned()).unwrap();
            let inputjs2: i32 = serde_json::from_value(input2.to_owned()).unwrap();
            let inputjs3: i32 = serde_json::from_value(input3.to_owned()).unwrap();
            finstance.call3(&JsValue::undefined(), &inputjs1.into(), &inputjs2.into(), &inputjs3.into())
        },
        _ => panic!("Function has more than 3 args"),
    }?;

    console_log!("result = {:?}", result);
    // TODO how to do this dynamically? extend dyn_into?

    // let out: i32 = result.into_serde().unwrap();
    // console_log!("out: {:?} = {:?}", &fdata.outputs[0].name, out);

    // mutable_inputs[&fdata.outputs[0].name] = out.into();
    // Ok(mutable_inputs)
    Ok(result.into_serde().unwrap())
}

fn deserialize_interface(data: String) -> serde_json::Result<ModuleInterface> {
    let parsed: ModuleInterface = serde_json::from_str(&data)?;
    // console_log!("parsed = {:?}", parsed);
    Ok(parsed)
}

fn deserialize_inputs(data: String) -> serde_json::Result<serde_json::Value> {
    let parsed: serde_json::Value = serde_json::from_str(&data)?;
    // console_log!("deserialize_inputs = {:?}", parsed);
    Ok(parsed)
}

#[wasm_bindgen]
pub fn run(module_url: String, function_name: String, inputs: String) -> Result<JsValue, JsValue> {
    let input_parsed = deserialize_inputs(inputs).expect("Graph data could not be deserialized.");

    WRAPPED_MODULES.with(|modules_map| -> Result<JsValue, JsValue> {
        let wrapped_modules = modules_map.borrow();
        let wrapped_module = wrapped_modules.get(&module_url).unwrap();
        let outputs = run_func(&wrapped_module.instance, &function_name, &input_parsed).unwrap();

        console_log!("---- outputs: {:?}", &outputs);
        let result: String = outputs.to_string();

        Ok(JsValue::from(result))
    })
}

#[wasm_bindgen]
pub fn init(module_url: String, module_interface: String) -> Promise {
    let parsed_interface = deserialize_interface(module_interface).expect("Graph data could not be deserialized.");

    future_to_promise(async move {
        let b = load_module_async(&module_url).await;
        let loaded_module = match b {
            Ok(m) => m,
            Err(m) => {
                // let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;
                // console_log!("Module loading error: {}", m.description());
                // console_log!("Module loading error: {}", m.source().unwrap());
                // console_log!("Module loading error: {}", m.backtrace());
                // &b.unwrap_throw();
                return Err(JsValue::from(String::from("Module loading error")));
            }
        };
        console_log!("Module : {:?}", loaded_module);
        let c = loaded_module.exports();
        console_log!("Module exports : {:?}", Reflect::own_keys(c.as_ref()).unwrap());

        WRAPPED_MODULES.with(|modules_map| {
            modules_map.borrow_mut().insert(
                module_url.to_owned(),
                WrappedModule {
                    instance: loaded_module.to_owned(),
                    interface: parsed_interface.to_owned(),
                },
            );
            console_log!("Contains: {:?}", modules_map.borrow_mut().contains_key(&module_url));
        });

        Ok(JsValue::from(true))
    })
}
