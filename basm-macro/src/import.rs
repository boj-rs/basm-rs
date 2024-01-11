extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Signature, parse::{Parse, ParseStream}, Result, Token, parse_macro_input};

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
    let vecsig: VecSignature = parse_macro_input!(input);
    for sig in vecsig.sigs {
        super::utils::verify_signature(&sig);
    }
    "".parse().unwrap()
}