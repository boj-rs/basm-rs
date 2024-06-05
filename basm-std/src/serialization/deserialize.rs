use super::Pair;
use alloc::string::String;
use alloc::vec::Vec;

pub trait De {
    fn de(buf: &mut &[u8]) -> Self;
}

impl De for () {
    fn de(_buf: &mut &[u8]) -> Self {}
}

macro_rules! impl_int {
    ($($ty:ty)*) => {
        $(
            impl De for $ty {
                fn de(buf: &mut &[u8]) -> Self {
                    let res = Self::from_be_bytes(buf[0..core::mem::size_of::<$ty>()].try_into().unwrap());
                    *buf = &mut &buf[core::mem::size_of::<$ty>()..];
                    res
                }
            }
        )*
    }
}

macro_rules! impl_int_ptr {
    ($($ty:ty)*) => {
        $(
            impl De for *const $ty {
                fn de(buf: &mut &[u8]) -> Self {
                    usize::de(buf) as Self
                }
            }
            impl De for *mut $ty {
                fn de(buf: &mut &[u8]) -> Self {
                    usize::de(buf) as Self
                }
            }
        )*
    }
}

impl_int!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
impl_int_ptr!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize bool);

impl De for bool {
    fn de(buf: &mut &[u8]) -> Self {
        let val = u8::de(buf);
        val != 0
    }
}

impl<T: De, const N: usize> De for [T; N] {
    fn de(buf: &mut &[u8]) -> Self {
        [0; N].map(move |_| T::de(buf))
    }
}

impl<T: De> De for Vec<T> {
    fn de(buf: &mut &[u8]) -> Self {
        let size = usize::de(buf);
        (0..size).map(move |_| T::de(buf)).collect()
    }
}

impl De for String {
    fn de(buf: &mut &[u8]) -> Self {
        let size = usize::de(buf);
        let out = String::from_utf8(buf[..size].to_vec()).unwrap();
        *buf = &buf[size..];
        out
    }
}

impl<T1: De, T2: De> De for Pair<T1, T2> {
    fn de(buf: &mut &[u8]) -> Self {
        // Expressions taking multiple operands are evaluated left to right
        // as written in the source code.
        Pair(T1::de(buf), T2::de(buf))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;

    #[test]
    fn check_deserialize() {
        let mut buf: &[u8] = &[0, 0, 1, 0, 0, 0, 0, 1];
        let x = i32::de(&mut buf);
        assert_eq!(256, x);
        let x = i32::de(&mut buf);
        assert!(buf.is_empty());
        assert_eq!(1, x);

        let mut buf: &[u8] = &[0, 0, 1, 0, 0, 0, 0, 1, 255, 255, 255, 253];
        let x = <[i32; 3]>::de(&mut buf);
        assert!(buf.is_empty());
        assert_eq!([256, 1, -3], x);

        let mut buf_vec = vec![0u8; core::mem::size_of::<usize>()];
        *buf_vec.last_mut().unwrap() = 3;
        buf_vec.extend_from_slice(&[0, 0, 1, 0, 0, 0, 0, 1, 255, 255, 255, 253]);
        let mut buf = &buf_vec[..];
        let x = <Vec<i32>>::de(&mut buf);
        assert!(buf.is_empty());
        assert_eq!(vec![256, 1, -3], x);

        let mut buf_vec = vec![0u8; core::mem::size_of::<usize>()];
        *buf_vec.last_mut().unwrap() = 12;
        buf_vec.extend_from_slice(b"Hello World!".as_slice());
        let mut buf = &buf_vec[..];
        let x = String::de(&mut buf);
        assert!(buf.is_empty());
        assert_eq!("Hello World!", x);

        let mut buf: &[u8] = &[253, 0, 0, 0, 0, 0, 0, 0, 7];
        let x = Pair::<i8, u64>::de(&mut buf);
        assert!(buf.is_empty());
        assert_eq!(Pair(-3i8, 7u64), x);
    }
}
