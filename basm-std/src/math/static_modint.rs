// Currently, no work has been done for preventing correctness issues from overflowing and
// performance problems. This should only be considered as a sort of skeleton code.

use core::{fmt::Display, ops::*};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ModInt<const M: u64>(u64);

impl<const M: u64> ModInt<M> {
    pub fn get(self) -> u64 {
        self.0
    }
}

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

// TODO: Handle the case for `self.0 + rhs.0` overflow (the implementation below is not enough)
impl<const M: u64> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (val, carry) = self.0.overflowing_add(rhs.0);
        if carry {
            let v: u64 = (1u64 << (u64::BITS - 1)) % M;
            Self((val + (v >> 1)) % M)
        } else {
            Self(val % M)
        }
    }
}

// TODO: Same with `Add`
impl<const M: u64> AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0) % M;
    }
}

// TODO: Try removing `i128` cast
impl<const M: u64> Div for ModInt<M> {
    type Output = Self;
    /// May panic if M == 0
    fn div(self, rhs: Self) -> Self::Output {
        let inv = super::egcd(rhs.0 as i128, M as i128).1;
        let inv = (M as i128 + inv) as u64 % M;
        Self(self.0) * Self(inv)
    }
}

// TODO: Same with `Div`
impl<const M: u64> DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        let inv = super::egcd(rhs.0 as i128, M as i128).1;
        let inv = (M as i128 + inv) as u64 % M;
        *self *= Self(inv);
    }
}

// TODO: Handle the case for `self.0 * rhs.0` overflow
impl<const M: u64> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % M)
    }
}

// TODO: Same with `Mul`
impl<const M: u64> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 * rhs.0) % M;
    }
}

// TODO: Check if there's a more performant way, doable without literal rem operation
impl<const M: u64> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self((M - self.0) % M)
    }
}

// TODO: This, or `self + (-rhs)`, which is more performant?
// TODO: Handle the case where `M + self.0` overflows
impl<const M: u64> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self((M + self.0 - rhs.0) % M)
    }
}

// TODO: This, or `self += -rhs`, which is more performant?
// TODO: Handle the case where `M + self.0` overflows
impl<const M: u64> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (M + self.0 - rhs.0) % M;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_static_small_mod_add() {
        let proper = |a: u64, b: u64, m: u64| ((a as u128 + b as u128) % (m as u128)) as u64;
        macro_rules! test {
            ($($m:expr),*) => {$(
                {
                    const M: u64 = $m;
                    for mut a in 0..100 {
                        if a > 50 {
                            a = a * 29 + 18;
                        }
                        let am = ModInt::<{ M }>::from(a);
                        for mut b in 0..100 {
                            if b > 50 {
                                b = b * 29 + 18;
                            }
                            let bm = ModInt::<{ M }>::from(b);
                            let t = am + bm;
                            assert_eq!(proper(a, b, M), t.0);
                        }
                    }
                }
            )*};
        }
        test!(2, 10, 593, 11729378, 2343246813781979);
    }

    #[test]
    fn test_static_large_mod_add() {
        let proper = |a: u64, b: u64, m: u64| ((a as u128 + b as u128) % (m as u128)) as u64;
        macro_rules! test {
            ($($m:expr),*) => {$(
                {
                    const M: u64 = $m;
                    let m = M / 3 * 2;
                    for a in [m, m.wrapping_add(1), m.wrapping_add(2), m.wrapping_add(30)] {
                        let am = ModInt::<{ M }>::from(a);
                        for b in [m, m.wrapping_add(1), m.wrapping_add(2), m.wrapping_add(30)] {
                            let bm = ModInt::<{ M }>::from(b);
                            let t = am + bm;
                            assert_eq!(proper(a, b, M), t.0);
                        }
                    }
                }
            )*};
        }
        test!(
            u64::MAX / 2,
            u64::MAX / 2 + 1,
            u64::MAX / 3 * 2 + 40,
            u64::MAX - 1,
            u64::MAX
        );
    }
}
