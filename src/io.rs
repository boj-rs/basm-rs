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
            for i in 0..len {
                self.0[self.1 + i].write(current[i]);
            }
            self.1 = N;
            self.flush();
        }
        for i in 0..buf.len() {
            self.0[self.1 + i].write(buf[i]);
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
    pub fn skip_white(&mut self) {
        loop {
            if self.peek() <= 32 {
                self.2 += 1;
            } else {
                break;
            }
        }
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
