extern crate syn;
use syn::Signature;

/// Verifies the function signature is compatible with basm-export/basm-import.
pub fn verify_signature(sig: &Signature) {
    assert!(sig.asyncness.is_none());
    assert!(sig.generics.lt_token.is_none());
    if sig.generics.params.iter().next().is_some() {
        panic!();
    }
    assert!(sig.generics.gt_token.is_none());
    assert!(sig.generics.where_clause.is_none());
    assert!(sig.variadic.is_none());
}