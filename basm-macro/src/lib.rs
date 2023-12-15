extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

use std::fmt::Write;

#[proc_macro_attribute]
pub fn basm_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_in = parse_macro_input!(item as ItemFn);
    let fn_name = &fn_in.sig.ident;

    /* verify the function signature is compatible with basm-export */
    assert!(fn_in.sig.asyncness.is_none());
    assert!(fn_in.sig.generics.lt_token.is_none());
    if fn_in.sig.generics.params.iter().next().is_some() {
        panic!();
    }
    assert!(fn_in.sig.generics.gt_token.is_none());
    assert!(fn_in.sig.generics.where_clause.is_none());
    assert!(fn_in.sig.variadic.is_none());

    let mut fn_name_out = String::new();
    write!(&mut fn_name_out, "_basm_export_{0}", &fn_name).unwrap();
    let fn_name_out: TokenStream2 = fn_name_out.parse().unwrap();
    let inputs = &fn_in.sig.inputs;
    let input_names: syn::punctuated::Punctuated::<syn::Ident, syn::token::Comma> = {
        let mut ret = vec![];
        for tok in inputs.iter() {
            match tok.clone() {
                syn::FnArg::Receiver(_) => { panic!(); }
                syn::FnArg::Typed(a) => {
                    if let syn::Pat::Ident(g) = *a.pat {
                        ret.push(g.ident);
                    } else {
                        panic!();
                    }
                }
            }
        }
        syn::punctuated::Punctuated::from_iter(ret)
    };
    let output = &fn_in.sig.output;
    let fn_export = quote!{
        #[cfg(target_arch = "x86_64")]
        #[no_mangle]
        unsafe extern "win64" fn #fn_name_out(#inputs) #output {
            #fn_name(#input_names)
        }
        #[cfg(not(target_arch = "x86_64"))]
        #[no_mangle]
        unsafe extern "C" fn #fn_name_out(#inputs) #output {
            #fn_name(#input_names)
        }
    };

    /* output consists of the original function and the exported thunk which calls it */
    let expanded = quote!{
        #fn_in
        #fn_export
    };
    expanded.to_token_stream().into()
}