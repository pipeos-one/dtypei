extern crate proc_macro;

use quote::quote;
use syn;

use syn::{ItemStruct};

use crate::utils;
use crate::fields_parse;

pub fn parse(structt: &ItemStruct) -> proc_macro2::TokenStream {
    // attrs, vis, struct_token, ident, generics, fields, semi_token

    let struct_token = &structt.struct_token;
    let ident = &structt.ident;
    let fields = &structt.fields;

    let nametoken = utils::quotify(quote!(#ident));
    let typetoken = utils::quotify(quote!(#struct_token));
    let dtype_inputs = fields_parse::parse(&fields);

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
