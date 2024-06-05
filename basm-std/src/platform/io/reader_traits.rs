use super::{Readable, ReaderTrait};
use alloc::string::String;

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

impl<T: Readable, const N: usize> Readable for [T; N] {
    fn read(reader: &mut impl ReaderTrait) -> Self {
        core::array::from_fn(|_| T::read(reader))
    }
}

impl Readable for () {
    fn read(_reader: &mut impl ReaderTrait) -> Self {}
}

impl<T: Readable> Readable for (T,) {
    fn read(reader: &mut impl ReaderTrait) -> Self {
        (T::read(reader),)
    }
}

macro_rules! impl_tuple {
    ($u:ident) => {};
    ($u:ident $($t:ident)+) => {
        impl<$u: Readable, $($t: Readable),+> Readable for ($u, $($t),+) {
            fn read(reader: &mut impl ReaderTrait) -> Self {
                ($u::read(reader), $($t::read(reader)),+)
            }
        }
        impl_tuple!($($t) +);
    };
}

impl_tuple!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
