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

fn check_type(input: &str, is_output_type: bool) -> bool {
    let base_types = ["i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64", "usize"];
    let mut derived_types = HashSet::new();
    for &ty in base_types.iter() {
        derived_types.insert(ty.to_string());
        let x = format!("* const {0}", &ty).to_string();
        derived_types.insert(x);
        let x = format!("* mut {0}", &ty).to_string();
        derived_types.insert(x);
        let x = format!("Vec :: < {0} >", &ty).to_string();
        derived_types.insert(x);
        if !is_output_type {
            let x = format!("& Vec :: < {0} >", &ty).to_string();
            derived_types.insert(x);
            let x = format!("& mut Vec :: < {0} >", &ty).to_string();
            derived_types.insert(x);
        }
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
    let mut inputs_thunk = vec![];
    let mut prologue = vec![];
    let mut input_names = vec![];
    for tok in inputs.iter() {
        match tok.clone() {
            syn::FnArg::Receiver(_) => { panic!(); }
            syn::FnArg::Typed(a) => {
                if let syn::Pat::Ident(ref g) = *a.pat {
                    input_names.push(g.ident.clone());
                } else {
                    panic!();
                }
                let a_ty = &a.ty;
                let a_ty = quote!{#a_ty}.to_string();
                if !check_type(&a_ty, false) {
                    panic!("Unsupported input type \"{}\"", &a_ty);
                }
                if a_ty.contains("Vec") {
                    let a_pat = &a.pat;
                    let ident = quote!{#a_pat}.to_string();
                    let ty = { 
                        let toks: Vec<&str> = a_ty.split(" ").collect();
                        toks[toks.len() - 2]
                    };
                    inputs_thunk.push(format!("basm_export_thunk_{0}_data: *const {1}", &ident, &ty).to_string());
                    inputs_thunk.push(format!("basm_export_thunk_{0}_len: usize", &ident).to_string());
                    prologue.push(format!("let mut basm_export_thunk_{0}_vec_instance = alloc::vec::Vec::new();", &ident).to_string());
                    prologue.push(format!("for i in 0..basm_export_thunk_{0}_len {{ basm_export_thunk_{0}_vec_instance.push(*basm_export_thunk_{0}_data.add(i)); }}", &ident).to_string());
                    if a_ty.starts_with("& mut Vec") {
                        prologue.push(format!("let {0} = &mut basm_export_thunk_{0}_vec_instance;", &ident).to_string());
                    } else if a_ty.starts_with("& Vec") {
                        prologue.push(format!("let {0} = &basm_export_thunk_{0}_vec_instance;", &ident).to_string());
                    } else {
                        prologue.push(format!("let {0} = basm_export_thunk_{0}_vec_instance;", &ident).to_string());
                    }
                } else {
                    inputs_thunk.push(quote!{#tok}.to_string());
                }
            }
        }
    }
    let inputs_thunk: TokenStream2 = inputs_thunk.join(", ").parse().unwrap();
    let prologue: TokenStream2 = prologue.join("\n").parse().unwrap();
    let input_names: syn::punctuated::Punctuated::<syn::Ident, syn::token::Comma> =
        syn::punctuated::Punctuated::from_iter(input_names);
    let output = &fn_in.sig.output;
    let (o_ty, output_type) = if let syn::ReturnType::Type(_, x) = output {
        let o_ty = quote!{#x}.to_string();
        if !check_type(&o_ty, true) {
            panic!("Unsupported output type \"{}\"", &o_ty);
        }
        (quote!{#x}, "_".to_owned() + &mangle(&o_ty))
    } else {
        (TokenStream2::new(), String::new())
    };

    /* name mangling */
    let fn_name = &fn_in.sig.ident;
    let mut fn_name_out = String::new();
    write!(&mut fn_name_out, "{0}_{1}{2}",
        &mangle(&fn_name.to_string()), &mangle(&quote!{#inputs}.to_string()), &output_type).unwrap();
    let fn_name_out = String::from("_basm_export_") + &fn_name_out;
    let fn_name_out: TokenStream2 = fn_name_out.parse().unwrap();

    /* output epilogue */
    let mut output = {
        let tmp = &fn_in.sig.output;
        quote!{#tmp}
    };
    let mut global_var = TokenStream2::new();
    let mut epilogue = quote!{
        out
    };
    if output.to_string().contains("Vec") {
        let vec_name: TokenStream2 = format!("BASM_EXPORT_{0}_VECTOR_REF_HOLDER", &fn_in.sig.ident).to_string().parse().unwrap();
        let dealloc_name: TokenStream2 = format!("basm_export_{0}_dealloc", &fn_in.sig.ident).to_string().parse().unwrap();
        global_var = quote!{
            static mut #vec_name: alloc::vec::Vec::<alloc::rc::Rc::<#o_ty>> = alloc::vec::Vec::<alloc::rc::Rc::<#o_ty>>::new();
            #[cfg(target_arch = "x86_64")]
            #[inline(never)]
            extern "win64" fn #dealloc_name() {
                unsafe { #vec_name.clear(); }
            }
            #[cfg(not(target_arch = "x86_64"))]
            #[inline(never)]
            extern "C" fn #dealloc_name() {
                unsafe { #vec_name.clear(); }
            }
        };
        epilogue = quote!{
            let out_len = out.len();
            #vec_name.push(alloc::rc::Rc::<#o_ty>::new(out));
            let ptr: &mut #o_ty = alloc::rc::Rc::get_mut(&mut #vec_name[#vec_name.len() - 1]).unwrap();
            [ptr.as_ptr() as usize, out_len, #dealloc_name as usize]
        };
        output = quote!{
            -> [usize; 3]
        };
    }

    /* emit original function along with a thunk */
    let fn_export = quote!{
        #global_var
        #[cfg(target_arch = "x86_64")]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "win64" fn #fn_name_out(#inputs_thunk) #output {
            #prologue
            let out = #fn_name(#input_names);
            #epilogue
        }
        #[cfg(not(target_arch = "x86_64"))]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn #fn_name_out(#inputs) #output {
            #prologue
            let out = #fn_name(#input_names);
            #epilogue
        }
    };

    /* output consists of the original function and the exported thunk which calls it */
    let expanded = quote!{
        #fn_in
        #fn_export
    };
    expanded.to_token_stream().into()
}