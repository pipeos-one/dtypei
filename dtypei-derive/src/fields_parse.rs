use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed};

use crate::utils;

pub fn parse(inputs: &Fields) -> Vec<proc_macro2::TokenStream> {
    let mut dtype_inputs = Vec::new();

    // eprintln!("inputs: {}", quote!(#inputs));

    match inputs {
        Fields::Named(FieldsNamed {brace_token, named}) => {
            // eprintln!("brace_token: {}", quote!(#brace_token));
            // eprintln!("named: {}", quote!(#named));
            // Punctuated<Field, Comma>
            for (_, field) in named.iter().enumerate() {
                // eprintln!("field: {}", quote!(#field));
                // let attrs = &field.attrs;
                let vis = &field.vis;
                let fident = &field.ident;
                let colon_token = &field.colon_token;
                let ty = &field.ty;
                // eprintln!("vis: {}", quote!(#vis));
                // eprintln!("fident: {}", quote!(#fident));
                // eprintln!("colon_token: {}", quote!(#colon_token));
                // eprintln!("ty: {}", quote!(#ty));

                let tpattoken = utils::quotify(quote!(#fident));
                let tytoken = utils::quotify(quote!(#ty));

                dtype_inputs.push(quote!(
                    dtypei::SubTypes {
                        name: String::from(#tytoken),
                        label: String::from(#tpattoken),
                        dimensions: Vec::new(),
                    }
                ));
            }
        }
        Fields::Unnamed(FieldsUnnamed {paren_token, unnamed}) => {
            // eprintln!("paren_token: {}", quote!(#paren_token));
            // eprintln!("unnamed: {}", quote!(#unnamed));
        }
        Fields::Unit => {
            // eprintln!("unit")
        }
    }

    dtype_inputs
}
