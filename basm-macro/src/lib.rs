extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

use data_encoding::HEXLOWER;
use std::collections::HashSet;
use std::fmt::Write;

fn mangle(input: &str) -> String {
    HEXLOWER.encode(input.as_bytes())
}

fn check_type(input: &str) -> bool {
    let base_types = ["i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64", "usize"];
    let mut derived_types = HashSet::new();
    for &ty in base_types.iter() {
        derived_types.insert(ty.to_string());
        let x = format!("* const {0}", &ty).to_string();
        derived_types.insert(x);
        let x = format!("* mut {0}", &ty).to_string();
        derived_types.insert(x);
    }
    derived_types.contains(input)
}

#[proc_macro_attribute]
pub fn basm_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_in = parse_macro_input!(item as ItemFn);

    /* verify the function signature is compatible with basm-export */
    assert!(fn_in.sig.asyncness.is_none());
    assert!(fn_in.sig.generics.lt_token.is_none());
    if fn_in.sig.generics.params.iter().next().is_some() {
        panic!();
    }
    assert!(fn_in.sig.generics.gt_token.is_none());
    assert!(fn_in.sig.generics.where_clause.is_none());
    assert!(fn_in.sig.variadic.is_none());

    let inputs = &fn_in.sig.inputs;
    let mut input_names = vec![];
    for tok in inputs.iter() {
        match tok.clone() {
            syn::FnArg::Receiver(_) => { panic!(); }
            syn::FnArg::Typed(a) => {
                if let syn::Pat::Ident(g) = *a.pat {
                    input_names.push(g.ident);
                } else {
                    panic!();
                }
                let a_ty = &a.ty;
                let a_ty = quote!{#a_ty}.to_string();
                if !check_type(&a_ty) {
                    panic!("Unsupported input type \"{}\"", &a_ty);
                }
            }
        }
    }
    let input_names: syn::punctuated::Punctuated::<syn::Ident, syn::token::Comma> =
        syn::punctuated::Punctuated::from_iter(input_names);
    let output = &fn_in.sig.output;
    let output_type = if let syn::ReturnType::Type(_, x) = output {
        let o_ty = quote!{#x}.to_string();
        if !check_type(&o_ty) {
            panic!("Unsupported output type \"{}\"", &o_ty);
        }
        "_".to_owned() + &mangle(&o_ty)
    } else {
        String::new()
    };

    /* name mangling */
    let fn_name = &fn_in.sig.ident;
    let mut fn_name_out = String::new();
    write!(&mut fn_name_out, "{0}_{1}{2}",
        &mangle(&fn_name.to_string()), &mangle(&quote!{#inputs}.to_string()), &output_type).unwrap();
    let fn_name_out = String::from("_basm_export_") + &fn_name_out;
    let fn_name_out: TokenStream2 = fn_name_out.parse().unwrap();

    let fn_export = quote!{
        #[cfg(target_arch = "x86_64")]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "win64" fn #fn_name_out(#inputs) #output {
            #fn_name(#input_names)
        }
        #[cfg(not(target_arch = "x86_64"))]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn #fn_name_out(#inputs) #output {
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