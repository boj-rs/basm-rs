use super::*;
use quote::quote;

fn try_parse_ident(value: &syn::Type) -> Result<String, String> {
    let syn::Type::Path(x) = value else {
        return Err("A primitive type must be of type Path".into());
    };
    if x.qself.is_some() {
        return Err("TypePath::qself must be None".into());
    }
    if x.path.leading_colon.is_some() {
        return Err("Path::leading_colon must be None".into());
    }
    for y in x.path.segments.iter() {
        if !y.arguments.is_empty() {
            return Err("PathSegment should not have any generic arguments".into());
        }
    }
    Ok(x.path
        .segments
        .iter()
        .map(|y| y.ident.to_string())
        .collect::<Vec<_>>()
        .join("::"))
}

fn try_parse_generics(value: &syn::Type) -> Result<(Vec<String>, Vec<&syn::Type>), String> {
    let syn::Type::Path(x) = value else {
        return Err("A generic type must be of type Path".into());
    };
    if x.qself.is_some() {
        return Err("TypePath::qself must be None".into());
    }
    if x.path.leading_colon.is_some() {
        return Err("Path::leading_colon must be None".into());
    }
    let segs: Vec<_> = x.path.segments.iter().collect();
    if segs.is_empty() {
        return Err("Path is empty".into());
    }
    let mut type_path = vec![];
    for &p in &segs[..segs.len() - 1] {
        if let syn::PathArguments::None = p.arguments {
            type_path.push(p.ident.to_string());
        } else {
            return Err("Non-rightmost PathSegment must have no arguments".into());
        }
    }
    let p = *segs.last().unwrap();
    type_path.push(p.ident.to_string());
    let syn::PathArguments::AngleBracketed(ga) = &p.arguments else {
        return Err("Generic arguments must be angle-bracketed".into());
    };
    let mut out = vec![];
    for y in ga.args.iter() {
        if let syn::GenericArgument::Type(z) = y {
            out.push(z);
        } else {
            return Err("A generic argument must have type Type".into());
        }
    }
    Ok((type_path, out))
}

impl TryFrom<&syn::Type> for TInteger {
    type Error = String;
    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        let x = try_parse_ident(value)?;
        match x.as_str() {
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "isize" => Ok(Self::Isize),
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "usize" => Ok(Self::Usize),
            "bool" => Ok(Self::Bool),
            _ => Err("Unsupported integer type ".to_owned() + &x.to_string()),
        }
    }
}

impl TryFrom<&syn::Type> for TPrimitive {
    type Error = String;
    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        if let Ok(x) = TInteger::try_from(value) {
            Ok(TPrimitive::Integer(PtrSpecifier::None, x))
        } else if let syn::Type::Ptr(x) = value {
            let sp = match (x.const_token.is_some(), x.mutability.is_some()) {
                (true, false) => PtrSpecifier::PtrConst,
                (false, true) => PtrSpecifier::PtrMut,
                _ => {
                    return Err("Invalid pointer specifier".into());
                }
            };
            let y = TInteger::try_from(&*x.elem)?;
            Ok(TPrimitive::Integer(sp, y))
        } else if let Ok(x) = try_parse_ident(value) {
            match x.as_str() {
                "alloc::string::String" | "string::String" | "String" => Ok(Self::String),
                _ => Err("Unsupported primitive token ".to_owned() + &x),
            }
        } else if let syn::Type::Tuple(x) = value {
            if x.elems.is_empty() {
                Ok(Self::Unit)
            } else {
                Err(format!(
                    "Failed to parse {}; note that tuples are not yet supported",
                    quote!(#value)
                ))
            }
        } else {
            Err(format!("Failed to parse TPrimitive {}", quote!(#value)))
        }
    }
}

impl TryFrom<&syn::Type> for TPair {
    type Error = String;
    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        let (type_path, generic_args) = try_parse_generics(value)?;
        let type_str = type_path.join("::");
        let types_allowed = [
            "basm_std::serialization::Pair",
            "basm::serialization::Pair",
            "serialization::Pair",
            "Pair",
        ];
        if !types_allowed.contains(&type_str.as_str()) {
            return Err("The supplied type is not allowed for Pair".into());
        }
        if generic_args.len() != 2 {
            return Err("Pair must have exactly two generic arguments".into());
        }
        let ty1 = TBase::try_from(generic_args[0])?;
        let ty2 = TBase::try_from(generic_args[1])?;
        Ok(Self {
            ty1: Box::new(ty1),
            ty2: Box::new(ty2),
        })
    }
}

impl TryFrom<&syn::Type> for TVec {
    type Error = String;
    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        let (type_path, generic_args) = try_parse_generics(value)?;
        let type_str = type_path.join("::");
        let types_allowed = ["alloc::vec::Vec", "vec::Vec", "Vec"];
        if !types_allowed.contains(&type_str.as_str()) {
            return Err("The supplied type is not allowed for Vec".into());
        }
        if generic_args.len() != 1 {
            return Err("Vec must have exactly one generic argument".into());
        }
        let ty = TBase::try_from(generic_args[0])?;
        Ok(Self { ty: Box::new(ty) })
    }
}

impl TryFrom<&syn::Type> for TBase {
    type Error = String;
    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        if let Ok(x) = TPair::try_from(value) {
            Ok(Self::Pair(x))
        } else if let Ok(x) = TVec::try_from(value) {
            Ok(Self::Vec(x))
        } else {
            match TPrimitive::try_from(value) {
                Ok(x) => Ok(Self::Prim(x)),
                Err(x) => Err(x),
            }
        }
    }
}

