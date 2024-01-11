extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Signature, parse::{Parse, ParseStream}, Result, Token};

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

fn import_impl_single(sig: Signature) -> TokenStream {
    super::utils::verify_signature(&sig);
    for tok in sig.inputs {
        match tok {
            syn::FnArg::Receiver(_) => {
                // self, &self, &mut self are not allowed
                panic!();
            }
            syn::FnArg::Typed(_pattype) => {
            }
        }
    }
    "".parse().unwrap()
}

pub fn import_impl(input: TokenStream) -> TokenStream {
    let vecsig: VecSignature = syn::parse2(input).unwrap();
    let out: Vec<_> = vecsig.sigs.into_iter().map(|sig| {
        import_impl_single(sig)
    }).collect();
    quote! {
        #(#out)*
    }
}