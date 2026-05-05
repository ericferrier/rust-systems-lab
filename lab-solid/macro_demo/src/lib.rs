use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let block = &input.block;
    let vis = &input.vis;
    let sig = &input.sig;

    // We "wrap" the function and register it
    let expanded = quote! {
        #vis #sig {
            println!("[dispatcher] calling handler: {}", stringify!(#fn_name));
            #block
        }
    };

    expanded.into()
}