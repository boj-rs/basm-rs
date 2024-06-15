#![allow(clippy::too_many_arguments)]

use alloc::vec;
use alloc::vec::Vec;

pub mod arith {
    // Extended Euclid algorithm:
    //   (g, x, y) is a solution to ax + by = g, where g = gcd(a, b)
    const fn egcd(mut a: i128, mut b: i128) -> (i128, i128, i128) {
        assert!(a > 0 && b > 0);
        let mut c = if a > b {
            (a, b) = (b, a);
            [0, 1, 1, 0]
        } else {
            [1, 0, 0, 1]
        }; // treat as a row-major 2x2 matrix
        loop {
            if a == 0 {
                break (b, c[1], c[3]);
            }
            let (q, r) = (b / a, b % a);
            (a, b) = (r, a);
            c = [c[1] - q * c[0], c[0], c[3] - q * c[2], c[2]];
        }
    }
    // Modular inverse: a^-1 mod modulus
    //   (m == 0 means m == 2^64)
    pub const fn invmod(a: u64, modulus: u64) -> u64 {
        let m = if modulus == 0 {
            1i128 << 64
        } else {
            modulus as i128
        };
        let (g, mut x, _y) = egcd(a as i128, m);
        assert!(g == 1);
        if x < 0 {
            x += m;
        }
        assert!(x > 0 && x < 1i128 << 64);
        x as u64
    }
}

pub struct Arith<const P: u64> {}
impl<const P: u64> Arith<P> {
    pub const R: u64 = ((1u128 << 64) % P as u128) as u64; // 2^64 mod P
    pub const R2: u64 = (Self::R as u128 * Self::R as u128 % P as u128) as u64; // R^2 mod P
    pub const PINV: u64 = arith::invmod(P, 0); // P^-1 mod 2^64
    pub const MAX_NTT_LEN: u64 =
        2u64.pow(Self::factors(2)) * 3u64.pow(Self::factors(3)) * 5u64.pow(Self::factors(5));
    pub const ROOTR: u64 = {
        // ROOT * R mod P (ROOT: MAX_NTT_LEN divides MultiplicativeOrder[ROOT, P])
        assert!(Self::MAX_NTT_LEN % 4050 == 0);
        let mut p = Self::R;
        loop {
            if Self::mpowmod(p, P / 2) != Self::R
                && Self::mpowmod(p, P / 3) != Self::R
                && Self::mpowmod(p, P / 5) != Self::R
            {
                break Self::mpowmod(p, P / Self::MAX_NTT_LEN);
            }
            p = Self::addmod(p, Self::R);
        }
    };
    // Counts the number of `divisor` factors in P-1.
    pub const fn factors(divisor: u64) -> u32 {
        let (mut tmp, mut ans) = (P - 1, 0);
        while tmp % divisor == 0 {
            tmp /= divisor;
            ans += 1;
        }
        ans
    }
    // Montgomery reduction:
    //   x * R^-1 mod P
    pub const fn mreduce(x: u128) -> u64 {
        let m = (x as u64).wrapping_mul(Self::PINV);
        let y = ((m as u128 * P as u128) >> 64) as u64;
        let (out, overflow) = ((x >> 64) as u64).overflowing_sub(y);
        if overflow {
            out.wrapping_add(P)
        } else {
            out
        }
    }
    // Multiplication with Montgomery reduction:
    //   a * b * R^-1 mod P
    pub const fn mmulmod(a: u64, b: u64) -> u64 {
        Self::mreduce(a as u128 * b as u128)
    }
    // Multiplication with Montgomery reduction:
    //   a * b * R^-1 mod P
    // This function only applies the multiplication when INV && TWIDDLE,
    //   otherwise it just returns b.
    pub const fn mmulmod_invtw<const INV: bool, const TWIDDLE: bool>(a: u64, b: u64) -> u64 {
        if INV && TWIDDLE {
            Self::mmulmod(a, b)
        } else {
            b
        }
    }
    // Fused-multiply-sub with Montgomery reduction:
    //   a * b * R^-1 - c mod P
    pub const fn mmulsubmod(a: u64, b: u64, c: u64) -> u64 {
        let x = a as u128 * b as u128;
        let lo = x as u64;
        let hi = Self::submod((x >> 64) as u64, c);
        Self::mreduce(lo as u128 | ((hi as u128) << 64))
    }
    // Computes base^exponent mod P with Montgomery reduction
    pub const fn mpowmod(mut base: u64, mut exponent: u64) -> u64 {
        let mut cur = Self::R;
        while exponent > 0 {
            if exponent % 2 > 0 {
                cur = Self::mmulmod(cur, base);
            }
            exponent /= 2;
            base = Self::mmulmod(base, base);
        }
        cur
    }
    // Computes c as u128 * mreduce(v) as u128,
    //   using d: u64 = mmulmod(P-1, c).
    // It is caller's responsibility to ensure that d is correct.
    // Note that d can be computed by calling mreducelo(c).
    pub const fn mmulmod_noreduce(v: u128, c: u64, d: u64) -> u128 {
        let a: u128 = c as u128 * (v >> 64);
        let b: u128 = d as u128 * (v as u64 as u128);
        let (w, overflow) = a.overflowing_sub(b);
        if overflow {
            w.wrapping_add((P as u128) << 64)
        } else {
            w
        }
    }
    // Computes submod(0, mreduce(x as u128)) fast.
    pub const fn mreducelo(x: u64) -> u64 {
        let m = x.wrapping_mul(Self::PINV);
        ((m as u128 * P as u128) >> 64) as u64
    }
    // Computes a + b mod P, output range [0, P)
    pub const fn addmod(a: u64, b: u64) -> u64 {
        Self::submod(a, P.wrapping_sub(b))
    }
    // Computes a + b mod P, output range [0, 2^64)
    pub const fn addmod64(a: u64, b: u64) -> u64 {
        let (out, overflow) = a.overflowing_add(b);
        if overflow {
            out.wrapping_sub(P)
        } else {
            out
        }
    }
    // Computes a + b mod P, selects addmod64 or addmod depending on INV && TWIDDLE
    pub const fn addmodopt_invtw<const INV: bool, const TWIDDLE: bool>(a: u64, b: u64) -> u64 {
        if INV && TWIDDLE {
            Self::addmod64(a, b)
        } else {
            Self::addmod(a, b)
        }
    }
    // Computes a - b mod P, output range [0, P)
    pub const fn submod(a: u64, b: u64) -> u64 {
        let (out, overflow) = a.overflowing_sub(b);
        if overflow {
            out.wrapping_add(P)
        } else {
            out
        }
    }
}

