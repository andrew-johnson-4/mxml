#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::{quote};

mod mixin;

#[proc_macro]
pub fn mixin(input: TokenStream) -> TokenStream {
    let mixin = parse_macro_input!(input as mixin::Mixin);

    let expanded = quote! {
       #mixin
    };

    TokenStream::from(expanded)
}
