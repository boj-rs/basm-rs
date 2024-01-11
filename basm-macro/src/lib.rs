mod export;
mod import;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn basm_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    export::export_impl(attr, item)
}

#[proc_macro]
pub fn basm_import(item: TokenStream) -> TokenStream {
    import::import_impl(item)
}