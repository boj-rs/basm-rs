use super::{polymul::polymul_ex_u64, polymul_u64};
use crate::math::{modadd, modinv, modmul, modsub};
use alloc::{vec, vec::Vec};
use core::cmp::min;

fn sanitize_u64(x: &[u64]) -> &[u64] {
    for i in (0..x.len()).rev() {
        if x[i] != 0 {
            return &x[..=i];
        }
    }
    &x[..0]
}

/// Computes the negated inverse of the input polynomial `poly`, modulo `x**n`.
/// `poly[i]` should be the coefficient of `x**i`.
///
/// If the inverse exists, the result is a Vec of length `n` wrapped in `Some`
/// containing the negated inverse's coefficients.
/// If the inverse does not exist, the result is `None`.
///
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
pub fn polyneginv_u64(h: &[u64], n: usize, modulo: u64) -> Option<Vec<u64>> {
    let h0_inv = modinv(h[0], modulo)?;
    let mut a = vec![0; n];
    a[0] = modsub(0, h0_inv, modulo);
    let mut d = vec![n];
    while *d.last().unwrap() > 1 {
        d.push(d.last().unwrap().div_ceil(2));
    }
    d.pop();
    let mut l = 1;
    let mut h1a_c = vec![0; n.div_ceil(2)];
    for &target_len in d.iter().rev() {
        let e = min(h.len(), target_len);
        let t = min(l + e - 1, target_len);
        polymul_ex_u64(&mut h1a_c, &a[..l], &h[..e], l, t, modulo);
        let (a_prefix, a_suffix) = a.split_at_mut(l);
        polymul_ex_u64(
            a_suffix,
            a_prefix,
            &h1a_c[..t - l],
            0,
            target_len - l,
            modulo,
        );
        l = target_len;
    }
    Some(a)
}

fn polynegdiv_u64(dividend: &[u64], divisor: &[u64], modulo: u64) -> Option<(Vec<u64>, usize)> {
    if dividend.is_empty() || divisor.is_empty() {
        return None;
    }

    let (f, g) = (sanitize_u64(dividend), sanitize_u64(divisor));
    if g.is_empty() {
        return None;
    }
    if f.len() < g.len() {
        return Some((vec![0], 0));
    }

    // reference: https://people.csail.mit.edu/madhu/ST12/scribe/lect06.pdf
    let rev_g: Vec<u64> = g.iter().rev().copied().collect();
    if let Some(mut rev_g_inv) = polyneginv_u64(&rev_g, f.len() - g.len() + 1, modulo) {
        rev_g_inv.reverse();
        let q = polymul_u64(&rev_g_inv, f, modulo);
        let l = q.len();
        Some((q, l - (f.len() - g.len() + 1)))
    } else {
        None
    }
}

/// Computes the inverse of the input polynomial `poly`, modulo `x**n`.
/// `poly[i]` should be the coefficient of `x**i`.
///
/// If the inverse exists, the result is a Vec of length `n` wrapped in `Some`
/// containing the inverse's coefficients.
/// If the inverse does not exist, the result is `None`.
///
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
pub fn polyinv_u64(poly: &[u64], n: usize, modulo: u64) -> Option<Vec<u64>> {
    let mut out = polyneginv_u64(poly, n, modulo);
    if let Some(x) = &mut out {
        if modulo == 0 {
            x.iter_mut().for_each(|y| *y = 0u64.wrapping_sub(*y));
        } else {
            x.iter_mut()
                .for_each(|y| *y = if *y == 0 { 0 } else { modulo - *y });
        }
    }
    out
}

