use quote::quote;

pub fn quotify(token: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let typestr = format!("\"{}\"", quote!(#token));
    typestr.parse().unwrap()
}
