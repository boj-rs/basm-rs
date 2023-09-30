use core::mem::MaybeUninit;
use crate::platform::services;

pub struct Writer<const N: usize = { super::DEFAULT_BUF_SIZE }> {
    buf: [MaybeUninit<u8>; N],
    off: usize,
}

impl<const N: usize> Default for Writer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Drop for Writer<N> {
    fn drop(&mut self) {
        self.flush();
    }
}

#[repr(align(16))]
struct B128([u8; 16]);
#[target_feature(enable = "avx2")]
unsafe fn cvt8(out: &mut B128, n: u32) -> usize {
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    let x = _mm_cvtsi32_si128(n as i32);
    let div_10000 = _mm_set1_epi32(0xd1b71759u32 as i32);
    let mul_10000_merge = _mm_set1_epi32(55536);
    let div_var = _mm_setr_epi16(
        8389,
        5243,
        13108,
        0x8000u16 as i16,
        8389,
        5243,
        13108,
        0x8000u16 as i16,
    );
    let shift_var = _mm_setr_epi16(
        1 << 7,
        1 << 11,
        1 << 13,
        (1 << 15) as i16,
        1 << 7,
        1 << 11,
        1 << 13,
        (1 << 15) as i16,
    );
    let mul_10 = _mm_set1_epi16(10);
    let ascii0 = _mm_set1_epi8(48);
    let x_div_10000 = _mm_srli_epi64::<45>(_mm_mul_epu32(x, div_10000));
    let y = _mm_add_epi32(x, _mm_mul_epu32(x_div_10000, mul_10000_merge));
    let t0 = _mm_slli_epi16::<2>(_mm_shuffle_epi32::<5>(_mm_unpacklo_epi16(y, y)));
    let t1 = _mm_mulhi_epu16(t0, div_var);
    let t2 = _mm_mulhi_epu16(t1, shift_var);
    let t3 = _mm_slli_epi64::<16>(t2);
    let t4 = _mm_mullo_epi16(t3, mul_10);
    let t5 = _mm_sub_epi16(t2, t4);
    let t6 = _mm_packus_epi16(_mm_setzero_si128(), t5);
    let mask = _mm_movemask_epi8(_mm_cmpeq_epi8(t6, _mm_setzero_si128()));
    let offset = (mask & !0x8000).trailing_ones() as usize;
    let ascii = _mm_add_epi8(t6, ascii0);
    _mm_store_si128(out.0.as_mut_ptr().cast(), ascii);
    offset
}

