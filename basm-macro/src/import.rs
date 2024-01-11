extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

pub fn import_impl(_item: TokenStream) -> TokenStream {
    for x in _item.into_iter() {
        use proc_macro::TokenTree::*;
        match x {
            Group(y) => println!("Group {y}"),
            Ident(y) => println!("Ident {y}"),
            Punct(y) => println!("Punct {y}"),
            Literal(y) => println!("Literal {y}")
        }
    }
    "".parse().unwrap()
}