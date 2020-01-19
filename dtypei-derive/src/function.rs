extern crate proc_macro;

// use crate::proc_macro::TokenStream;
// use proc_macro2::Span;
use quote::quote;
use syn;

use syn::{FnArg, ItemFn, PatType, ReturnType, Receiver};
// use syn::{Attribute, Ident, Type};
use syn::punctuated::{Punctuated};
use syn::token::Comma;

// use dtypei;

pub fn parse_function(function: &ItemFn) -> proc_macro2::TokenStream {
    // eprintln!("function: {}", quote!(#function));

    // function: attrs, vis, sig, block
    // let attrs = &function.attrs;
    // let vis = &function.vis;
    // let sig = &function.sig;

    // eprintln!("vis: {}", quote!(#vis));
    // eprintln!("sig: {}", quote!(#sig));

    // sig: constness, asyncness, unsafety, abi, fn_token, ident, paren_token, inputs, variadic, output
    // let constness = &sig.constness;
    // let asyncness = &sig.asyncness;
    // let unsafety = &sig.unsafety;
    // let abi = &sig.abi;
    // let fn_token = &sig.fn_token;
    let ident = &function.sig.ident;
    // let paren_token = &sig.paren_token;
    // let variadic = &sig.variadic;
    let output = &function.sig.output;

    // eprintln!("constness: {}", quote!(#constness));
    // eprintln!("asyncness: {}", quote!(#asyncness));
    // eprintln!("unsafety: {}", quote!(#unsafety));
    // eprintln!("abi: {}", quote!(#abi));
    // eprintln!("fn_token: {}", quote!(#fn_token));
    // eprintln!("ident: {}", quote!(#ident));
    // eprintln!("paren_token: {}", quote!(#paren_token));
    // eprintln!("variadic: {}", quote!(#variadic));
    // eprintln!("output: {}", quote!(#output));

    let dtype_inputs = parse_inputs(&function.sig.inputs);

    let nametoken = quotify(quote!(#ident));

    let dtype_outputs = match output {
        ReturnType::Default => quote!(-> []),
        ReturnType::Type(_,  out) => {
            let typetoken = quotify(quote!(#out));
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
            // type_choice: dtypei::TypeChoices::PureFunction,
            inputs: vec![
                #(#dtype_inputs),*
            ],
            outputs: #dtype_outputs,
        }
    );

    // eprintln!("dtype_func: {}", dtype_func);

    dtype_func
}

fn parse_inputs(inputs: &Punctuated<FnArg, Comma>) -> Vec<proc_macro2::TokenStream> {
    let mut dtype_inputs = Vec::new();

    for (_, input) in inputs.iter().enumerate() {
        // eprintln!("input: {}", quote!(#input));

        match input {
            FnArg::Typed(PatType { pat, ty, .. }) => {
            // FnArg::Typed(PatType { pat, attrs, colon_token, ty }) => {
                // eprintln!("pat: {:?}", &pat);
                // eprintln!("pat: {}", quote!(#pat));
                // eprintln!("attrs: {}", quote!(#attrs));
                // eprintln!("colon_token: {}", quote!(#colon_token));
                // eprintln!("ty: {}", quote!(#ty));

                let tpattoken = quotify(quote!(#pat));
                let tytoken = quotify(quote!(#ty));

                dtype_inputs.push(quote!(
                    dtypei::SubTypes {
                        name: String::from(#tytoken),
                        label: String::from(#tpattoken),
                        dimensions: Vec::new(),
                    }
                ));
            }
            FnArg::Receiver(Receiver {..}) => {
            // FnArg::Receiver(Receiver {attrs, reference, mutability, self_token}) => {
                // eprintln!("attrs: {}", quote!(#attrs));
                // eprintln!("reference: {}", quote!(#reference));
                // eprintln!("mutability: {}", quote!(#mutability));
                // eprintln!("self_token: {}", quote!(#self_token));
            }
        }
    }

    dtype_inputs
}

fn quotify(token: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let typestr = format!("\"{}\"", quote!(#token));
    typestr.parse().unwrap()
}
