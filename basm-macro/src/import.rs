use proc_macro2::TokenStream;
use quote::quote;
use syn::{Signature, parse::{Parse, ParseStream}, Result, Token};

use super::types::{TFunction, Mangle};

struct VecSignature {
    sigs: Vec<Signature>
}

impl Parse for VecSignature {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut sigs = vec![];
        while !input.is_empty() {
            let sig: Signature = input.parse()?;
            let _semi: Token![;] = input.parse()?;
            sigs.push(sig);
        }
        Ok(Self { sigs })
    }
}

fn import_impl_single(sig: &Signature) -> TokenStream {
    super::utils::verify_signature(sig);
    let tfn = match TFunction::try_from(sig) {
        Ok(x) => x,
        Err(x) => panic!("{}", x)
    };
    let arg_names = tfn.arg_names();
    let mangled = tfn.mangle();

    let basm_import_mod: TokenStream = ("basm_import_mod_".to_owned() + &mangled).parse().unwrap();
    let basm_import: TokenStream = ("_basm_import_".to_owned() + &mangled).parse().unwrap();
    let internals: TokenStream = ("internals_".to_owned() + &mangled).parse().unwrap();
    let fn_name = &tfn.ident;
    let return_type: TokenStream = match &sig.output {
        syn::ReturnType::Default => { "()".parse().unwrap() }
        syn::ReturnType::Type(_x, y) => { quote!(#y) }
    };
    let out = quote! {
        mod #basm_import_mod {
            mod #internals {
                pub static mut SER_VEC: alloc::vec::Vec::<u8> = alloc::vec::Vec::<u8>::new();
                pub static mut PTR_FN: usize = 0;
            
                #[cfg(target_arch = "x86_64")]
                #[inline(never)]
                pub unsafe extern "win64" fn free() { SER_VEC.clear() }
            
                #[cfg(not(target_arch = "x86_64"))]
                #[inline(never)]
                pub unsafe extern "C" fn free() { SER_VEC.clear() }
            
                #[cfg(target_arch = "x86_64")]
                #[no_mangle]
                #[inline(never)]
                pub unsafe extern "win64" fn #basm_import(ptr_fn: usize) { PTR_FN = ptr_fn; }
            
                #[cfg(not(target_arch = "x86_64"))]
                #[no_mangle]
                #[inline(never)]
                pub unsafe extern "C" fn #basm_import(ptr_fn: usize) { PTR_FN = ptr_fn; }
            }
        
            use super::*;
            pub #sig {
                extern crate basm_std;
                use basm_std::serialization::{Ser, De, eat, Pair};    
                use core::mem::transmute;
                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    let ptr_fn: extern "win64" fn(usize) -> usize = transmute(#internals::PTR_FN);
                    #[cfg(not(target_arch = "x86_64"))]
                    let ptr_fn: extern "C" fn(usize) -> usize = transmute(#internals::PTR_FN);
            
                    assert!(#internals::SER_VEC.is_empty());
                    #( #arg_names.ser_len(&mut #internals::SER_VEC, 0); )*
                    (#internals::free as usize).ser(&mut #internals::SER_VEC);
                    let ptr_serialized = ptr_fn(#internals::SER_VEC.as_ptr() as usize);
            
                    let (mut buf, ptr_free_remote): (&'static [u8], usize) = eat(ptr_serialized);
                    #[cfg(target_arch = "x86_64")]
                    let free_remote: extern "win64" fn() -> () = transmute(ptr_free_remote);
                    #[cfg(not(target_arch = "x86_64"))]
                    let free_remote: extern "C" fn() -> () = transmute(ptr_free_remote);
        
                    type return_type = #return_type;
                    let out = return_type::de(&mut buf);
                    assert!(buf.is_empty());
                    free_remote();
                    out
                }
            }
        }
        use #basm_import_mod::#fn_name;
    };
    out
}

pub fn import_impl(input: TokenStream) -> TokenStream {
    let vecsig: VecSignature = syn::parse2(input).unwrap();
    let out = vecsig.sigs.iter().map(|sig| {
        import_impl_single(sig)
    });
    quote! {
        #(#out)*
    }
}