use alloc::string::String;
use super::{ReaderTrait, Readable};

macro_rules! impl_primitive {
    ($($ty:ident)*) => {
        $(
            impl Readable for $ty {
                fn read(reader: &mut impl ReaderTrait) -> Self {
                    reader.$ty()
                }
            }
        )*
    }
}

impl_primitive!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f64);

impl Readable for String {
    fn read(reader: &mut impl ReaderTrait) -> Self {
        reader.word()
    }
}

#[allow(dead_code)]
pub struct Line(pub String);

impl Readable for Line {
    fn read(reader: &mut impl ReaderTrait) -> Self {
        Self(reader.line())
    }
}