pub struct NttPlan {
    pub n: usize, // n == g*m
    pub g: usize, // g: size of the base case
    pub m: usize, // m divides Arith::<P>::MAX_NTT_LEN
    pub cost: usize,
    pub last_radix: usize,
    pub s_list: Vec<(usize, usize)>,
}
impl NttPlan {
    pub fn build<const P: u64>(min_len: usize) -> Self {
        assert!(min_len as u64 <= Arith::<P>::MAX_NTT_LEN);
        let (mut len_max, mut len_max_cost, mut g) = (usize::MAX, usize::MAX, 1);
        for m7 in 0..=1 {
            for m5 in 0..=Arith::<P>::factors(5) {
                let len75 = 7u64.pow(m7) * 5u64.pow(m5);
                if len75 >= 2 * min_len as u64 {
                    break;
                }
                for m3 in 0..=Arith::<P>::factors(3) {
                    let len = len75 * 3u64.pow(m3);
                    if len >= 2 * min_len as u64 {
                        break;
                    }
                    let (mut len, mut m2) = (len as usize, 0);
                    while len < min_len && m2 < Arith::<P>::factors(2) {
                        len *= 2;
                        m2 += 1;
                    }
                    if len >= min_len && len < len_max_cost {
                        let (mut tmp, mut cost) = (len, 0);
                        let mut g_new = 1;
                        if len % 7 == 0 {
                            (g_new, tmp, cost) = (7, tmp / 7, cost + len * 115 / 100);
                        } else if len % 5 == 0 {
                            (g_new, tmp, cost) = (5, tmp / 5, cost + len * 89 / 100);
                        } else if m3 >= m2 + 2 {
                            (g_new, tmp, cost) = (9, tmp / 9, cost + len * 180 / 100);
                        } else if m2 >= m3 + 3 && (m2 - m3) % 2 == 1 {
                            (g_new, tmp, cost) = (8, tmp / 8, cost + len * 130 / 100);
                        } else if m2 >= m3 + 2 && m3 == 0 {
                            (g_new, tmp, cost) = (4, tmp / 4, cost + len * 87 / 100);
                        } else if m2 == 0 && m3 >= 1 {
                            (g_new, tmp, cost) = (3, tmp / 3, cost + len * 86 / 100);
                        } else if m3 == 0 && m2 >= 1 {
                            (g_new, tmp, cost) = (2, tmp / 2, cost + len * 86 / 100);
                        } else if len % 6 == 0 {
                            (g_new, tmp, cost) = (6, tmp / 6, cost + len * 91 / 100);
                        }
                        let (mut b6, mut b2) = (false, false);
                        while tmp % 6 == 0 {
                            (tmp, cost) = (tmp / 6, cost + len * 106 / 100);
                            b6 = true;
                        }
                        while tmp % 5 == 0 {
                            (tmp, cost) = (tmp / 5, cost + len * 131 / 100);
                        }
                        while tmp % 4 == 0 {
                            (tmp, cost) = (tmp / 4, cost + len);
                        }
                        while tmp % 3 == 0 {
                            (tmp, cost) = (tmp / 3, cost + len);
                        }
                        while tmp % 2 == 0 {
                            (tmp, cost) = (tmp / 2, cost + len);
                            b2 = true;
                        }
                        if b6 && b2 {
                            cost -= len * 6 / 100;
                        }
                        if cost < len_max_cost {
                            (len_max, len_max_cost, g) = (len, cost, g_new);
                        }
                    }
                }
            }
        }
        let (mut cnt6, mut cnt5, mut cnt4, mut cnt3, mut cnt2) = (0, 0, 0, 0, 0);
        let mut tmp = len_max / g;
        while tmp % 6 == 0 {
            tmp /= 6;
            cnt6 += 1;
        }
        while tmp % 5 == 0 {
            tmp /= 5;
            cnt5 += 1;
        }
        while tmp % 4 == 0 {
            tmp /= 4;
            cnt4 += 1;
        }
        while tmp % 3 == 0 {
            tmp /= 3;
            cnt3 += 1;
        }
        while tmp % 2 == 0 {
            tmp /= 2;
            cnt2 += 1;
        }
        while cnt6 > 0 && cnt2 > 0 {
            cnt6 -= 1;
            cnt2 -= 1;
            cnt4 += 1;
            cnt3 += 1;
        }
        let s_list = {
            let mut out = vec![];
            let mut tmp = len_max;
            for _ in 0..cnt2 {
                out.push((tmp, 2));
                tmp /= 2;
            }
            for _ in 0..cnt3 {
                out.push((tmp, 3));
                tmp /= 3;
            }
            for _ in 0..cnt4 {
                out.push((tmp, 4));
                tmp /= 4;
            }
            for _ in 0..cnt5 {
                out.push((tmp, 5));
                tmp /= 5;
            }
            for _ in 0..cnt6 {
                out.push((tmp, 6));
                tmp /= 6;
            }
            out
        };
        Self {
            n: len_max,
            g,
            m: len_max / g,
            cost: len_max_cost,
            last_radix: s_list.last().unwrap_or(&(1, 1)).1,
            s_list,
        }
    }
}
fn conv_base<const P: u64>(n: usize, x: *mut u64, y: *mut u64, c: u64) {
    unsafe {
        let c2 = Arith::<P>::mreducelo(c);
        let out = x.sub(n);
        for i in 0..n {
            let mut v: u128 = 0;
            for j in i + 1..n {
                let (w, overflow) =
                    v.overflowing_sub(*x.add(j) as u128 * *y.add(i + n - j) as u128);
                v = if overflow {
                    w.wrapping_add((P as u128) << 64)
                } else {
                    w
                };
            }
            v = Arith::<P>::mmulmod_noreduce(v, c, c2);
            for j in 0..=i {
                let (w, overflow) = v.overflowing_sub(*x.add(j) as u128 * *y.add(i - j) as u128);
                v = if overflow {
                    w.wrapping_add((P as u128) << 64)
                } else {
                    w
                };
            }
            *out.add(i) = Arith::<P>::mreduce(v);
        }
    }
}

