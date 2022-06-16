use alloc::{vec, vec::Vec};

pub trait SegmentOp {
    type T: Clone;
    type U;
    fn e() -> Self::T;
    fn combine(l: &Self::T, r: &Self::T) -> Self::T;
    fn apply(v: &mut Self::T, u: &Self::U);
}

pub struct SegmentTree<Op: SegmentOp> {
    v: Vec<Op::T>,
    n: usize,
}

impl<Op: SegmentOp> SegmentTree<Op> {
    pub fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        let v = vec![Op::e(); n * 2];
        Self { v, n }
    }

    pub fn from_iter<I>(n: usize, iter: I) -> Self
    where
        I: IntoIterator<Item = Op::T>,
    {
        let off = n.next_power_of_two();
        let mut v = Vec::with_capacity(off * 2);
        unsafe { v.set_len(off) };
        v.extend(iter.into_iter().take(n));
        v.resize(off * 2, Op::e());
        for i in (0..off).rev() {
            v[i] = Op::combine(&v[i * 2], &v[i * 2 + 1]);
        }
        Self { v, n: off }
    }

    pub fn query<B>(&self, range: B) -> Op::T
    where
        B: core::ops::RangeBounds<usize>,
    {
        use core::ops::Bound::*;
        let mut l = self.n
            + match range.start_bound() {
                Included(&x) => x,
                Excluded(&x) => x + 1,
                Unbounded => 0,
            };
        let mut r = self.n
            + match range.end_bound() {
                Included(&x) => x + 1,
                Excluded(&x) => x,
                Unbounded => self.n,
            };
        let mut lsum = Op::e();
        let mut rsum = Op::e();
        while l < r {
            if l & 1 != 0 {
                lsum = Op::combine(&lsum, &self.v[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                rsum = Op::combine(&self.v[r], &rsum);
            }
            l >>= 1;
            r >>= 1;
        }
        Op::combine(&lsum, &rsum)
    }

    pub fn update<U>(&mut self, mut i: usize, u: U)
    where
        U: core::borrow::Borrow<Op::U>,
    {
        i += self.n;
        Op::apply(&mut self.v[i], u.borrow());
        while i > 1 {
            i >>= 1;
            self.v[i] = Op::combine(&self.v[i * 2], &self.v[i * 2 + 1]);
        }
    }

    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: Fn(&Op::T) -> bool,
    {
        let mut p = 1;
        if pred(&self.v[p]) {
            self.n
        } else {
            let mut pivot = Op::e();
            while p < self.n {
                p *= 2;
                let test = Op::combine(&pivot, &self.v[p]);
                if pred(&test) {
                    pivot = test;
                    p |= 1;
                }
            }
            p - self.n
        }
    }
}
