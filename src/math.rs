pub mod miller_rabin;
pub use miller_rabin::*;

// reference: https://lemire.me/blog/2013/12/26/fastest-way-to-compute-the-greatest-common-divisor/
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
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
            let m = ((b as i32) >> 31) as u32;
            a = a.wrapping_add(b & m);
            b = b.wrapping_add(m) ^ m;
            if b == 0 {
                break;
            }
        }
        a << shift
    }
}

pub fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

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
}
