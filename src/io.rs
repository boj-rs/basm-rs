use core::mem::MaybeUninit;

use crate::syscall;

#[cfg(not(feature = "slow-io"))]
pub use fast::*;
#[cfg(feature = "slow-io")]
pub use slow::*;

#[cfg(not(feature = "slow-io"))]
mod fast;
#[cfg(feature = "slow-io")]
mod slow;

pub struct Reader<const N: usize>(pub [MaybeUninit<u8>; N], pub usize, pub usize);
pub struct Writer<const N: usize>(pub [MaybeUninit<u8>; N], pub usize);

impl<const N: usize> Writer<N> {
    pub fn new() -> Self {
        Self(MaybeUninit::uninit_array(), 0)
    }
    #[inline(always)]
    pub fn write(&mut self, mut buf: &[u8]) {
        while self.1 + buf.len() > N {
            let len = N - self.1;
            let (current, next) = buf.split_at(len);
            buf = next;
            for (i, &b) in current.iter().enumerate() {
                self.0[self.1 + i].write(b);
            }
            self.1 = N;
            self.flush();
        }
        for (i, &b) in buf.iter().enumerate() {
            self.0[self.1 + i].write(b);
        }
        self.1 += buf.len();
    }
    #[inline(always)]
    pub fn flush(&mut self) {
        syscall::write(1, unsafe {
            MaybeUninit::slice_assume_init_ref(&self.0[..self.1])
        });
        self.1 = 0;
    }
    #[inline(always)]
    pub fn write_f64(&mut self, mut f: f64) {
        // integer part
        if f < 0.0 {
            self.write(b"-");
            f = -f;
        }
        let mut n = f as usize;
        self.write_usize(n);

        // fractional part
        let frac = f - (n as f64);
        if frac == 0.0 {
            return;
        }
        let mut buf = [b'0'; 11];
        buf[0] = b'.';
        let mut i = buf.len();
        n = (frac * 10_000_000_000.0) as usize;
        while n > 0 {
            i -= 1;
            buf[i] = (n % 10) as u8 + b'0';
            n /= 10;
        }

        // remove trailing zeros
        let mut len = buf.len();
        while len > 0 && buf[len - 1] == b'0' {
            len -= 1;
        }
        if len > 1 {
            self.write(&buf[..len]);
        }
    }
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

impl<const N: usize> Reader<N> {
    #[inline(always)]
    pub fn new() -> Self {
        Self(MaybeUninit::uninit_array(), 0, 0)
    }
    #[inline(always)]
    fn peek(&mut self) -> u8 {
        if self.2 >= self.1 {
            self.fill();
        }
        unsafe { self.0.get_unchecked(self.2).assume_init_read() }
    }
    #[inline(always)]
    pub fn fill(&mut self) {
        self.1 = syscall::read(0, unsafe {
            MaybeUninit::slice_assume_init_mut(&mut self.0)
        }) as usize;
        self.2 = 0;
    }
    #[inline(always)]
    pub fn next_i64(&mut self) -> i64 {
        if self.peek() == b'-' {
            self.2 += 1;
            -(self.next_usize() as i64)
        } else {
            self.next_usize() as i64
        }
    }
    #[inline(always)]
    pub fn next_i32(&mut self) -> i32 {
        if self.peek() == b'-' {
            self.2 += 1;
            -(self.next_usize() as i32)
        } else {
            self.next_usize() as i32
        }
    }
    #[inline(always)]
    pub fn next_usize(&mut self) -> usize {
        let mut n = 0;
        loop {
            let b = self.peek();
            self.2 += 1;
            if b > 32 {
                n *= 10;
                n += b as usize & 0x0F;
            } else {
                break;
            }
        }
        n
    }
    #[inline(always)]
    pub fn skip_white(&mut self) -> usize {
        let mut skip = 0;
        loop {
            if self.peek() <= 32 {
                self.2 += 1;
                skip += 1;
            } else {
                break skip;
            }
        }
    }
    #[inline(always)]
    pub fn skip_until(&mut self, delim: u8) -> usize {
        let mut skip = 0;
        while self.peek() != delim {
            self.2 += 1;
            skip += 1;
        }
        self.2 += 1;
        skip
    }
    #[inline(always)]
    pub fn next_word(&mut self, buf: &mut [u8]) -> usize {
        let mut i = 0;
        loop {
            let b = self.peek();
            self.2 += 1;
            if b <= 32 {
                break i;
            } else {
                buf[i] = b;
                i += 1;
            }
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        let mut buf: [MaybeUninit<u8>; 40] = MaybeUninit::uninit_array();
        let buf = unsafe { MaybeUninit::slice_assume_init_mut(&mut buf) };
        let n = self.next_word(buf);
        let mut int: usize = 0;
        let mut i = 0;
        let sign = if buf[0] == b'-' {
            i += 1;
            -1.0
        } else {
            1.0
        };
        while i < n && matches!(buf[i], b'0'..=b'9') {
            int = int * 10 + (buf[i] - b'0') as usize;
            i += 1;
        }
        if i == n {
            return sign * int as f64;
        }
        // assert_eq!(buf[i], b'.');
        if buf[i] == b'.' {
            i += 1;
        }
        let mut d = 1;
        let mut frac = 0;
        while i < n {
            frac = frac * 10 + (buf[i] - b'0') as usize;
            d *= 10;
            i += 1;
        }
        sign * (int as f64 + frac as f64 / d as f64)
    }

    #[inline(always)]
    pub fn next_until(&mut self, buf: &mut [u8], delim: u8) -> usize {
        let mut i = 0;
        loop {
            let b = self.peek();
            self.2 += 1;
            if b == delim {
                break i;
            } else {
                buf[i] = b;
                i += 1;
            }
        }
    }
    #[inline(always)]
    pub fn iter_i32(&mut self) -> I32Iterator<N> {
        I32Iterator { inner: self }
    }
    #[inline(always)]
    pub fn iter_i64(&mut self) -> I64Iterator<N> {
        I64Iterator { inner: self }
    }
    #[inline(always)]
    pub fn iter_usize(&mut self) -> UsizeIterator<N> {
        UsizeIterator { inner: self }
    }
}

impl<const N: usize> Default for Reader<N> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct I32Iterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for I32Iterator<'a, N> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_i32())
    }
}

