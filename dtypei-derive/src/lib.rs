extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

use syn::{parse_macro_input, parse_quote, ItemFn, ItemStruct, ItemEnum};

use dtypei;

mod utils;
mod fields_parse;
mod function_parse;
mod structure_parse;
mod enum_parse;

#[proc_macro_attribute]
pub fn dtypei_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let inputstr = &input.to_string();

    if inputstr.contains("wasm_bindgen") {
        return input;
    }

    // eprintln!("---- dtypei_attr input: {:?}", &input.to_string());

    let mut functiont = parse_macro_input!(input as ItemFn);

    let dtype_func = function_parse::parse(&functiont);

    // eprintln!("dtype_func: {}", dtype_func);

    dtypei::istradd(format!("{}", dtype_func));

    if inputstr.contains("typedinterface") {
        let interf = dtypei::istrget().into_iter().map(|item| {
            let tokenized: proc_macro2::TokenStream = item.parse().unwrap();
            tokenized
        });

        functiont.block = Box::new(parse_quote!({
            vec![
                #(#interf),*
            ]
        }));

        return TokenStream::from(quote!(#functiont));
    }

    TokenStream::from(quote!(#functiont))
}

#[proc_macro_attribute]
pub fn dtypei_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let inputstr = &input.to_string();

    // eprintln!("---- dtypei_struct input: {:?}", inputstr);

    let structt = parse_macro_input!(input as ItemStruct);

    // eprintln!("structt: {}", quote!(#structt));

    let dtype_struct = structure_parse::parse(&structt);

    // eprintln!("dtype_struct: {}", dtype_struct);

    dtypei::istradd(format!("{}", dtype_struct));

    TokenStream::from(quote!(#structt))

}

#[proc_macro_attribute]
pub fn dtypei_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let inputstr = &input.to_string();

    // eprintln!("---- dtypei_enum input: {:?}", inputstr);

    let enumt = parse_macro_input!(input as ItemEnum);

    // eprintln!("enumt: {}", quote!(#enumt));

    let dtype_enum = enum_parse::parse(&enumt);

    // eprintln!("dtype_enum: {}", dtype_enum);

    dtypei::istradd(format!("{}", dtype_enum));

    TokenStream::from(quote!(#enumt))
}
