// Currently, no work has been done for preventing correctness issues from overflowing and
// performance problems. This should only be considered as a sort of skeleton code.

use core::{fmt::Display, ops::*};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ModInt<const M: u64>(pub u64);

impl<const M: u64> Display for ModInt<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<const M: u64> From<u64> for ModInt<M> {
    fn from(value: u64) -> Self {
        Self(value % M)
    }
}

impl<const M: u64> From<ModInt<M>> for u64 {
    fn from(value: ModInt<M>) -> Self {
        value.0
    }
}

impl<const M: u64> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % M)
    }
}

impl<const M: u64> AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0) % M;
    }
}

impl<const M: u64> Div for ModInt<M> {
    type Output = Self;
    /// May panic
    fn div(self, rhs: Self) -> Self::Output {
        let inv = super::egcd(rhs.0 as i128, M as i128).1;
        let inv = (M as i128 + inv) as u64 % M;
        Self(self.0) * Self(inv)
    }
}

impl<const M: u64> DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        let inv = super::egcd(rhs.0 as i128, M as i128).1;
        let inv = (M as i128 + inv) as u64 % M;
        *self *= Self(inv);
    }
}

impl<const M: u64> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % M)
    }
}

impl<const M: u64> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 * rhs.0) % M;
    }
}

impl<const M: u64> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self((M - self.0) % M)
    }
}

impl<const M: u64> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self((M + self.0 - rhs.0) % M)
    }
}

impl<const M: u64> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (M + self.0 - rhs.0) % M;
    }
}
