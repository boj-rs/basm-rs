use super::Nonwhite;
use crate::platform::services;
use alloc::string::{String, ToString};
use core::fmt::Arguments;
use core::mem::MaybeUninit;

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

#[cfg(any(not(feature = "short"), feature = "fastio"))]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
unsafe fn cvt8(n: u32) -> (u64, usize) {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
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
    let mask = _mm_movemask_epi8(_mm_cmpgt_epi8(t6, _mm_setzero_si128()));
    let offset = (mask | 0x8000).trailing_zeros() as usize;
    let ascii = _mm_add_epi8(t6, ascii0);
    let value = _mm_extract_epi64::<1>(ascii) as u64;
    let len = 16 - offset;
    (value, len)
}

impl<const N: usize> Writer<N> {
    const DUMMY: () = assert!(
        N >= super::MIN_BUF_SIZE,
        "Buffer size for Writer must be at least MIN_BUF_SIZE"
    );
    /// Constructs a new `Writer` with buffer size `N`, specified as a const generic parameter.
    /// Note: For convenience, use `Default::default()`.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer = Writer::<128>::new();
    /// let mut writer: Writer = Default::default();
    /// ```
    pub fn new() -> Self {
        #[allow(clippy::let_unit_value)]
        let _ = Self::DUMMY;
        Self {
            buf: [const { MaybeUninit::uninit() }; N],
            off: 0,
        }
    }
    /// Flushes the buffer of the current `Writer`.
    pub fn flush(&mut self) {
        services::write_stdio(1, unsafe { self.buf[..self.off].assume_init_ref() });
        self.off = 0;
    }
    /// Flushes the buffer of the current `Writer` if readahead plus the current offset exceeds the buffer length,
    /// thereby ensuring that at least `readahead` bytes are available in the buffer.
    pub fn try_flush(&mut self, readahead: usize) {
        if self.off + readahead > self.buf.len() {
            self.flush();
        }
    }
    // Call this function instead of `byte` if it is known for sure that
    // the buffer is not full. For example, this function is called in
    // println() after calling respective print routines, which ensure an
    // extra space for one byte in the buffer to allow for safe use of
    // this function.
    fn byte_unchecked(&mut self, b: u8) {
        // SAFETY: we ensure self.off < self.buf.len() before calling this function
        unsafe {
            self.buf
                .as_mut_ptr()
                .wrapping_add(self.off)
                .write(MaybeUninit::<u8>::new(b));
        }
        self.off += 1;
    }
    /// Writes a single byte to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.byte(b'c'); // c
    /// ```
    pub fn byte(&mut self, b: u8) {
        self.try_flush(2);
        self.byte_unchecked(b);
    }
    /// Writes multiple bytes to standard output.
    ///
    /// This method ensures an extra byte in the buffer to make sure that `println()` can safely use the private method `byte_unchecked`. It is achieved by `self.try_flush(1)`.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.bytes("Hello World".as_bytes()); // Hello World
    /// ```
    #[cfg(any(not(feature = "short"), feature = "fastio"))]
    pub fn bytes(&mut self, mut s: &[u8]) {
        while !s.is_empty() {
            let rem = s.len().min(self.buf[self.off..].len());
            unsafe {
                self.buf[self.off..self.off + rem]
                    .assume_init_mut()
                    .copy_from_slice(&s[..rem]);
            }
            self.off += rem;
            s = &s[rem..];
            self.try_flush(1);
        }
    }
    /// Writes multiple bytes to standard output.
    ///
    /// This function ensures an extra byte in the buffer to make sure that
    /// println() can safely use `byte_unchecked`. This is achieved by
    /// calling `self.try_flush(2)` (instead of `self.try_flush(1)`) in byte().
    #[cfg(all(feature = "short", not(feature = "fastio")))]
    pub fn bytes(&mut self, s: &[u8]) {
        for x in s {
            self.byte(*x);
        }
    }
    /// Writes a single `&str` to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.str("Hello, World"); // Hello, World
    /// ```
    pub fn str(&mut self, s: &str) {
        self.bytes(s.as_bytes());
    }
    /// Writes a single `i8` to standard output.
    /// ```no run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.i8(i8::MIN); // -128
    /// ```
    pub fn i8(&mut self, n: i8) {
        self.i32(n as i32);
    }
    /// Writes a single `u8` to standard output.
    /// ```no run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.u8(u8::MAX); // 255
    /// ```
    pub fn u8(&mut self, n: u8) {
        self.u32(n as u32);
    }
    /// Writes a single `i16` to standard output.
    /// ```no run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.i16(i16::MIN); // -32768
    /// ```
    pub fn i16(&mut self, n: i16) {
        self.i32(n as i32);
    }
    /// Writes a single `u16` to standard output.
    /// ```no run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.u16(u16::MAX); // 65535
    /// ```
    pub fn u16(&mut self, n: u16) {
        self.u32(n as u32);
    }
    /// Writes a single `i32` to standard output.
    /// ```no run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.i32(i32::MIN); // -2147483648
    /// ```
    pub fn i32(&mut self, n: i32) {
        if n < 0 {
            self.byte(b'-');
            self.u32((n as u32).wrapping_neg());
        } else {
            self.u32(n as u32);
        }
    }
    /// Writes a single `u32` to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.u32(u32::MAX); // 4294967295
    /// ```
    #[cfg(any(not(feature = "short"), feature = "fastio"))]
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub fn u32(&mut self, n: u32) {
        self.try_flush(11 + 8);
        let mut p;
        if n < 100_000_000 {
            p = unsafe { cvt8(n) };
            p.0 >>= (8 - p.1) << 3;
        } else {
            let mut hi = n / 100_000_000;
            let lo = n % 100_000_000;
            p = (unsafe { cvt8(lo).0 }, 8);
            let mut len = 0;
            while hi > 0 {
                self.buf[self.off].write((hi % 10) as u8 + b'0');
                hi /= 10;
                self.off += 1;
                len += 1;
            }
            let mut i = 0;
            while 2 * i + 1 < len {
                self.buf.swap(self.off - len + i, self.off - i - 1);
                i += 1;
            }
        }
        unsafe {
            core::ptr::write_unaligned(
                self.buf.as_mut_ptr().wrapping_add(self.off) as *mut u64,
                p.0,
            );
        }
        self.off += p.1;
    }
    #[cfg(any(
        all(feature = "short", not(feature = "fastio")),
        not(any(target_arch = "x86_64", target_arch = "x86"))
    ))]
    pub fn u32(&mut self, n: u32) {
        self.u64(n as u64)
    }
    /// Writes a single `i64` to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.i64(i64::MIN); // -9223372036854775808
    /// ```
    #[cfg(any(not(feature = "short"), feature = "fastio"))]
    pub fn i64(&mut self, n: i64) {
        if n < 0 {
            self.byte(b'-');
            self.u64((n as u64).wrapping_neg());
        } else {
            self.u64(n as u64);
        }
    }
    #[cfg(all(feature = "short", not(feature = "fastio")))]
    pub fn i64(&mut self, n: i64) {
        self.try_flush(22);
        let mut n = if n < 0 {
            self.byte_unchecked(b'-');
            (n as u64).wrapping_neg()
        } else {
            n as u64
        };
        let mut buf = [MaybeUninit::<u8>::uninit(); 21];
        let mut i = 0;
        loop {
            buf[i].write(b'0' + (n % 10) as u8);
            n /= 10;
            i += 1;
            if n == 0 {
                break;
            }
        }
        for j in (0..i).rev() {
            self.buf[self.off].write(unsafe { buf[j].assume_init() });
            self.off += 1;
        }
    }
    /// Writes a single `u64` to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.u64(u64::MAX); // 18446744073709551615
    /// ```
    #[cfg(any(not(feature = "short"), feature = "fastio"))]
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub fn u64(&mut self, n: u64) {
        self.try_flush(21 + 8);
        let mut plo;
        if n < 100_000_000 {
            // For small n, we avoid the cost of zero-initializing hi128.
            // This provides a significant gain in performance.
            plo = unsafe { cvt8(n as u32) };
            plo.0 >>= (8 - plo.1) << 3;
        } else {
            let mut phi;
            if n < 10_000_000_000_000_000 {
                let hi = (n / 100_000_000) as u32;
                let lo = (n % 100_000_000) as u32;
                phi = unsafe { cvt8(hi) };
                phi.0 >>= (8 - phi.1) << 3;
                plo = (unsafe { cvt8(lo).0 }, 8);
            } else {
                let mut hi = (n / 10_000_000_000_000_000) as u32;
                let lo = n % 10_000_000_000_000_000;
                let lohi = (lo / 100_000_000) as u32;
                let lolo = (lo % 100_000_000) as u32;
                phi = (unsafe { cvt8(lohi).0 }, 8);
                plo = (unsafe { cvt8(lolo).0 }, 8);
                let mut len = 0;
                while hi > 0 {
                    self.buf[self.off].write((hi % 10) as u8 + b'0');
                    hi /= 10;
                    self.off += 1;
                    len += 1;
                }
                let mut i = 0;
                while 2 * i + 1 < len {
                    self.buf.swap(self.off - len + i, self.off - i - 1);
                    i += 1;
                }
            }
            unsafe {
                core::ptr::write_unaligned(
                    self.buf.as_mut_ptr().wrapping_add(self.off) as *mut u64,
                    phi.0,
                );
            }
            self.off += phi.1;
        }
        unsafe {
            core::ptr::write_unaligned(
                self.buf.as_mut_ptr().wrapping_add(self.off) as *mut u64,
                plo.0,
            );
        }
        self.off += plo.1;
    }
    #[cfg(any(
        all(feature = "short", not(feature = "fastio")),
        not(any(target_arch = "x86_64", target_arch = "x86"))
    ))]
    pub fn u64(&mut self, mut n: u64) {
        self.try_flush(21);
        let mut buf = [MaybeUninit::<u8>::uninit(); 21];
        let mut i = 0;
        loop {
            buf[i].write(b'0' + (n % 10) as u8);
            n /= 10;
            i += 1;
            if n == 0 {
                break;
            }
        }
        for j in (0..i).rev() {
            self.buf[self.off].write(unsafe { buf[j].assume_init() });
            self.off += 1;
        }
    }
    /// Writes a single `i128` to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.i128(i128::MIN); // -170141183460469231731687303715884105728
    /// ```
    pub fn i128(&mut self, n: i128) {
        if n < 0 {
            self.byte(b'-');
            self.u128((n as u128).wrapping_neg());
        } else {
            self.u128(n as u128);
        }
    }
    /// Writes a single `u128` to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.u128(u128::MAX); // 340282366920938463463374607431768211455
    /// ```
    pub fn u128(&mut self, mut n: u128) {
        let mut buf = [const { MaybeUninit::<u8>::uninit() }; 40];
        let mut offset = buf.len() - 1;
        buf[offset].write(b'0' + (n % 10) as u8);
        n /= 10;
        while n > 0 {
            offset -= 1;
            buf[offset].write(b'0' + (n % 10) as u8);
            n /= 10;
        }
        self.bytes(unsafe { buf[offset..].assume_init_ref() });
    }
    #[cfg(target_pointer_width = "32")]
    pub fn isize(&mut self, n: isize) {
        self.i32(n as i32);
    }
    #[cfg(target_pointer_width = "32")]
    pub fn usize(&mut self, n: usize) {
        self.u32(n as u32);
    }
    /// Writes a single `isize` to standard output. Note that the in-memory size of `isize` depends on the target platform.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.isize(isize::MIN); // -9223372036854775808 (On 64-bit targets)
    /// ```
    #[cfg(target_pointer_width = "64")]
    pub fn isize(&mut self, n: isize) {
        self.i64(n as i64);
    }
    /// Writes a single `usize` to standard output. Note that the in-memory size of `isize` depends on the target platform.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.usize(usize::MAX); // 18446744073709551615 (On 64-bit targets)
    /// ```
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
    /// Writes a single `f64` to standard output.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.f64(1.23); // 1.23
    /// ```
    pub fn f64(&mut self, f: f64) {
        let mut buffer = ryu::Buffer::new();
        let printed = buffer.format(f);
        self.bytes(printed.as_bytes());
    }
    /// Writes a single `char` to standard output, encoded as UTF-8.
    /// ```no_run
    /// use basm_std::platform::io::Writer;
    /// let mut writer: Writer = Default::default();
    /// writer.char('c'); // c
    /// ```
    pub fn char(&mut self, c: char) {
        self.try_flush(6);
        let u = c as u32;
        if u < 0x80 {
            self.byte_unchecked(u as u8);
        } else if u < 0x800 {
            self.byte_unchecked(0b11000000 | (u >> 6) as u8);
            self.byte_unchecked(0b10000000 | (u & 0x3F) as u8);
        } else if u < 0xFFFF {
            self.byte_unchecked(0b11100000 | (u >> 12) as u8);
            self.byte_unchecked(0b10000000 | ((u >> 6) & 0x3F) as u8);
            self.byte_unchecked(0b10000000 | (u & 0x3F) as u8);
        } else {
            // The Unicode standard dictates that every codepoint
            // that is not representable in UTF-16 (including the use of surrogates)
            // should be considered invalid. Hence, we put an assert here
            // and do not deal with the case of u > 0x10_FFFF.
            assert!(u <= 0x10_FFFF);
            self.byte_unchecked(0b11110000 | (u >> 18) as u8);
            self.byte_unchecked(0b10000000 | ((u >> 12) & 0x3F) as u8);
            self.byte_unchecked(0b10000000 | ((u >> 6) & 0x3F) as u8);
            self.byte_unchecked(0b10000000 | (u & 0x3F) as u8);
        }
    }
}