impl<const N: usize> Writer<N> {
    const _DUMMY: usize = {
        assert!(N >= super::MIN_BUF_SIZE, "Buffer size for Writer must be at least MIN_BUF_SIZE");
        0
    };
    pub fn new() -> Self {
        Self {
            buf: MaybeUninit::uninit_array(),
            off: 0
        }
    }
    pub fn flush(&mut self) {
        services::write_stdio(1, unsafe {
            MaybeUninit::slice_assume_init_ref(&self.buf[..self.off])
        });
        self.off = 0;
    }
    pub fn try_flush(&mut self, readahead: usize) {
        if self.off + readahead > self.buf.len() {
            self.flush();
        }
    }
    pub fn byte(&mut self, b: u8) {
        self.try_flush(1);
        self.buf[self.off].write(b);
        self.off += 1;
    }
    pub fn bytes(&mut self, s: &[u8]) {
        let mut i = 0;
        while i < s.len() {
            let rem = s[i..].len().min(self.buf[self.off..].len());
            unsafe { MaybeUninit::slice_assume_init_mut(&mut self.buf[self.off..self.off + rem]).copy_from_slice(&s[i..i + rem]); }
            self.off += rem;
            i += rem;
            if self.off == self.buf.len() {
                self.flush();
            }
        }
    }
    pub fn str(&mut self, s: &str) {
        self.bytes(s.as_bytes());
    }
    pub fn i8(&mut self, n: i8) {
        self.i32(n as i32);
    }
    pub fn u8(&mut self, n: u8) {
        self.u32(n as u32);
    }
    pub fn i16(&mut self, n: i16) {
        self.i32(n as i32);
    }
    pub fn u16(&mut self, n: u16) {
        self.u32(n as u32);
    }
    pub fn i32(&mut self, n: i32) {
        if n < 0 {
            self.byte(b'-');
            self.u32((n as u32).wrapping_neg());
        } else {
            self.u32(n as u32);
        }
    }
    pub fn u32(&mut self, n: u32) {
        self.try_flush(10);
        let mut b128 = B128([0u8; 16]);
        let mut off;
        if n >= 100_000_000 {
            let mut hi = n / 100_000_000;
            let lo = n % 100_000_000;
            unsafe { cvt8(&mut b128, lo) };
            off = 8;
            off -= 1;
            b128.0[off] = (hi % 10) as u8 + b'0';
            if hi >= 10 {
                off -= 1;
                hi /= 10;
                b128.0[off] = hi as u8 + b'0';
            }
        } else {
            off = unsafe { cvt8(&mut b128, n) };
        }
        let len = 16 - off;
        unsafe { MaybeUninit::slice_assume_init_mut(&mut self.buf[self.off..self.off + len]).copy_from_slice(&b128.0[off..]); }
        self.off += len;
    }
    pub fn i64(&mut self, n: i64) {
        if n < 0 {
            self.byte(b'-');
            self.u64((n as u64).wrapping_neg());
        } else {
            self.u64(n as u64);
        }
    }
    pub fn u64(&mut self, n: u64) {
        self.try_flush(20);
        let mut hi128 = B128([0u8; 16]);
        let mut lo128 = B128([0u8; 16]);
        let mut hioff;
        let looff;
        if n >= 10_000_000_000_000_000 {
            let mut hi = (n / 10_000_000_000_000_000) as u32;
            let lo = n % 10_000_000_000_000_000;
            let lohi = (lo / 100_000_000) as u32;
            let lolo = (lo % 100_000_000) as u32;
            unsafe { cvt8(&mut hi128, lohi) };
            unsafe { cvt8(&mut lo128, lolo) };
            hioff = 8;
            looff = 8;
            hioff -= 1;
            hi128.0[hioff] = (hi % 10) as u8 + b'0';
            if hi >= 10 {
                hioff -= 1;
                hi /= 10;
                hi128.0[hioff] = (hi % 10) as u8 + b'0';
            }
            if hi >= 10 {
                hioff -= 1;
                hi /= 10;
                hi128.0[hioff] = (hi % 10) as u8 + b'0';
            }
            if hi >= 10 {
                hioff -= 1;
                hi /= 10;
                hi128.0[hioff] = hi as u8 + b'0';
            }
        } else if n >= 100_000_000 {
            let hi = (n / 100_000_000) as u32;
            let lo = (n % 100_000_000) as u32;
            hioff = unsafe { cvt8(&mut hi128, hi) };
            unsafe { cvt8(&mut lo128, lo) };
            looff = 8;
        } else {
            hioff = 16;
            looff = unsafe { cvt8(&mut lo128, n as u32) };
        }
        let len = 16 - hioff;
        unsafe { MaybeUninit::slice_assume_init_mut(&mut self.buf[self.off..self.off + len]).copy_from_slice(&hi128.0[hioff..]); }
        self.off += len;
        let len = 16 - looff;
        unsafe { MaybeUninit::slice_assume_init_mut(&mut self.buf[self.off..self.off + len]).copy_from_slice(&lo128.0[looff..]); }
        self.off += len;
    }
    pub fn i128(&mut self, n: i128) {
        if n < 0 {
            self.byte(b'-');
            self.u128((n as u128).wrapping_neg());
        } else {
            self.u128(n as u128);
        }
    }
    pub fn u128(&mut self, mut n: u128) {
        let mut buf: [MaybeUninit<u8>; 40] = MaybeUninit::uninit_array();
        let mut offset = buf.len() - 1;
        buf[offset].write(b'0' + (n % 10) as u8);
        n /= 10;
        while n > 0 {
            offset -= 1;
            buf[offset].write(b'0' + (n % 10) as u8);
            n /= 10;
        }
        self.bytes(unsafe { MaybeUninit::slice_assume_init_ref(&buf[offset..]) });
    }
    #[cfg(target_pointer_width = "32")]
    pub fn isize(&mut self, n: isize) {
        self.i32(n as i32);
    }
    #[cfg(target_pointer_width = "32")]
    pub fn usize(&mut self, n: usize) {
        self.u32(n as u32);
    }
    #[cfg(target_pointer_width = "64")]
    pub fn isize(&mut self, n: isize) {
        self.i64(n as i64);
    }
    #[cfg(target_pointer_width = "64")]
    pub fn usize(&mut self, n: usize) {
        self.u64(n as u64);
    }
    #[cfg(all(not(target_pointer_width = "32"), not(target_pointer_width = "64")))]
    pub fn isize(&mut self, mut n: isize) {
        self.i128(n as i128);
    }
    #[cfg(all(not(target_pointer_width = "32"), not(target_pointer_width = "64")))]
    pub fn usize(&mut self, mut n: usize) {
        self.u128(n as u128);
    }
    pub fn f64(&mut self, f: f64) {
        let mut buffer = ryu::Buffer::new();
        let printed = buffer.format(f);
        self.bytes(printed.as_bytes());
    }
}

