use crate::io::{Reader, Writer};

impl<const N: usize> Reader<N> {
    pub fn iter_long(&mut self) -> LongIterator<N> {
        LongIterator { inner: self }
    }
    pub fn iter_int(&mut self) -> IntIterator<N> {
        IntIterator { inner: self }
    }
    pub fn iter_uint(&mut self) -> UintIterator<N> {
        UintIterator { inner: self }
    }
}

pub struct LongIterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for LongIterator<'a, N> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_long())
    }
}

pub struct IntIterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for IntIterator<'a, N> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_int())
    }
}

pub struct UintIterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for UintIterator<'a, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_uint())
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
        self.write_int(x);
    }
    fn println(&mut self, x: i32) {
        self.write_int(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<i64> for Writer<N> {
    fn print(&mut self, x: i64) {
        self.write_long(x);
    }
    fn println(&mut self, x: i64) {
        self.write_long(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<usize> for Writer<N> {
    fn print(&mut self, x: usize) {
        self.write_uint(x);
    }
    fn println(&mut self, x: usize) {
        self.write_uint(x);
        self.write(b"\n");
    }
}
