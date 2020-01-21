use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed};

use crate::utils;

pub fn parse(inputs: &Fields) -> Vec<proc_macro2::TokenStream> {
    let mut dtype_inputs = Vec::new();

    match inputs {
        Fields::Named(FieldsNamed {brace_token: _, named}) => {
            for (_, field) in named.iter().enumerate() {
                let fident = &field.ident;
                let ty = &field.ty;
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
        Fields::Unnamed(FieldsUnnamed {paren_token: _, unnamed: _}) => {
            // TODO
        }
        Fields::Unit => {
            // TODO
        }
    }

    dtype_inputs
}
