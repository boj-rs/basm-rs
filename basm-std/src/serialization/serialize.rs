use alloc::string::String;
use alloc::vec::Vec;
use super::Pair;

pub trait Ser {
    fn ser(&self, buf: &mut Vec<u8>);
}

macro_rules! impl_int {
    ($($ty:ty)*) => {
        $(
            impl Ser for $ty {
                fn ser(&self, buf: &mut Vec<u8>) {
                    buf.extend_from_slice(&self.to_be_bytes());
                }
            }
        )*
    }
}

impl_int!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

impl<T: Ser, const N: usize> Ser for [T; N] {
    fn ser(&self, buf: &mut Vec<u8>) {
        self.iter().for_each(|x| x.ser(buf))
    }
}

impl<T: Ser> Ser for Vec<T> {
    fn ser(&self, buf: &mut Vec<u8>) {
        self.len().ser(buf);
        self.iter().for_each(move |x| x.ser(buf))
    }
}

impl Ser for String {
    fn ser(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        bytes.len().ser(buf);
        buf.extend_from_slice(bytes)
    }
}

impl<T1: Ser, T2: Ser> Ser for Pair<T1, T2> {
    fn ser(&self, buf: &mut Vec<u8>) {
        self.0.ser(buf);
        self.1.ser(buf)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;

    #[test]
    fn check_serialize() {
        let mut buf = vec![];
        256i32.ser(&mut buf);
        1i32.ser(&mut buf);
        assert_eq!(vec![0, 0, 1, 0, 0, 0, 0, 1], buf);

        let mut buf = vec![];
        [256, 1, -3].ser(&mut buf);
        assert_eq!(vec![0, 0, 1, 0, 0, 0, 0, 1, 255, 255, 255, 253], buf);

        let mut buf = vec![];
        vec![256, 1, -3].ser(&mut buf);
        let mut target = vec![0u8; core::mem::size_of::<usize>()];
        *target.last_mut().unwrap() = 3;
        target.extend_from_slice(&[0, 0, 1, 0, 0, 0, 0, 1, 255, 255, 255, 253]);
        assert_eq!(target, buf);

        let mut buf = vec![];
        String::from("Hello World!").ser(&mut buf);
        let mut target = vec![0u8; core::mem::size_of::<usize>()];
        *target.last_mut().unwrap() = 12;
        target.extend_from_slice(b"Hello World!".as_slice());
        assert_eq!(target, buf);

        let mut buf = vec![];
        Pair(-3i8, 7u64).ser(&mut buf);
        assert_eq!(vec![253, 0, 0, 0, 0, 0, 0, 0, 7], buf);
    }
}