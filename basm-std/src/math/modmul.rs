const TWO64: u128 = 1u128 << 64;
const MASK64: u128 = TWO64 - 1;

/// Computes `(x / modulo, x % modulo)`, where `x = hi * (2 ** 128) + lo`.
/// Panics if `modulo` is zero (division by zero) or `hi >= y` (quotient overflow).
/// The current implementation is based on
/// https://cs.opensource.google/go/go/+/refs/tags/go1.21.5:src/math/bits/bits.go;l=518,
/// which is again based on the Algorithm 4.3.1 D of Donald Knuth's TAOCP.
fn divmod128(hi: u128, lo: u128, modulo: u128) -> (u128, u128) {
    let mut y = modulo;
    debug_assert!(y > 0);
    debug_assert!(hi < y);

    // If high part is zero, we can directly return the results.
    if hi == 0 {
        return (lo / y, lo % y);
    }

    let s: u32 = y.leading_zeros();
    y <<= s;

    let yn1 = y >> 64;
    let yn0 = y & MASK64;
    let un32 = if s == 0 {
        hi
    } else {
        (hi << s) | (lo >> (128 - s))
    };
    let un10 = lo << s;
    let un1 = un10 >> 64;
    let un0 = un10 & MASK64;
    let mut q1 = un32 / yn1;
    let mut rhat = un32 - q1 * yn1;

    while q1 >= TWO64 || q1 * yn0 > TWO64 * rhat + un1 {
        q1 -= 1;
        rhat += yn1;
        if rhat >= TWO64 {
            break;
        }
    }

    let un21 = un32
        .wrapping_mul(TWO64)
        .wrapping_add(un1)
        .wrapping_sub(q1.wrapping_mul(y));
    let mut q0 = un21 / yn1;
    let mut rhat = un21 - q0 * yn1;

    while q0 >= TWO64 || q0 * yn0 > TWO64 * rhat + un0 {
        q0 -= 1;
        rhat += yn1;
        if rhat >= TWO64 {
            break;
        }
    }

    (
        q1 * TWO64 + q0,
        (un21
            .wrapping_mul(TWO64)
            .wrapping_add(un0)
            .wrapping_sub(q0.wrapping_mul(y)))
            >> s,
    )
}

/// Computes the modular multiplication of `x` and `y` mod `modulo`.
/// Panics if `modulo` is zero.
pub fn modmul128(x: u128, y: u128, modulo: u128) -> u128 {
    debug_assert!(modulo > 0);
    let (xh, xl) = (x >> 64, x & MASK64);
    let (yh, yl) = (y >> 64, y & MASK64);
    let mut hi = xh * yh;
    let lo = xl * yl;
    let (mid, overflow) = (xh * yl).overflowing_add(xl * yh);
    if overflow {
        hi += TWO64;
    }
    let (lo, overflow) = lo.overflowing_add(mid << 64);
    if overflow {
        hi += 1;
    }
    hi += mid >> 64;
    divmod128(hi % modulo, lo, modulo).1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn modmul128_returns_modmul128() {
        let p128 = 0u128.wrapping_sub(159);
        assert_eq!(6, modmul128(p128 - 3, p128 - 2, p128));
    }
}