pub trait Print<T> {
    fn print(&mut self, x: T);
    fn println(&mut self, x: T);
}

/// Writes a single `Nonwhite` to standard output. Note that `print()` doesn't add a newline at the end of the output. If a newline is needed, use `println()`.
impl<const N: usize> Print<Nonwhite> for Writer<N> {
    fn print(&mut self, x: Nonwhite) {
        self.print(*x as char);
    }

    fn println(&mut self, x: Nonwhite) {
        self.println(*x as char);
    }
}

/// Writes a single `&Nonwhite` to standard output. Note that `print()` doesn't add a newline at the end of the output. If a newline is needed, use `println()`.
impl<const N: usize> Print<&Nonwhite> for Writer<N> {
    fn print(&mut self, x: &Nonwhite) {
        self.print(*x);
    }

    fn println(&mut self, x: &Nonwhite) {
        self.println(*x);
    }
}

/// Writes a single `&[u8]` using [`Writer::bytes()`] to standard output. Note that `print()` doesn't add a newline at the end of the output. If a newline is needed, use `println()`.
impl<const N: usize> Print<&[u8]> for Writer<N> {
    fn print(&mut self, x: &[u8]) {
        self.bytes(x);
    }
    fn println(&mut self, x: &[u8]) {
        self.bytes(x);
        self.byte_unchecked(b'\n');
    }
}

