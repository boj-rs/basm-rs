use core::mem::MaybeUninit;

use crate::io::Writer;

macro_rules! write_u_impl {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self, i: $type) {
            self.write_usize(i as usize);
        }
    };
}

macro_rules! write_i_impl {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self, i: $type) {
            if i.is_negative() {
                self.write(b"-");
            }
            self.write_usize(i.abs_diff(0) as usize);
        }
    };
}

impl<const N: usize> Writer<N> {
    write_u_impl!(write_u8, u8);
    write_u_impl!(write_u16, u16);
    write_u_impl!(write_u32, u32);
    write_u_impl!(write_u64, u64);
    write_i_impl!(write_i8, i8);
    write_i_impl!(write_i16, i16);
    write_i_impl!(write_i32, i32);
    write_i_impl!(write_i64, i64);
    pub fn write_usize(&mut self, mut i: usize) {
        let mut buf: [MaybeUninit<u8>; 20] = MaybeUninit::uninit_array();
        let mut offset = buf.len() - 1;
        buf[offset].write(b'0' + (i % 10) as u8);
        i /= 10;
        while i > 0 {
            offset -= 1;
            buf[offset].write(b'0' + (i % 10) as u8);
            i /= 10;
        }
        self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buf[offset..]) });
    }
}
