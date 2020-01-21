extern crate proc_macro;

use quote::quote;
use syn::{ItemEnum, Variant};
use syn::punctuated::{Punctuated};
use syn::token::Comma;

use crate::utils;

pub fn parse(token: &ItemEnum) -> proc_macro2::TokenStream {
    // function: attrs, vis, enum_token, ident, generics, brace_token, variants

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

    for (_, variant) in inputs.iter().enumerate() {
        // attrs, ident, fields, discriminant
        let ident = &variant.ident;
        let nametoken = utils::quotify(quote!(#ident));

        // TODO - more complex enums; wasm_bidgen is a bottleneck here
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
