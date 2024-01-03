pub mod miller_rabin;
pub use miller_rabin::*;
mod sieve;
pub use sieve::LinearSieve;
mod pollard_rho;
use crate::traits::PrimUint;
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
}