/// Writes a single `&[u8; M]` using [`Writer::bytes()`] to standard output. Note that `print()` doesn't add a newline at the end of the output. If a newline is needed, use `println()`.
impl<const N: usize, const M: usize> Print<&[u8; M]> for Writer<N> {
    fn print(&mut self, x: &[u8; M]) {
        self.bytes(x);
    }
    fn println(&mut self, x: &[u8; M]) {
        self.bytes(x);
        self.byte_unchecked(b'\n');
    }
}

/// Writes a single `&str` using [`Writer::bytes()`] and [`str::as_bytes()`] to standard output. Note that `print()` doesn't add a newline at the end of the output. If a newline is needed, use `println()`.
impl<const N: usize> Print<&str> for Writer<N> {
    fn print(&mut self, x: &str) {
        self.bytes(x.as_bytes());
    }
    fn println(&mut self, x: &str) {
        self.bytes(x.as_bytes());
        self.byte_unchecked(b'\n');
    }
}

/// Write a single `String`` using [`Writer::Print<&str>()`] and [`String::as_str()`] to standard output. Note that `print()` doesn't add a newline at the end of the output. If a newline is needed, use `println()`.
impl<const N: usize> Print<String> for Writer<N> {
    fn print(&mut self, x: String) {
        self.print(x.as_str());
    }
    fn println(&mut self, x: String) {
        self.println(x.as_str());
    }
}

