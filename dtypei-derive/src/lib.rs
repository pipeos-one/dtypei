extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn;
use serde_json;

use syn::{parse_macro_input, parse_quote, Attribute, FnArg, Ident, ItemFn, PatType, ReturnType, Receiver, Type};

use dtypei;

#[proc_macro_attribute]
pub fn dtypei_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let inputstr = &input.to_string();

    if (inputstr.contains("wasm_bindgen")) {
        return input;
    }

    eprintln!("---- dtypei_attr input: {:?}", &input.to_string());

    let mut function = parse_macro_input!(input as ItemFn);

    let mut dtype_inputs = Vec::new();

    eprintln!("function: {}", quote!(#function));
    // attrs, vis, sig, block
    // let attrs = &function.attrs;
    let vis = &function.vis;
    let sig = &function.sig;

    eprintln!("vis: {}", quote!(#vis));
    eprintln!("sig: {}", quote!(#sig));

    // constness, asyncness, unsafety, abi, fn_token, ident, paren_token, inputs, variadic, output
    // let constness = &sig.constness;
    // let asyncness = &sig.asyncness;
    // let unsafety = &sig.unsafety;
    // let abi = &sig.abi;
    // let fn_token = &sig.fn_token;
    let ident = &sig.ident;
    // let paren_token = &sig.paren_token;
    // let variadic = &sig.variadic;
    let output = &sig.output;

    // eprintln!("constness: {}", quote!(#constness));
    // eprintln!("asyncness: {}", quote!(#asyncness));
    // eprintln!("unsafety: {}", quote!(#unsafety));
    // eprintln!("abi: {}", quote!(#abi));
    // eprintln!("fn_token: {}", quote!(#fn_token));
    eprintln!("ident: {}", quote!(#ident));
    // eprintln!("paren_token: {}", quote!(#paren_token));
    // eprintln!("variadic: {}", quote!(#variadic));
    eprintln!("output: {}", quote!(#output));


    for (i, input) in function.sig.inputs.iter().enumerate() {
        eprintln!("input: {}", quote!(#input));

        match input {
            FnArg::Typed(PatType { pat, attrs, colon_token, ty }) => {
                // eprintln!("pat: {:?}", &pat);
                eprintln!("pat: {}", quote!(#pat));
                // eprintln!("attrs: {}", quote!(#attrs));
                // eprintln!("colon_token: {}", quote!(#colon_token));
                eprintln!("ty: {}", quote!(#ty));

                let patstr = format!("\"{}\"", quote!(#pat));
                let tpattoken: proc_macro2::TokenStream = patstr.parse().unwrap();

                let tystr = format!("\"{}\"", quote!(#ty));
                let tytoken: proc_macro2::TokenStream = tystr.parse().unwrap();

                dtype_inputs.push(quote!(
                    dtypei::SubTypes {
                        name: String::from(#tytoken),
                        label: String::from(#tpattoken),
                        dimensions: Vec::new(),
                    }
                ));
            }
            FnArg::Receiver(Receiver {attrs, reference, mutability, self_token}) => {
                // eprintln!("attrs: {}", quote!(#attrs));
                // eprintln!("reference: {}", quote!(#reference));
                eprintln!("mutability: {}", quote!(#mutability));
                eprintln!("self_token: {}", quote!(#self_token));
            }
        }
    }

    let dtype_outputs = match output {
        ReturnType::Default => quote!(-> []),
        ReturnType::Type(_,  out) => {
            let typestr = format!("\"{}\"", quote!(#out));
            let typetoken: proc_macro2::TokenStream = typestr.parse().unwrap();
            quote!(
                vec![
                    dtypei::SubTypes {
                        name: String::from(#typetoken),
                        label: String::from("unknown"),
                        dimensions: Vec::new(),
                    }
                ]
            )
        }
    };

    eprintln!("dtype_outputs: {}", dtype_outputs);

    let namestr = format!("\"{}\"", quote!(#ident));
    let nametoken: proc_macro2::TokenStream = namestr.parse().unwrap();
    let dtype_func = quote!(
        dtypei::Typei {
            name: String::from(#nametoken),
            types: vec![
                #(#dtype_inputs),*
            ],
            outputs: #dtype_outputs,
        }
    );

    eprintln!("dtype_func: {}", dtype_func);

    dtypei::istradd(format!("{}", dtype_func));

    if (inputstr.contains("typedinterface")) {
        let interf = dtypei::istrget().into_iter().map(|item| {
            let tokenized: proc_macro2::TokenStream = item.parse().unwrap();
            tokenized
        });

        function.block = Box::new(parse_quote!({
            vec![
                #(#interf),*
            ]
        }));


        eprintln!("----- iiiiiiiii {:?}", TokenStream::from(quote!(#function)).to_string());

        return TokenStream::from(quote!(#function));
    }

    TokenStream::from(quote!(#function))
}

fn quotify(token: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // let typestr = format!("\"{}\"", quote!(#out));
    // let typetoken: proc_macro2::TokenStream = typestr.parse().unwrap();

    let typestr = format!("\"{}\"", quote!(#token));
    typestr.parse().unwrap()
}

fn impl_dtypei_macro(ast: &syn::DeriveInput) -> TokenStream {
    let gen = quote! {
        pub fn sum(n1: i32, n2: i32) -> i32 {
            n1 + n2
        }
    };
    gen.into()
}