/// Computes the quotient of the polynomial `dividend` divided by `divisor`.
/// `poly[i]` should be the coefficient of `x**i`.
///
/// Please note that this function will only compute the quotient
/// if the leading coefficient of the polynomial `divisor` is
/// invertible modulo `modulo`. This is for performance reasons.
/// If it is necessary to perform divisions that violate this condition,
/// please factor away the gcd from the coefficients manually before calling this function.
///
/// If the quotient exists and the aforementioned condition is satisfied,
/// the result is the quotient wrapped in `Some`.
/// Otherwise, the result is `None`.
///
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
pub fn polydiv_u64(dividend: &[u64], divisor: &[u64], modulo: u64) -> Option<Vec<u64>> {
    let out = polynegdiv_u64(dividend, divisor, modulo);
    if let Some(x) = &out {
        if modulo == 0 {
            Some(x.0[x.1..].iter().map(|&y| 0u64.wrapping_sub(y)).collect())
        } else {
            Some(
                x.0[x.1..]
                    .iter()
                    .map(|&y| if y == 0 { 0 } else { modulo - y })
                    .collect(),
            )
        }
    } else {
        None
    }
}

/// Computes the remainder of the polynomial `dividend` modulo `divisor`.
/// `poly[i]` should be the coefficient of `x**i`.
///
/// Please note that this function will only compute the remainder
/// if the leading coefficient of the polynomial `divisor` is
/// invertible modulo `modulo`. This is for performance reasons.
/// If it is necessary to perform divisions that violate this condition,
/// please factor away the gcd from the coefficients manually before calling this function.
///
/// If the remainder exists and the aforementioned condition is satisfied,
/// the result is the remainder wrapped in `Some`.
/// Otherwise, the result is `None`.
///
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
pub fn polymod_u64(dividend: &[u64], divisor: &[u64], modulo: u64) -> Option<Vec<u64>> {
    let (f, g) = (sanitize_u64(dividend), sanitize_u64(divisor));
    if g.is_empty() {
        return None;
    }
    if f.len() < g.len() {
        return Some(Vec::from(f));
    }
    if g.len() <= 32 {
        let lead = *g.last().unwrap();
        let lead_inv = modinv(lead, modulo)?;
        let mut out = Vec::from(f);
        for i in (g.len() - 1..f.len()).rev() {
            let m = modmul(lead_inv, out[i], modulo);
            out[i] = 0;
            for j in 0..g.len() - 1 {
                let r = &mut out[i + 1 - g.len() + j];
                let (v, overflow) = r.overflowing_sub(modmul(m, g[j], modulo));
                *r = if overflow { v.wrapping_add(modulo) } else { v };
            }
        }
        out.resize(g.len() - 1, 0);
        return Some(out);
    }
    if let Some((q, pos)) = polynegdiv_u64(dividend, divisor, modulo) {
        let out_len = divisor.len() - 1;
        let mut out = vec![0; out_len];
        let (x, y) = (divisor, &q[pos..]);
        polymul_ex_u64(
            &mut out,
            x,
            y,
            0,
            min(out_len, x.len() + y.len() - 1),
            modulo,
        );
        for i in 0..min(out.len(), dividend.len()) {
            out[i] = modadd(out[i], dividend[i], modulo);
        }
        Some(out)
    } else {
        None
    }
}

/// Computes `x + y` given two polynomials `x` and `y`.
///
/// The resulting Vec has length `max(x.len(), y.len())`.
///
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
pub fn polyadd_u64(x: &[u64], y: &[u64], modulo: u64) -> Vec<u64> {
    let (x, y) = if x.len() > y.len() { (x, y) } else { (y, x) };
    let mut out = x.to_vec();
    for i in 0..x.len() {
        out[i] = modadd(out[i], if i < y.len() { y[i] } else { 0 }, modulo);
    }
    out
}

/// Computes `x - y` given two polynomials `x` and `y`.
///
/// The resulting Vec has length `max(x.len(), y.len())`.
///
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
pub fn polysub_u64(x: &[u64], y: &[u64], modulo: u64) -> Vec<u64> {
    let n = x.len().max(y.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(modsub(
            *x.get(i).unwrap_or(&0),
            *y.get(i).unwrap_or(&0),
            modulo,
        ));
    }
    out
}
