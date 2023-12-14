use alloc::vec::Vec;

pub struct Kmp<'a, I, T> {
    pub pi: Vec<u32>,
    haystack: I,
    needle: &'a [T],
    i: usize,
}

impl<'a, I, T> Kmp<'a, I, T> {
    pub fn new<H>(haystack: H, needle: &'a [T], pi: Vec<u32>) -> Self
    where
        H: IntoIterator<IntoIter = I>,
    {
        Self {
            pi,
            haystack: haystack.into_iter(),
            needle,
            i: 0,
        }
    }
}

impl<I, T, B> Iterator for Kmp<'_, I, T>
where
    T: PartialEq,
    B: core::borrow::Borrow<T>,
    I: Iterator<Item = B>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.haystack.next()?;
        loop {
            if self.needle.get(self.i) == Some(c.borrow()) {
                self.i += 1;
                break;
            } else if self.i == 0 {
                break;
            } else {
                self.i = self.pi[self.i - 1] as usize;
            }
        }
        Some(self.i)
    }
}
