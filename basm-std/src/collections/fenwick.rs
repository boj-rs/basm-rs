use alloc::{vec, vec::Vec};

pub trait FenwickOp {
    type T: Clone;
    type U;
    fn e() -> Self::T;
    fn combine(l: &Self::T, r: &Self::T) -> Self::T;
    fn apply(v: &mut Self::T, u: &Self::U);
}

pub struct FenwickTree<Op: FenwickOp> {
    v: Vec<Op::T>,
}

impl<Op: FenwickOp> FenwickTree<Op> {
    pub fn new(n: usize) -> Self {
        Self {
            v: vec![Op::e(); n],
        }
    }

    pub fn update<U>(&mut self, mut i: usize, u: U)
    where
        U: core::borrow::Borrow<Op::U>,
    {
        while i < self.v.len() {
            Op::apply(&mut self.v[i], u.borrow());
            i |= i + 1;
        }
    }

    pub fn query(&self, count: usize) -> Op::T {
        let mut result = Op::e();
        let mut i = count;
        while i > 0 {
            result = Op::combine(&result, &self.v[i - 1]);
            i &= i - 1;
        }
        result
    }
}

impl<Op: FenwickOp> FromIterator<Op::T> for FenwickTree<Op> {
    fn from_iter<T: IntoIterator<Item = Op::T>>(iter: T) -> Self {
        let mut v: Vec<Op::T> = iter.into_iter().collect();
        for i in 0..v.len() {
            let k = i | (i + 1);
            if k < v.len() {
                v[k] = Op::combine(&v[i], &v[k]);
            }
        }
        Self { v }
    }
}
