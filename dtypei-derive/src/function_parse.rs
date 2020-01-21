extern crate proc_macro;

use quote::quote;
use syn;

use syn::{FnArg, ItemFn, PatType, ReturnType, Receiver};
use syn::punctuated::{Punctuated};
use syn::token::Comma;

use crate::utils;

pub fn parse(function: &ItemFn) -> proc_macro2::TokenStream {
    let ident = &function.sig.ident;
    let output = &function.sig.output;
    let fn_token = &function.sig.fn_token;

    let nametoken = utils::quotify(quote!(#ident));
    let fntypetoken = utils::quotify(quote!(#fn_token));

    let dtype_inputs = parse_inputs(&function.sig.inputs);

    let dtype_outputs = match output {
        ReturnType::Default => quote!(-> []),
        ReturnType::Type(_,  out) => {
            let typetoken = utils::quotify(quote!(#out));
            quote!(
                vec![
                    dtypei::SubTypes {
                        name: String::from(#typetoken),
                        label: String::from(#nametoken),
                        dimensions: Vec::new(),
                    }
                ]
            )
        }
    };

    let dtype_func = quote!(
        dtypei::Typei {
            name: String::from(#nametoken),
            type_choice: String::from(#fntypetoken),
            inputs: vec![
                #(#dtype_inputs),*
            ],
            outputs: #dtype_outputs,
        }
    );

    dtype_func
}

fn parse_inputs(inputs: &Punctuated<FnArg, Comma>) -> Vec<proc_macro2::TokenStream> {
    let mut dtype_inputs = Vec::new();

    for (_, input) in inputs.iter().enumerate() {
        match input {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                let tpattoken = utils::quotify(quote!(#pat));
                let tytoken = utils::quotify(quote!(#ty));

                dtype_inputs.push(quote!(
                    dtypei::SubTypes {
                        name: String::from(#tytoken),
                        label: String::from(#tpattoken),
                        dimensions: Vec::new(),
                    }
                ));
            }
            FnArg::Receiver(Receiver {..}) => {
                // TODO
            }
        }
    }

    dtype_inputs
}