pub trait Print<T> {
    fn print(&mut self, x: T);
    fn println(&mut self, x: T);
}

impl<const N: usize> Print<&[u8]> for Writer<N> {
    fn print(&mut self, x: &[u8]) {
        self.bytes(x);
    }
    fn println(&mut self, x: &[u8]) {
        self.bytes(x);
        self.byte(b'\n');
    }
}

impl<const N: usize, const M: usize> Print<&[u8; M]> for Writer<N> {
    fn print(&mut self, x: &[u8; M]) {
        self.bytes(x);
    }
    fn println(&mut self, x: &[u8; M]) {
        self.bytes(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<&str> for Writer<N> {
    fn print(&mut self, x: &str) {
        self.bytes(x.as_bytes());
    }
    fn println(&mut self, x: &str) {
        self.bytes(x.as_bytes());
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<i16> for Writer<N> {
    fn print(&mut self, x: i16) {
        self.i16(x);
    }
    fn println(&mut self, x: i16) {
        self.i16(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<u16> for Writer<N> {
    fn print(&mut self, x: u16) {
        self.u16(x);
    }
    fn println(&mut self, x: u16) {
        self.u16(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<i32> for Writer<N> {
    fn print(&mut self, x: i32) {
        self.i32(x);
    }
    fn println(&mut self, x: i32) {
        self.i32(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<u32> for Writer<N> {
    fn print(&mut self, x: u32) {
        self.u32(x);
    }
    fn println(&mut self, x: u32) {
        self.u32(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<i64> for Writer<N> {
    fn print(&mut self, x: i64) {
        self.i64(x);
    }
    fn println(&mut self, x: i64) {
        self.i64(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<u64> for Writer<N> {
    fn print(&mut self, x: u64) {
        self.u64(x);
    }
    fn println(&mut self, x: u64) {
        self.u64(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<isize> for Writer<N> {
    fn print(&mut self, x: isize) {
        self.isize(x);
    }
    fn println(&mut self, x: isize) {
        self.isize(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<usize> for Writer<N> {
    fn print(&mut self, x: usize) {
        self.usize(x);
    }
    fn println(&mut self, x: usize) {
        self.usize(x);
        self.byte(b'\n');
    }
}

impl<const N: usize> Print<f64> for Writer<N> {
    fn print(&mut self, x: f64) {
        self.f64(x);
    }
    fn println(&mut self, x: f64) {
        self.f64(x);
        self.byte(b'\n');
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_numbers_without_flush() {
        clear_stdout();
        let mut writer = Writer::<100>::new();

        writer.write_usize(10);
        writer.write_usize(20);
        assert_eq!(get_stdout_content(), b""); // not flushed yet
    }

    #[test]
    fn write_numbers_with_explicit_flush() {
        clear_stdout();
        let mut writer = Writer::<100>::new();

        writer.write_usize(10);
        writer.write_usize(20);
        writer.flush();
        assert_eq!(get_stdout_content(), b"1020");
    }

    #[test]
    fn write_numbers_implicit_flush() {
        clear_stdout();
        let mut writer = Writer::<4>::new();

        writer.write_usize(10);
        writer.write_usize(20);
        writer.write_usize(3);
        assert_eq!(get_stdout_content(), b"1020");
    }

    #[test]
    fn write_f64() {
        clear_stdout();
        let mut writer = Writer::<100>::new();

        writer.write_f64(1.23);
        writer.write_f64(-0.001);
        writer.flush();
        assert_eq!(get_stdout_content(), b"1.23-0.001");
    }

    #[test]
    fn print() {
        clear_stdout();
        let mut writer = Writer::<100>::new();
        writer.print(123usize);
        writer.print(" ");
        writer.print(45i32);
        writer.print(b" ");
        writer.print(78.9_f64);
        writer.flush();
        assert_eq!(get_stdout_content(), b"123 45 78.9");
    }

    #[test]
    fn println() {
        clear_stdout();
        let mut writer = Writer::<100>::new();
        writer.println(123usize);
        writer.println(45i32);
        writer.println(78.9_f64);
        writer.println("str");
        writer.println(b"bytes");
        writer.flush();
        assert_eq!(get_stdout_content(), b"123\n45\n78.9\nstr\nbytes\n");
    }
}
*/