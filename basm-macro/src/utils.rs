extern crate proc_macro2;
extern crate syn;
use proc_macro2::TokenStream;
use syn::{Type, Signature};

/// Verifies the function signature is compatible with basm-export/basm-import.
pub fn verify_signature(sig: &Signature) {
    assert!(sig.constness.is_none());
    assert!(sig.asyncness.is_none());
    assert!(sig.unsafety.is_none());
    assert!(sig.abi.is_none());
    assert!(sig.generics.lt_token.is_none());
    if sig.generics.params.iter().next().is_some() {
        panic!();
    }
    assert!(sig.generics.gt_token.is_none());
    assert!(sig.generics.where_clause.is_none());
    assert!(sig.variadic.is_none());
}

/// Verifies and canonicalizes the type.
/// The type 
pub fn canonicalize_type(_ty: &Type) -> TokenStream {
    "".parse().unwrap()
}

pub fn mangle(_sig: &Signature) -> String {
    "some_mangled_name".into()
}