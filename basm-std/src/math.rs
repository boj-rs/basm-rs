pub mod miller_rabin;
pub use miller_rabin::*;
mod sieve;
pub use sieve::LinearSieve;
mod pollard_rho;
use crate::traits::{PrimSint, PrimUint};
pub use pollard_rho::factorize;

pub mod ntt;
pub use ntt::*;

// reference: https://nyaannyaan.github.io/library/trial/fast-gcd.hpp.html

pub fn gcd<T: PrimUint>(mut a: T, mut b: T) -> T {
    if a.is_zero() || b.is_zero() {
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

pub fn lcm<T: PrimUint>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

pub fn egcd<T: PrimSint>(mut a: T, mut b: T) -> (T, T, T) {
    let mut c = if a > b {
        (a, b) = (b, a);
        [0, 1, 1, 0].map(|x| x.into())
    } else {
        [1, 0, 0, 1].map(|x| x.into())
    }; // treat as a row-major 2x2 matrix
    loop {
        if a.is_zero() {
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
        let a: i64 = 823327498201749212;
        let b: i64 = 734892783927949214;
        let (g, s, t) = egcd(a, b);
        let normal = gcd(a as u64, b as u64) as i64;
        assert_eq!(normal, g);
        assert_eq!(a as i128 * s as i128 + b as i128 * t as i128, g as i128);
    }
}
