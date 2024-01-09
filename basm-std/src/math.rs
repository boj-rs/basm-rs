pub mod miller_rabin;
pub use miller_rabin::*;
mod sieve;
pub use sieve::LinearSieve;
mod pollard_rho;
pub use pollard_rho::factorize;

pub mod ntt;
pub use ntt::*;

// reference: https://nyaannyaan.github.io/library/trial/fast-gcd.hpp.html

use core::ops::*;

pub trait GcdOps:
    Copy
    + From<u8>
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + ShrAssign<u32>
{
    fn trailing_zeros(self) -> u32;
    fn wrapping_sub(self, rhs: Self) -> Self;
}

macro_rules! impl_gcd_ops {
    ($($t:ty),*) => { $(
        impl GcdOps for $t {
            fn trailing_zeros(self) -> u32 {
                self.trailing_zeros()
            }
            fn wrapping_sub(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }
        }
    )* };
}
impl_gcd_ops!(u8, u16, u32, u64, u128, usize);

/// Returns the greatest common divisor (GCD) of `a` and `b` if neither is zero, otherwise returns `a + b`.
pub fn gcd<T: GcdOps>(mut a: T, mut b: T) -> T {
    if a == 0.into() || b == 0.into() {
        a + b
    } else {
        let n = a.trailing_zeros();
        let m = b.trailing_zeros();
        a >>= n;
        b >>= m;
        while a != b {
            let m = a.wrapping_sub(b).trailing_zeros();
            let f = a > b;
            let c = if f { a } else { b };
            b = if f { b } else { a };
            a = (c - b) >> m;
        }
        a << n.min(m)
    }
}

/// Returns the least common multiplier (LCM) of `a` and `b` if neither is zero, otherwise returns `0`.
pub fn lcm<T: GcdOps + Mul<Output = T> + Div<Output = T>>(a: T, b: T) -> T {
    if a == 0.into() && b == 0.into() {
        0.into()
    } else {
        a / gcd(a, b) * b
    }
}

pub trait EgcdOps:
    Copy
    + From<i8>
    + PartialOrd
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}

macro_rules! impl_egcd_ops {
    ($($t:ty),*) => { $(
        impl EgcdOps for $t {}
    )* };
}
impl_egcd_ops!(i8, i16, i32, i64, i128, isize);

/// Returns `(g, x, y)` where `g` is the greatest common divisor (GCD), and `x`, `y` are integers such that `a*x + b*y = g`.
pub fn egcd<T: EgcdOps>(mut a: T, mut b: T) -> (T, T, T) {
    let mut c = if a > b {
        (a, b) = (b, a);
        [0, 1, 1, 0].map(|x| x.into())
    } else {
        [1, 0, 0, 1].map(|x| x.into())
    }; // treat as a row-major 2x2 matrix
    loop {
        if a == 0.into() {
            break (b, c[1], c[3]);
        }
        let (q, r) = (b / a, b % a);
        (a, b) = (r, a);
        c = [c[1] - q * c[0], c[0], c[3] - q * c[2], c[2]];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gcd_returns_gcd() {
        assert_eq!(32, gcd::<u32>(128736, 72352));
    }

    #[test]
    fn lcm_returns_lcm() {
        assert_eq!(249318024, lcm::<u32>(234984, 12732));
    }

    #[test]
    fn gcd_u64_returns_gcd() {
        assert_eq!(6, gcd::<u64>(2763162631554, 1276921782234));
    }

    #[test]
    fn lcm_u64_returns_lcm() {
        assert_eq!(4264971179382324, lcm::<u64>(273652348, 62341452));
    }

    #[test]
    fn egcd_returns_gcd() {
        let a: i128 = 823327498201749212;
        let b: i128 = 734892783927949214;
        let (g, s, t) = egcd(a, b);
        let normal = gcd(a as u64, b as u64) as i128;
        assert_eq!(normal, g);
        assert_eq!(a * s + b * t, g);
    }
}