struct NttKernelImpl<const P: u64, const INV: bool>;
impl<const P: u64, const INV: bool> NttKernelImpl<P, INV> {
    const ROOTR: u64 = Arith::<P>::mpowmod(
        Arith::<P>::ROOTR,
        if INV { Arith::<P>::MAX_NTT_LEN - 1 } else { 1 },
    );
    const U3: u64 = Arith::<P>::mpowmod(Self::ROOTR, Arith::<P>::MAX_NTT_LEN / 3);
    const U4: u64 = Arith::<P>::mpowmod(Self::ROOTR, Arith::<P>::MAX_NTT_LEN / 4);
    const U5: u64 = Arith::<P>::mpowmod(Self::ROOTR, Arith::<P>::MAX_NTT_LEN / 5);
    const U6: u64 = Arith::<P>::mpowmod(Self::ROOTR, Arith::<P>::MAX_NTT_LEN / 6);
    const C5: (u64, u64, u64, u64, u64, u64) = {
        let w = Self::U5;
        let w2 = Arith::<P>::mpowmod(w, 2);
        let w4 = Arith::<P>::mpowmod(w, 4);
        let inv2 = Arith::<P>::mmulmod(Arith::<P>::R2, arith::invmod(2, P));
        let inv4 = Arith::<P>::mmulmod(Arith::<P>::R2, arith::invmod(4, P));
        let c51 = Arith::<P>::addmod(Arith::<P>::R, inv4); // 1 + 4^-1 mod P
        let c52 = Arith::<P>::addmod(Arith::<P>::mmulmod(inv2, Arith::<P>::addmod(w, w4)), inv4); // 4^-1 * (2*w + 2*w^4 + 1) mod P
        let c53 = Arith::<P>::mmulmod(inv2, Arith::<P>::submod(w, w4)); // 2^-1 * (w - w^4) mod P
        let c54 = Arith::<P>::addmod(Arith::<P>::addmod(w, w2), inv2); // 2^-1 * (2*w + 2*w^2 + 1) mod P
        let c55 = Arith::<P>::addmod(Arith::<P>::addmod(w2, w4), inv2); // 2^-1 * (2*w^2 + 2*w^4 + 1) mod P
        (0, c51, c52, c53, c54, c55)
    };
}
const fn ntt2_kernel<const P: u64, const INV: bool, const TWIDDLE: bool>(
    w1: u64,
    a: u64,
    mut b: u64,
) -> (u64, u64) {
    if !INV && TWIDDLE {
        b = Arith::<P>::mmulmod(w1, b);
    }
    let out0 = Arith::<P>::addmod(a, b);
    let out1 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(w1, Arith::<P>::submod(a, b));
    (out0, out1)
}
unsafe fn ntt2_single_block<const P: u64, const INV: bool, const TWIDDLE: bool>(
    s1: usize,
    mut px: *mut u64,
    ptf: *const u64,
) -> (*mut u64, *const u64) {
    let w1 = if TWIDDLE { *ptf } else { 0 };
    for _ in 0..s1 {
        (*px, *px.add(s1)) = ntt2_kernel::<P, INV, TWIDDLE>(w1, *px, *px.add(s1));
        px = px.add(1);
    }
    (px.add(s1), ptf.add(1))
}
const fn ntt3_kernel<const P: u64, const INV: bool, const TWIDDLE: bool>(
    w1: u64,
    w2: u64,
    a: u64,
    mut b: u64,
    mut c: u64,
) -> (u64, u64, u64) {
    if !INV && TWIDDLE {
        b = Arith::<P>::mmulmod(w1, b);
        c = Arith::<P>::mmulmod(w2, c);
    }
    let kbmc = Arith::<P>::mmulmod(NttKernelImpl::<P, INV>::U3, Arith::<P>::submod(b, c));
    let out0 = Arith::<P>::addmod(a, Arith::<P>::addmod(b, c));
    let out1 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w1,
        Arith::<P>::submod(a, Arith::<P>::submod(c, kbmc)),
    );
    let out2 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w2,
        Arith::<P>::submod(Arith::<P>::submod(a, b), kbmc),
    );
    (out0, out1, out2)
}
unsafe fn ntt3_single_block<const P: u64, const INV: bool, const TWIDDLE: bool>(
    s1: usize,
    mut px: *mut u64,
    ptf: *const u64,
) -> (*mut u64, *const u64) {
    let w1 = if TWIDDLE { *ptf } else { 0 };
    let w2 = Arith::<P>::mmulmod(w1, w1);
    for _ in 0..s1 {
        (*px, *px.add(s1), *px.add(2 * s1)) =
            ntt3_kernel::<P, INV, TWIDDLE>(w1, w2, *px, *px.add(s1), *px.add(2 * s1));
        px = px.add(1);
    }
    (px.add(2 * s1), ptf.add(1))
}
const fn ntt4_kernel<const P: u64, const INV: bool, const TWIDDLE: bool>(
    w1: u64,
    w2: u64,
    w3: u64,
    a: u64,
    mut b: u64,
    mut c: u64,
    mut d: u64,
) -> (u64, u64, u64, u64) {
    if !INV && TWIDDLE {
        b = Arith::<P>::mmulmod(w1, b);
        c = Arith::<P>::mmulmod(w2, c);
        d = Arith::<P>::mmulmod(w3, d);
    }
    let apc = Arith::<P>::addmod(a, c);
    let amc = Arith::<P>::submod(a, c);
    let bpd = Arith::<P>::addmod(b, d);
    let bmd = Arith::<P>::submod(b, d);
    let jbmd = Arith::<P>::mmulmod(NttKernelImpl::<P, INV>::U4, bmd);
    let out0 = Arith::<P>::addmod(apc, bpd);
    let out1 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w1,
        Arith::<P>::addmodopt_invtw::<INV, TWIDDLE>(amc, jbmd),
    );
    let out2 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(w2, Arith::<P>::submod(apc, bpd));
    let out3 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(w3, Arith::<P>::submod(amc, jbmd));
    (out0, out1, out2, out3)
}
unsafe fn ntt4_single_block<const P: u64, const INV: bool, const TWIDDLE: bool>(
    s1: usize,
    mut px: *mut u64,
    ptf: *const u64,
) -> (*mut u64, *const u64) {
    let w1 = if TWIDDLE { *ptf } else { 0 };
    let w2 = Arith::<P>::mmulmod(w1, w1);
    let w3 = Arith::<P>::mmulmod(w1, w2);
    for _ in 0..s1 {
        (*px, *px.add(s1), *px.add(2 * s1), *px.add(3 * s1)) = ntt4_kernel::<P, INV, TWIDDLE>(
            w1,
            w2,
            w3,
            *px,
            *px.add(s1),
            *px.add(2 * s1),
            *px.add(3 * s1),
        );
        px = px.add(1);
    }
    (px.add(3 * s1), ptf.add(1))
}
const fn ntt5_kernel<const P: u64, const INV: bool, const TWIDDLE: bool>(
    w1: u64,
    w2: u64,
    w3: u64,
    w4: u64,
    a: u64,
    mut b: u64,
    mut c: u64,
    mut d: u64,
    mut e: u64,
) -> (u64, u64, u64, u64, u64) {
    if !INV && TWIDDLE {
        b = Arith::<P>::mmulmod(w1, b);
        c = Arith::<P>::mmulmod(w2, c);
        d = Arith::<P>::mmulmod(w3, d);
        e = Arith::<P>::mmulmod(w4, e);
    }
    let t1 = Arith::<P>::addmod(b, e);
    let t2 = Arith::<P>::addmod(c, d);
    let t3 = Arith::<P>::submod(b, e);
    let t4 = Arith::<P>::submod(d, c);
    let t5 = Arith::<P>::addmod(t1, t2);
    let t6 = Arith::<P>::submod(t1, t2);
    let t7 = Arith::<P>::addmod64(t3, t4);
    let m1 = Arith::<P>::addmod(a, t5);
    let m2 = Arith::<P>::mmulsubmod(NttKernelImpl::<P, INV>::C5.1, t5, m1);
    let m3 = Arith::<P>::mmulmod(NttKernelImpl::<P, INV>::C5.2, t6);
    let m4 = Arith::<P>::mmulmod(NttKernelImpl::<P, INV>::C5.3, t7);
    let m5 = Arith::<P>::mmulsubmod(NttKernelImpl::<P, INV>::C5.4, t4, m4);
    let m6 = Arith::<P>::mmulsubmod(P.wrapping_sub(NttKernelImpl::<P, INV>::C5.5), t3, m4);
    let s1 = Arith::<P>::submod(m3, m2);
    let s2 = Arith::<P>::addmod(m2, m3);
    let out0 = m1;
    let out1 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(w1, Arith::<P>::submod(s1, m5));
    let out2 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w2,
        Arith::<P>::submod(Arith::<P>::submod(0, s2), m6),
    );
    let out3 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(w3, Arith::<P>::submod(m6, s2));
    let out4 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w4,
        Arith::<P>::addmodopt_invtw::<INV, TWIDDLE>(s1, m5),
    );
    (out0, out1, out2, out3, out4)
}
unsafe fn ntt5_single_block<const P: u64, const INV: bool, const TWIDDLE: bool>(
    s1: usize,
    mut px: *mut u64,
    ptf: *const u64,
) -> (*mut u64, *const u64) {
    let w1 = if TWIDDLE { *ptf } else { 0 };
    let w2 = Arith::<P>::mmulmod(w1, w1);
    let w3 = Arith::<P>::mmulmod(w1, w2);
    let w4 = Arith::<P>::mmulmod(w2, w2);
    for _ in 0..s1 {
        (
            *px,
            *px.add(s1),
            *px.add(2 * s1),
            *px.add(3 * s1),
            *px.add(4 * s1),
        ) = ntt5_kernel::<P, INV, TWIDDLE>(
            w1,
            w2,
            w3,
            w4,
            *px,
            *px.add(s1),
            *px.add(2 * s1),
            *px.add(3 * s1),
            *px.add(4 * s1),
        );
        px = px.add(1);
    }
    (px.add(4 * s1), ptf.add(1))
}
const fn ntt6_kernel<const P: u64, const INV: bool, const TWIDDLE: bool>(
    w1: u64,
    w2: u64,
    w3: u64,
    w4: u64,
    w5: u64,
    mut a: u64,
    mut b: u64,
    mut c: u64,
    mut d: u64,
    mut e: u64,
    mut f: u64,
) -> (u64, u64, u64, u64, u64, u64) {
    if !INV && TWIDDLE {
        b = Arith::<P>::mmulmod(w1, b);
        c = Arith::<P>::mmulmod(w2, c);
        d = Arith::<P>::mmulmod(w3, d);
        e = Arith::<P>::mmulmod(w4, e);
        f = Arith::<P>::mmulmod(w5, f);
    }
    (a, d) = (Arith::<P>::addmod(a, d), Arith::<P>::submod(a, d));
    (b, e) = (Arith::<P>::addmod(b, e), Arith::<P>::submod(b, e));
    (c, f) = (Arith::<P>::addmod(c, f), Arith::<P>::submod(c, f));
    let lbmc = Arith::<P>::mmulmod(NttKernelImpl::<P, INV>::U6, Arith::<P>::submod(b, c));
    let out0 = Arith::<P>::addmod(a, Arith::<P>::addmod(b, c));
    let out2 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w2,
        Arith::<P>::submod(a, Arith::<P>::submod(b, lbmc)),
    );
    let out4 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w4,
        Arith::<P>::submod(Arith::<P>::submod(a, c), lbmc),
    );
    let lepf = Arith::<P>::mmulmod(NttKernelImpl::<P, INV>::U6, Arith::<P>::addmod64(e, f));
    let out1 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w1,
        Arith::<P>::submod(d, Arith::<P>::submod(f, lepf)),
    );
    let out3 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w3,
        Arith::<P>::submod(d, Arith::<P>::submod(e, f)),
    );
    let out5 = Arith::<P>::mmulmod_invtw::<INV, TWIDDLE>(
        w5,
        Arith::<P>::submod(d, Arith::<P>::submod(lepf, e)),
    );
    (out0, out1, out2, out3, out4, out5)
}
unsafe fn ntt6_single_block<const P: u64, const INV: bool, const TWIDDLE: bool>(
    s1: usize,
    mut px: *mut u64,
    ptf: *const u64,
) -> (*mut u64, *const u64) {
    let w1 = if TWIDDLE { *ptf } else { 0 };
    let w2 = Arith::<P>::mmulmod(w1, w1);
    let w3 = Arith::<P>::mmulmod(w1, w2);
    let w4 = Arith::<P>::mmulmod(w2, w2);
    let w5 = Arith::<P>::mmulmod(w2, w3);
    for _ in 0..s1 {
        (
            *px,
            *px.add(s1),
            *px.add(2 * s1),
            *px.add(3 * s1),
            *px.add(4 * s1),
            *px.add(5 * s1),
        ) = ntt6_kernel::<P, INV, TWIDDLE>(
            w1,
            w2,
            w3,
            w4,
            w5,
            *px,
            *px.add(s1),
            *px.add(2 * s1),
            *px.add(3 * s1),
            *px.add(4 * s1),
            *px.add(5 * s1),
        );
        px = px.add(1);
    }
    (px.add(5 * s1), ptf.add(1))
}