/// Writes a single `&String` using `Writer::Print<&str>()` and [`String::as_str()`] to standard output. Note that `print()` doesn't add a newline at the end of the output. If a newline is needed, use `println()`.
impl<const N: usize> Print<&String> for Writer<N> {
    fn print(&mut self, x: &String) {
        self.print(x.as_str());
    }
    fn println(&mut self, x: &String) {
        self.println(x.as_str());
    }
}

macro_rules! impl_print{
    ($($ty:ident)*) => {
        $(
            impl<const N: usize> Print<$ty> for Writer<N> {
                fn print(&mut self, x: $ty) {
                    self.$ty(x);
                }
                fn println(&mut self, x: $ty) {
                    self.$ty(x);
                    self.byte_unchecked(b'\n');
                }
            }
            impl<const N: usize> Print<&$ty> for Writer<N> {
                fn print(&mut self, x: &$ty) {
                    self.$ty(*x);
                }
                fn println(&mut self, x: &$ty) {
                    self.$ty(*x);
                    self.byte_unchecked(b'\n');
                }
            }
            impl<const N: usize> Print<&mut $ty> for Writer<N> {
                fn print(&mut self, x: &mut $ty) {
                    self.$ty(*x);
                }
                fn println(&mut self, x: &mut $ty) {
                    self.$ty(*x);
                    self.byte_unchecked(b'\n');
                }
            }
        )*
    }
}

impl_print!(i8 u8 i16 u16 i32 u32 i64 u64 f64 i128 u128 isize usize char);

impl<'a, const N: usize> Print<Arguments<'a>> for Writer<N> {
    fn print(&mut self, x: Arguments<'a>) {
        if let Some(s) = x.as_str() {
            self.print(s);
        } else {
            self.print(x.to_string());
        }
    }

    fn println(&mut self, x: Arguments<'a>) {
        if let Some(s) = x.as_str() {
            self.println(s);
        } else {
            self.println(x.to_string());
        }
    }
}

#[macro_export]
macro_rules! bprint {
    ($writer:tt, $($arg:tt)*) => {{
        use $crate::platform::io::Print;
        $writer.print(core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! bprintln {
    ($writer:tt, $($arg:tt)*) => {{
        use $crate::platform::io::Print;
        $writer.println(core::format_args!($($arg)*));
    }};
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