impl TryFrom<&syn::Type> for TInput {
    type Error = String;
    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        if let syn::Type::Reference(x) = value {
            if x.lifetime.is_some() {
                Err("basm-import does not support lifetime annotations".into())
            } else {
                match TBase::try_from(&*x.elem) {
                    Ok(y) => {
                        let sp = if x.mutability.is_some() {
                            BorrowSpecifier::BorrowMut
                        } else {
                            BorrowSpecifier::BorrowConst
                        };
                        Ok(Self { borrow: sp, ty: y })
                    }
                    Err(y) => Err(y),
                }
            }
        } else {
            match TBase::try_from(value) {
                Ok(y) => Ok(Self {
                    borrow: BorrowSpecifier::None,
                    ty: y,
                }),
                Err(y) => Err(y),
            }
        }
    }
}

impl TryFrom<&syn::ReturnType> for TOutput {
    type Error = String;
    fn try_from(value: &syn::ReturnType) -> Result<Self, Self::Error> {
        match value {
            syn::ReturnType::Default => Ok(Self {
                ty: TBase::Prim(TPrimitive::Unit),
            }),
            syn::ReturnType::Type(_, ty) => match TBase::try_from(&**ty) {
                Ok(x) => Ok(Self { ty: x }),
                Err(x) => Err(x),
            },
        }
    }
}

impl TryFrom<&syn::FnArg> for TArg {
    type Error = String;
    fn try_from(value: &syn::FnArg) -> Result<Self, Self::Error> {
        match value {
            syn::FnArg::Receiver(_) => Err("FnArg: self, &self, &mut self are not allowed".into()),
            syn::FnArg::Typed(pattype) => {
                let (pat, ty) = (&*pattype.pat, &*pattype.ty);
                let ident = match pat {
                    syn::Pat::Ident(x) => x.ident.clone(),
                    _ => return Err("Pat must be a pure Ident".into()),
                };
                let ty = TInput::try_from(ty)?;
                Ok(Self { ident, ty })
            }
        }
    }
}

impl TryFrom<&syn::Signature> for TFunction {
    type Error = String;
    fn try_from(value: &syn::Signature) -> Result<Self, Self::Error> {
        let ident = value.ident.clone();
        let mut args = vec![];
        for x in value.inputs.iter() {
            let y = TArg::try_from(x)?;
            args.push(y);
        }
        let output = TOutput::try_from(&value.output)?;
        Ok(Self {
            ident,
            args,
            output,
        })
    }
}
