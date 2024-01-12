/// Verifies that the function signature is compatible with basm-export/basm-import,
/// except for input/output types. The input/output types will be checked by
/// `TryFrom` implementations.
pub fn verify_signature(sig: &syn::Signature) {
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