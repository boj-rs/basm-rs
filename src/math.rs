pub mod miller_rabin;
pub use miller_rabin::*;

// reference: https://lemire.me/blog/2013/12/26/fastest-way-to-compute-the-greatest-common-divisor/

macro_rules! define_gcd_lcm {
    ($gcd:ident, $lcm:ident, $unsigned:ty, $signed:ty) => {
        pub fn $gcd(mut a: $unsigned, mut b: $unsigned) -> $unsigned {
            if a == 0 {
                b
            } else if b == 0 {
                a
            } else {
                let shift = (a | b).trailing_zeros();
                a >>= shift;
                loop {
                    b >>= b.trailing_zeros();
                    b = b.wrapping_sub(a);
                    let m = ((b as $signed) >> (<$signed>::BITS - 1)) as $unsigned;
                    a = a.wrapping_add(b & m);
                    b = b.wrapping_add(m) ^ m;
                    if b == 0 {
                        break;
                    }
                }
                a << shift
            }
        }

        pub fn $lcm(a: $unsigned, b: $unsigned) -> $unsigned {
            a / $gcd(a, b) * b
        }
    };
}

define_gcd_lcm!(gcd, lcm, u32, i32);
define_gcd_lcm!(gcd_u64, lcm_u64, u64, i64);
define_gcd_lcm!(gcd_usize, lcm_usize, usize, isize);

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