pub struct I64Iterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for I64Iterator<'a, N> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_i64())
    }
}

pub struct UsizeIterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for UsizeIterator<'a, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_usize())
    }
}

pub trait Print<T> {
    fn print(&mut self, x: T);
    fn println(&mut self, x: T);
}

impl<const N: usize> Print<&[u8]> for Writer<N> {
    fn print(&mut self, x: &[u8]) {
        self.write(x);
    }
    fn println(&mut self, x: &[u8]) {
        self.write(x);
        self.write(b"\n");
    }
}

impl<const N: usize, const M: usize> Print<&[u8; M]> for Writer<N> {
    fn print(&mut self, x: &[u8; M]) {
        self.write(x);
    }
    fn println(&mut self, x: &[u8; M]) {
        self.write(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<&str> for Writer<N> {
    fn print(&mut self, x: &str) {
        self.write(x.as_bytes());
    }
    fn println(&mut self, x: &str) {
        self.write(x.as_bytes());
        self.write(b"\n");
    }
}

impl<const N: usize> Print<i32> for Writer<N> {
    fn print(&mut self, x: i32) {
        self.write_i32(x);
    }
    fn println(&mut self, x: i32) {
        self.write_i32(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<i64> for Writer<N> {
    fn print(&mut self, x: i64) {
        self.write_i64(x);
    }
    fn println(&mut self, x: i64) {
        self.write_i64(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<usize> for Writer<N> {
    fn print(&mut self, x: usize) {
        self.write_usize(x);
    }
    fn println(&mut self, x: usize) {
        self.write_usize(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<f64> for Writer<N> {
    fn print(&mut self, x: f64) {
        self.write_f64(x);
    }
    fn println(&mut self, x: f64) {
        self.write_f64(x);
        self.write(b"\n");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syscall::dummy::{clear_stdout, get_stdout_content, prepare_stdin};

    #[test]
    fn read_numbers() {
        prepare_stdin(b"1234 -56\n-9999.9999\n");
        let mut reader = Reader::<100>::new();

        assert_eq!(reader.next_usize(), 1234);
        assert_eq!(reader.next_i32(), -56);
        assert_eq!(reader.next_f64(), -9999.9999);
    }

    #[test]
    #[ignore]
    fn read_scientifi_notation() {
        prepare_stdin(b"1e1\n1e-1\n");
        let mut reader = Reader::<100>::new();

        assert_eq!(reader.next_f64(), 10.0);
        assert_eq!(reader.next_f64(), 1e-1);
    }

    #[test]
    fn read_word() {
        prepare_stdin(b"Hello World\nBye\n");
        let mut reader = Reader::<100>::new();
        let mut buf = [0; 100];

        let n = reader.next_word(&mut buf);
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"Hello");

        let n = reader.next_word(&mut buf);
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"World");

        let n = reader.next_word(&mut buf);
        assert_eq!(n, 3);
        assert_eq!(&buf[..n], b"Bye");
    }

    #[test]
    fn next_until() {
        prepare_stdin(b"Hello World\nBye\n");
        let mut reader = Reader::<100>::new();
        let mut buf = [0; 100];

        let n = reader.next_until(&mut buf, b'\n');
        assert_eq!(n, 11);
        assert_eq!(&buf[..n], b"Hello World");

        let n = reader.next_word(&mut buf);
        assert_eq!(n, 3);
        assert_eq!(&buf[..n], b"Bye");
    }

    #[test]
    #[ignore]
    fn read_word_without_terminator() {
        prepare_stdin(b"no-terminator");
        let mut reader = Reader::<100>::new();
        let mut buf = [0; 100];

        let n = reader.next_word(&mut buf);
        assert_eq!(n, 13);
        assert_eq!(&buf[..n], b"no-terminator");
    }

    #[test]
    #[ignore]
    fn read_word_multiple_space_in_between() {
        // This also affects number reading.
        prepare_stdin(b"1 \n5"); // Trailing space in first line
        let mut reader = Reader::<100>::new();
        let mut buf = [0; 100];

        let n = reader.next_word(&mut buf);
        assert_eq!(n, 1);

        let n = reader.next_word(&mut buf);
        assert_eq!(n, 1);
        assert_eq!(&buf[..n], b"b");
    }

    #[test]
    fn skip_white() {
        prepare_stdin(b" \t\x0b\n5\n");
        let mut reader = Reader::<100>::new();
        assert_eq!(reader.skip_white(), 4);
        assert_eq!(reader.next_usize(), 5);
    }

    #[test]
    fn skip_until() {
        prepare_stdin(b"garbage,5\n");
        let mut reader = Reader::<100>::new();
        assert_eq!(reader.skip_until(b','), b"garbage".len());
        assert_eq!(reader.next_usize(), 5);
    }

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
