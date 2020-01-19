extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

use syn::{parse_macro_input, parse_quote, ItemFn};

use dtypei;

mod function;

#[proc_macro_attribute]
pub fn dtypei_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let inputstr = &input.to_string();

    if inputstr.contains("wasm_bindgen") {
        return input;
    }

    // eprintln!("---- dtypei_attr input: {:?}", &input.to_string());

    let mut functiont = parse_macro_input!(input as ItemFn);

    let dtype_func = function::parse_function(&functiont);

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
