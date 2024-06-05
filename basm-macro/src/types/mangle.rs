use super::*;

impl Mangle for TInteger {
    fn mangle(&self) -> String {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::Isize => "isize",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::Usize => "usize",
            Self::Bool => "bool",
        }
        .into()
    }
}

impl Mangle for TPrimitive {
    fn mangle(&self) -> String {
        "prim_".to_owned()
            + &match self {
                Self::Integer(sp, ty) => {
                    match sp {
                        PtrSpecifier::None => "",
                        PtrSpecifier::PtrConst => "ptr_",
                        PtrSpecifier::PtrMut => "ptrmut_",
                    }
                    .to_owned()
                        + &ty.mangle()
                }
                Self::String => "string".into(),
                Self::Unit => "unit".into(),
            }
    }
}

impl Mangle for TPair {
    fn mangle(&self) -> String {
        format!("pair_{0}_{1}", self.ty1.mangle(), self.ty2.mangle())
    }
}

impl Mangle for TVec {
    fn mangle(&self) -> String {
        format!("vec_{0}", self.ty.mangle())
    }
}

impl Mangle for TBase {
    fn mangle(&self) -> String {
        match self {
            Self::Prim(x) => x.mangle(),
            Self::Pair(x) => x.mangle(),
            Self::Vec(x) => x.mangle(),
        }
    }
}

impl Mangle for TInput {
    fn mangle(&self) -> String {
        match self.borrow {
            BorrowSpecifier::None => "",
            BorrowSpecifier::BorrowConst => "bor_",
            BorrowSpecifier::BorrowMut => "bormut_",
        }
        .to_owned()
            + &self.ty.mangle()
    }
}

impl Mangle for TOutput {
    fn mangle(&self) -> String {
        self.ty.mangle()
    }
}

impl Mangle for Ident {
    fn mangle(&self) -> String {
        let x = self.to_string();
        format!("{0}_{1}", x.len(), x)
    }
}

impl Mangle for TArg {
    fn mangle(&self) -> String {
        format!("{0}_{1}", self.ident.mangle(), self.ty.mangle())
    }
}

impl Mangle for TFunction {
    fn mangle(&self) -> String {
        let mut args_mangled = vec![];
        for x in self.args.iter() {
            args_mangled.push("_".into());
            args_mangled.push(x.mangle());
        }
        let args_mangled_all = args_mangled.join("");
        format!(
            "{0}_{1}{2}_{3}",
            self.ident.mangle(),
            self.args.len(),
            args_mangled_all,
            self.output.mangle()
        )
    }
}
