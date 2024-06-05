use proc_macro2::{Span, TokenStream};
use syn::Ident;

pub enum TInteger {
    I8,
    I16,
    I32,
    I64,
    Isize,
    U8,
    U16,
    U32,
    U64,
    Usize,
    Bool,
}

pub enum PtrSpecifier {
    None,
    PtrConst,
    PtrMut,
}

pub enum TPrimitive {
    Integer(PtrSpecifier, TInteger),
    String,
    Unit,
}

pub struct TPair {
    pub ty1: Box<TBase>,
    pub ty2: Box<TBase>,
}

pub struct TVec {
    pub ty: Box<TBase>,
}

pub enum TBase {
    Prim(TPrimitive),
    Pair(TPair),
    Vec(TVec),
}

pub enum BorrowSpecifier {
    None,
    BorrowConst,
    BorrowMut,
}

pub struct TInput {
    pub borrow: BorrowSpecifier,
    pub ty: TBase,
}

pub struct TOutput {
    pub ty: TBase,
}

pub struct TArg {
    pub ident: Ident,
    pub ty: TInput,
}

pub struct TFunction {
    pub ident: Ident,
    pub args: Vec<TArg>,
    pub output: TOutput,
}

pub trait Mangle {
    fn mangle(&self) -> String;
}

impl TFunction {
    pub fn arg_names(&self) -> Vec<Ident> {
        self.args.iter().map(|x| x.ident.clone()).collect()
    }
    pub fn arg_names_anonymous(&self) -> Vec<Ident> {
        (0..self.args.len())
            .map(|id| Ident::new(&format!("arg{id}"), Span::mixed_site()))
            .collect()
    }
    pub fn arg_borrows(&self) -> Vec<TokenStream> {
        self.args
            .iter()
            .map(|x| {
                match x.ty.borrow {
                    BorrowSpecifier::None => "",
                    BorrowSpecifier::BorrowConst => "&",
                    BorrowSpecifier::BorrowMut => "&mut",
                }
                .parse()
                .unwrap()
            })
            .collect()
    }
    pub fn arg_muts(&self) -> Vec<TokenStream> {
        self.args
            .iter()
            .map(|x| {
                match x.ty.borrow {
                    BorrowSpecifier::None => "",
                    BorrowSpecifier::BorrowConst => "",
                    BorrowSpecifier::BorrowMut => "mut",
                }
                .parse()
                .unwrap()
            })
            .collect()
    }
}

pub mod mangle;
pub mod try_from;
