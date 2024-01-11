extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
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

pub fn import_impl(input: TokenStream) -> TokenStream {
    let vecsig: VecSignature = syn::parse2(input).unwrap();
    for sig in vecsig.sigs {
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
    }
    "".parse().unwrap()
}