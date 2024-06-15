extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod export;
mod import;
mod types;
mod utils;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn basm_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr);
    let item = parse_macro_input!(item);
    export::export_impl(attr, item).into()
}

#[proc_macro]
pub fn basm_import(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item);
    import::import_impl(item).into()
}
