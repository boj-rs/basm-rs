use crate::io::Reader;

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
