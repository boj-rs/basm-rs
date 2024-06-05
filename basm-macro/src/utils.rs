/// Verifies that the function signature is compatible with basm-export/basm-import,
/// except for input/output types. The input/output types will be checked by
/// `TryFrom` implementations.
pub fn verify_signature(sig: &syn::Signature) {
    assert!(
        sig.constness.is_none(),
        "Export/Import functions should not be marked as const"
    );
    assert!(
        sig.asyncness.is_none(),
        "Export/Import functions should not be marked as async"
    );
    assert!(
        sig.unsafety.is_none(),
        "Export/Import functions should not be marked as unsafe"
    );
    assert!(
        sig.abi.is_none(),
        "Export/Import functions should not have an ABI specifier"
    );
    assert!(
        sig.generics.params.iter().next().is_none(),
        "Export/Import functions should not have generic parameters"
    );
    assert!(sig.generics.lt_token.is_none());
    assert!(sig.generics.gt_token.is_none());
    assert!(sig.generics.where_clause.is_none());
    assert!(
        sig.variadic.is_none(),
        "Export/Import functions should not have variadic arguments"
    );
}
