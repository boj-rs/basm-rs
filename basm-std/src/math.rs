pub mod miller_rabin;
pub use miller_rabin::*;
mod sieve;
pub use sieve::LinearSieve;

// reference: https://nyaannyaan.github.io/library/trial/fast-gcd.hpp.html

macro_rules! define_gcd_lcm {
    ($gcd:ident, $lcm:ident, $unsigned:ty) => {
        pub fn $gcd(mut a: $unsigned, mut b: $unsigned) -> $unsigned {
            if a == 0 || b == 0 {
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

        pub fn $lcm(a: $unsigned, b: $unsigned) -> $unsigned {
            a / $gcd(a, b) * b
        }
    };
}

define_gcd_lcm!(gcd, lcm, u32);
define_gcd_lcm!(gcd_u64, lcm_u64, u64);
define_gcd_lcm!(gcd_usize, lcm_usize, usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gcd_returns_gcd() {
        assert_eq!(32, gcd(128736, 72352));
    }

    #[test]
    fn lcm_returns_lcm() {
        assert_eq!(249318024, lcm(234984, 12732));
    }

    #[test]
    fn gcd_u64_returns_gcd() {
        assert_eq!(6, gcd_u64(2763162631554, 1276921782234));
    }

    #[test]
    fn lcm_u64_returns_lcm() {
        assert_eq!(4264971179382324, lcm_u64(273652348, 62341452));
    }
}
