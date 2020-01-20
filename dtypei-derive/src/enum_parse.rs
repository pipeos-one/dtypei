extern crate proc_macro;

use quote::quote;
use syn::{ItemEnum, Variant};
use syn::punctuated::{Punctuated};
use syn::token::Comma;

use crate::utils;
use crate::fields_parse;

pub fn parse(token: &ItemEnum) -> proc_macro2::TokenStream {
    // function: attrs, vis, enum_token, ident, generics, brace_token, variants

    // let attrs = &token.attrs;
    // let vis = &token.vis;
    // let generics = &token.generics;
    // let brace_token = &token.brace_token;

    // eprintln!("vis: {}", quote!(#vis));
    // eprintln!("attrs: {}", quote!(#attrs));
    // eprintln!("enum_token: {}", quote!(#enum_token));
    // eprintln!("ident: {}", quote!(#ident));
    // eprintln!("generics: {}", quote!(#generics));
    // eprintln!("brace_token: {}", quote!(#brace_token));
    // eprintln!("variants: {}", quote!(#variants));

    let enum_token = &token.enum_token;
    let ident = &token.ident;
    let variants = &token.variants;

    let nametoken = utils::quotify(quote!(#ident));
    let typetoken = utils::quotify(quote!(#enum_token));
    let dtype_inputs = parse_variants(&variants);

    let dtype_struct = quote!(
        dtypei::Typei {
            name: String::from(#nametoken),
            type_choice: String::from(#typetoken),
            // type_choice: dtypei::TypeChoices::PureFunction,
            inputs: vec![
                #(#dtype_inputs),*
            ],
            outputs: vec![],
        }
    );

    dtype_struct
}

fn parse_variants(inputs: &Punctuated<Variant, Comma>) -> Vec<proc_macro2::TokenStream> {
    let mut dtype_inputs = Vec::new();

    eprintln!("inputs: {}", quote!(#inputs));

    for (_, variant) in inputs.iter().enumerate() {
        eprintln!("variant: {}", quote!(#variant));
        // attrs, ident, fields, discriminant
        let ident = &variant.ident;
        let fields = &variant.fields;
        // let discriminant = &variant.discriminant;
        eprintln!("ident: {}", quote!(#ident));
        eprintln!("fields: {}", quote!(#fields));
        // eprintln!("variant: {}", quote!(#discriminant));
        let nametoken = utils::quotify(quote!(#ident));

        // TODO - more complex enums; wasm_bidgen is a bottleneck here
        // let dtype_inputss = fields_parse::parse(&fields);
        // eprintln!("dtype_inputss: {}", quote!(
        //     #(#dtype_inputss),*
        // ));

        dtype_inputs.push(quote!(
            dtypei::SubTypes {
                name: String::from("u8"),
                label: String::from(#nametoken),
                dimensions: Vec::new(),
            }
        ));
    }

    dtype_inputs
}