fn ntt_dif_dit<const P: u64, const INV: bool>(plan: &NttPlan, x: &mut [u64], tf_list: &[u64]) {
    let mut i_list: Vec<_> = (0..plan.s_list.len()).collect();
    if INV {
        i_list.reverse();
    }
    let mut ptf = tf_list.as_ptr();
    for i in i_list {
        let (s, radix) = plan.s_list[i];
        let s1 = s / radix;
        unsafe {
            let mut px = x.as_mut_ptr();
            let px_end = px.add(plan.n);
            match radix {
                2 => {
                    (px, ptf) = ntt2_single_block::<P, INV, false>(s1, px, ptf);
                    while px < px_end {
                        (px, ptf) = ntt2_single_block::<P, INV, true>(s1, px, ptf);
                    }
                }
                3 => {
                    (px, ptf) = ntt3_single_block::<P, INV, false>(s1, px, ptf);
                    while px < px_end {
                        (px, ptf) = ntt3_single_block::<P, INV, true>(s1, px, ptf);
                    }
                }
                4 => {
                    (px, ptf) = ntt4_single_block::<P, INV, false>(s1, px, ptf);
                    while px < px_end {
                        (px, ptf) = ntt4_single_block::<P, INV, true>(s1, px, ptf);
                    }
                }
                5 => {
                    (px, ptf) = ntt5_single_block::<P, INV, false>(s1, px, ptf);
                    while px < px_end {
                        (px, ptf) = ntt5_single_block::<P, INV, true>(s1, px, ptf);
                    }
                }
                6 => {
                    (px, ptf) = ntt6_single_block::<P, INV, false>(s1, px, ptf);
                    while px < px_end {
                        (px, ptf) = ntt6_single_block::<P, INV, true>(s1, px, ptf);
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
}

fn calc_twiddle_factors<const P: u64, const INV: bool>(
    s_list: &[(usize, usize)],
    out: &mut [u64],
) -> usize {
    let mut p = 1;
    out[0] = Arith::<P>::R;
    for i in (1..s_list.len()).rev() {
        let radix = s_list[i - 1].1;
        let w = Arith::<P>::mpowmod(
            NttKernelImpl::<P, INV>::ROOTR,
            Arith::<P>::MAX_NTT_LEN / (p * radix * s_list.last().unwrap().1) as u64,
        );
        for j in p..radix * p {
            out[j] = Arith::<P>::mmulmod(w, out[j - p]);
        }
        p *= radix;
    }
    p
}

// Performs (cyclic) integer convolution modulo P using NTT.
// Modifies the input buffers in-place.
// The output is saved in the slice `x`.
// The input slices must have the same length.
pub fn conv<const P: u64>(
    plan: &NttPlan,
    x: &mut [u64],
    xlen: usize,
    y: &mut [u64],
    ylen: usize,
    mut mult: u64,
) {
    assert!(!x.is_empty() && x.len() == y.len());
    let (_n, g, m, last_radix) = (plan.n, plan.g, plan.m, plan.last_radix as u64);

    /* multiply by a constant in advance */
    mult = Arith::<P>::mmulmod(
        Arith::<P>::mpowmod(Arith::<P>::R2, 3),
        Arith::<P>::mmulmod(mult, (P - 1) / m as u64),
    );
    for v in if xlen < ylen {
        &mut x[g..g + xlen]
    } else {
        &mut y[g..g + ylen]
    } {
        *v = Arith::<P>::mmulmod(*v, mult);
    }

    /* compute the total space needed for twiddle factors */
    let (mut radix_cumul, mut tf_all_count) = (1, 2); // 2 extra slots
    for &(_, radix) in &plan.s_list {
        tf_all_count += radix_cumul;
        radix_cumul *= radix;
    }

    /* build twiddle factors */
    let mut tf_list = vec![0u64; tf_all_count];
    let mut tf_last_start = 0;
    for i in 0..plan.s_list.len() {
        let x =
            calc_twiddle_factors::<P, false>(&plan.s_list[0..=i], &mut tf_list[tf_last_start..]);
        if i + 1 < plan.s_list.len() {
            tf_last_start += x;
        }
    }

    /* dif fft */
    ntt_dif_dit::<P, false>(plan, &mut x[g..], &tf_list);
    ntt_dif_dit::<P, false>(plan, &mut y[g..], &tf_list);

    /* naive multiplication */
    let (mut i, mut ii, mut ii_mod_last_radix) = (g, tf_last_start, 0);
    let mut tf_current = Arith::<P>::R;
    let tf_mult = Arith::<P>::mpowmod(
        NttKernelImpl::<P, false>::ROOTR,
        Arith::<P>::MAX_NTT_LEN / last_radix,
    );
    while i < g + plan.n {
        conv_base::<P>(g, x[i..].as_mut_ptr(), y[i..].as_mut_ptr(), tf_current);
        i += g;
        ii_mod_last_radix += 1;
        if ii_mod_last_radix == last_radix {
            ii += 1;
            ii_mod_last_radix = 0;
            tf_current = tf_list[ii];
        } else {
            tf_current = Arith::<P>::mmulmod(tf_current, tf_mult);
        }
    }

    /* dit fft */
    let mut tf_last_start = 0;
    for i in (0..plan.s_list.len()).rev() {
        tf_last_start +=
            calc_twiddle_factors::<P, true>(&plan.s_list[0..=i], &mut tf_list[tf_last_start..]);
    }
    ntt_dif_dit::<P, true>(plan, x, &tf_list);
}

////////////////////////////////////////////////////////////////////////////////

pub use core::cmp::{max, min};

pub const P1: u64 = 14_259_017_916_245_606_401; // Max NTT length = 2^22 * 3^21 * 5^2 = 1_096_847_532_018_892_800
pub const P2: u64 = 17_984_575_660_032_000_001; // Max NTT length = 2^19 * 3^17 * 5^6 = 1_057_916_215_296_000_000
pub const P3: u64 = 17_995_154_822_184_960_001; // Max NTT length = 2^17 * 3^22 * 5^4 = 2_570_736_403_169_280_000

pub const P1INV_R_MOD_P2: u64 = Arith::<P2>::mmulmod(Arith::<P2>::R2, arith::invmod(P1, P2));
pub const P1P2INV_R_MOD_P3: u64 = Arith::<P3>::mmulmod(
    Arith::<P3>::R2,
    arith::invmod((P1 as u128 * P2 as u128 % P3 as u128) as u64, P3),
);
pub const P1_R_MOD_P3: u64 = Arith::<P3>::mmulmod(Arith::<P3>::R2, P1);
pub const P1P2_LO: u64 = (P1 as u128 * P2 as u128) as u64;
pub const P1P2_HI: u64 = ((P1 as u128 * P2 as u128) >> 64) as u64;

/// Propagates carry from the beginning to the end of acc,
///   and returns the resulting carry if it is nonzero.
pub fn propagate_carry(acc: &mut [u64], mut carry: u64) -> u64 {
    for x in acc {
        let (v, overflow) = x.overflowing_add(carry);
        (*x, carry) = (v, u64::from(overflow));
        if !overflow {
            break;
        }
    }
    carry
}

pub fn pack_into(src: &[u64], dst1: &mut [u64], dst2: &mut [u64], bits: u64) {
    let mut p = 0u64;
    let mut pdst1 = dst1.as_mut_ptr();
    let mut pdst2 = dst2.as_mut_ptr();
    let mut x = 0u64;
    let mask = (1u64 << bits) - 1;
    for v in src {
        let mut k = 0;
        while k < 64 {
            x |= (v >> k) << p;
            let q = 64 - k;
            if p + q >= bits {
                unsafe {
                    let out = x & mask;
                    *pdst1 = out;
                    *pdst2 = out;
                }
                x = 0;
                unsafe {
                    (pdst1, pdst2, k, p) = (pdst1.add(1), pdst2.add(1), k + bits - p, 0);
                }
            } else {
                p += q;
                break;
            }
        }
    }
    unsafe {
        if p > 0 {
            let out = x & mask;
            *pdst1 = out;
            *pdst2 = out;
        }
    }
}

pub const fn compute_bits(l: u64) -> u64 {
    let total_bits = l * 64;
    let (mut lo, mut hi) = (42, 62);
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        let single_digit_max_val = (1u64 << mid) - 1;
        let l_corrected = (total_bits + mid - 1) / mid;
        let (lhs, overflow) = (single_digit_max_val as u128)
            .pow(2)
            .overflowing_mul(l_corrected as u128);
        if !overflow && lhs < P2 as u128 * P3 as u128 {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}
