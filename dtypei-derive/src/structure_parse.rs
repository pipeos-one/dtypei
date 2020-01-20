extern crate proc_macro;

// use crate::proc_macro::TokenStream;
// use proc_macro2::Span;
use quote::quote;
use syn;

use syn::{ItemStruct}; //, Fields, FieldsNamed, FieldsUnnamed, Field};
// use syn::punctuated::{Punctuated};
// use syn::token::Comma;

// use dtypei;

use crate::utils;
use crate::fields_parse;

pub fn parse(structt: &ItemStruct) -> proc_macro2::TokenStream {
    // function: attrs, vis, struct_token, ident, generics, fields, semi_token

    // let attrs = &structt.attrs;
    let vis = &structt.vis;
    let struct_token = &structt.struct_token;
    let ident = &structt.ident;
    let generics = &structt.generics;
    let fields = &structt.fields;
    let semi_token = &structt.semi_token;

    eprintln!("vis: {}", quote!(#vis));
    // eprintln!("attrs: {}", quote!(#attrs));
    eprintln!("struct_token: {}", quote!(#struct_token));
    eprintln!("ident: {}", quote!(#ident));
    eprintln!("generics: {}", quote!(#generics));
    eprintln!("fields: {}", quote!(#fields));
    eprintln!("semi_token: {}", quote!(#semi_token));

    let dtype_inputs = fields_parse::parse(&fields);
    let nametoken = utils::quotify(quote!(#ident));

    let dtype_struct = quote!(
        dtypei::Typei {
            name: String::from(#nametoken),
            // type_choice: dtypei::TypeChoices::PureFunction,
            inputs: vec![
                #(#dtype_inputs),*
            ],
            outputs: vec![],
        }
    );

    dtype_struct
}
