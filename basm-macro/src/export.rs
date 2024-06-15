use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

use super::types::{Mangle, TFunction};

pub fn export_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let itemfn: &ItemFn = &syn::parse2(item).unwrap();
    let sig = &itemfn.sig;
    super::utils::verify_signature(sig);
    let tfn = match TFunction::try_from(sig) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    };
    let arg_names_anonymous = tfn.arg_names_anonymous();
    let arg_borrows = tfn.arg_borrows();
    let arg_muts = tfn.arg_muts();
    let arg_pure_types: Vec<_> = sig
        .inputs
        .iter()
        .map(|x| {
            let syn::FnArg::Typed(pattype) = x else {
                panic!()
            };
            let ty = &*pattype.ty;
            match ty {
                syn::Type::Reference(x) => &*x.elem,
                _ => ty,
            }
        })
        .collect();
    let mangled = tfn.mangle();

    let basm_export_mod: TokenStream = ("basm_export_mod_".to_owned() + &mangled).parse().unwrap();
    let basm_export: TokenStream = ("_basm_export_".to_owned() + &mangled).parse().unwrap();
    let internals: TokenStream = ("internals_".to_owned() + &mangled).parse().unwrap();
    let fn_name = &tfn.ident;
    let out = quote! {
        #[allow(clippy::ptr_arg)]
        #itemfn
        mod #basm_export_mod {
            mod #internals {
                pub static mut SER_VEC: alloc::vec::Vec::<u8> = alloc::vec::Vec::<u8>::new();

                #[cfg(target_arch = "x86_64")]
                #[inline(never)]
                pub unsafe extern "win64" fn free() { SER_VEC.clear() }

                #[cfg(not(target_arch = "x86_64"))]
                #[inline(never)]
                pub unsafe extern "C" fn free() { SER_VEC.clear() }

                #[cfg(target_arch = "x86_64")]
                #[no_mangle]
                #[inline(never)]
                unsafe extern "win64" fn #basm_export(ptr_serialized: usize) -> usize {
                    super::basm_export_impl(ptr_serialized)
                }

                #[cfg(not(target_arch = "x86_64"))]
                #[no_mangle]
                #[inline(never)]
                unsafe extern "C" fn #basm_export(ptr_serialized: usize) -> usize {
                    super::basm_export_impl(ptr_serialized)
                }
            }

            use super::*;
            unsafe fn basm_export_impl(ptr_serialized: usize) -> usize {
                extern crate basm_std;
                use basm_std::serialization::{Ser, De};

                let mut buf: &'static [u8] = basm_std::serialization::eat(ptr_serialized);
                #( let #arg_muts #arg_names_anonymous = <#arg_pure_types>::de(&mut buf); )*
                let ptr_free_remote = usize::de(&mut buf);
                assert!(buf.is_empty());
                basm_std::serialization::call_free(ptr_free_remote);
                let out = super::#fn_name(#( #arg_borrows #arg_names_anonymous ),*);

                assert!(#internals::SER_VEC.is_empty());
                out.ser_len(&mut #internals::SER_VEC, 0);
                (#internals::free as usize).ser_len(&mut #internals::SER_VEC, 0);
                #internals::SER_VEC.as_ptr() as usize
            }
        }
    };
    out
}
