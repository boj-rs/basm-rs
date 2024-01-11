mod utils;
mod export;
mod import;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn basm_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    export::export_impl(attr, item)
}

#[proc_macro]
pub fn basm_import(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item);
    import::import_impl(item).into()
}