use crate::math::{factorize, modadd, modinv, modmul, modsub};
use alloc::{vec, vec::Vec};
use core::cmp::max;

fn reeds_sloane_prime_power(first_terms: &[u64], p: u64, e: usize) -> Vec<u64> {
    let n = first_terms.len();
    assert!(n >= 2);
    assert!(p >= 2);
    assert!(e >= 1);

    let ppow: Vec<u64> = (0..=e).map(|x| p.wrapping_pow(x as u32)).collect();

    let p_e = ppow[e];
    let s: Vec<u64> = first_terms
        .iter()
        .map(|&x| if p_e > 0 { x % p_e } else { x })
        .collect();

    // Utility functions (core logic)
    fn l(a: &[u64], b: &[u64]) -> usize {
        max(a.len() - 1, b.len())
    }
    let factor_by_p = |x: u64| -> (u64, usize) {
        // Returns (theta, u)
        let (mut lo, mut hi) = (0, e);
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            #[allow(clippy::collapsible_else_if)]
            if ppow[mid] == 0 {
                if x == 0 {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            } else {
                if x % ppow[mid] == 0 {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
        }
        (if ppow[lo] == 0 { 0 } else { x / ppow[lo] }, lo)
    };

    // Step 0
    let mut a: Vec<Vec<u64>> = vec![];
    let mut b = vec![];
    let mut a_new = vec![];
    let mut b_new = vec![];
    let mut theta = vec![0; e];
    let mut u = vec![0; e];
    for eta in 0..e {
        let p_eta = ppow[eta];
        a.push(vec![p_eta]);
        b.push(vec![]);
        a_new.push(vec![p_eta]);
        let p_eta_s0 = modmul(p_eta, s[0], p_e);
        b_new.push(if p_eta_s0 == 0 {
            vec![]
        } else {
            vec![p_eta_s0]
        });
        let c = modmul(s[0], p_eta, p_e);
        (theta[eta], u[eta]) = factor_by_p(c);
    }

    // Step k
    let mut a_old: Vec<Vec<u64>> = vec![vec![]; e];
    let mut b_old: Vec<Vec<u64>> = vec![vec![]; e];
    let mut u_old = vec![0; e];
    let mut theta_old = vec![0; e];
    let mut r = vec![0; e];
    for k in 1..n {
        // Part 1
        for g in 0..e {
            if l(&a_new[g], &b_new[g]) > l(&a[g], &b[g]) {
                let h = e - 1 - u[g];
                a_old[g].clone_from(&a[h]);
                b_old[g].clone_from(&b[h]);
                u_old[g] = u[h];
                theta_old[g] = theta[h];
                r[g] = k - 1;
            }
        }
        // Part 2
        a.clone_from(&a_new);
        b.clone_from(&b_new);
        // Part 3
        for eta in 0..e {
            let mut c = modsub(0, if b[eta].len() > k { b[eta][k] } else { 0 }, p_e);
            for j in 0..=k {
                if a[eta].len() > j {
                    c = modadd(c, modmul(s[k - j], a[eta][j], p_e), p_e);
                }
            }
            (theta[eta], u[eta]) = factor_by_p(c);
            if u[eta] == e {
                // Case I
                a_new[eta].clone_from(&a[eta]);
                b_new[eta].clone_from(&b[eta]);
            } else {
                // Case II
                let g = e - 1 - u[eta];
                if l(&a[g], &b[g]) == 0 {
                    // Case IIa
                    a_new[eta].clone_from(&a[eta]);
                    let mut tmp = b[eta].clone();
                    if tmp.len() <= k {
                        tmp.resize(k + 1, 0);
                    }
                    tmp[k] = modadd(tmp[k], modmul(theta[eta], ppow[u[eta]], p_e), p_e);
                    b_new[eta] = tmp;
                } else {
                    // Case IIb
                    let theta_g_old_inv = modinv(theta_old[g], p_e).unwrap();
                    let m = modmul(theta[eta], theta_g_old_inv, p_e);
                    let m = modmul(m, ppow[u[eta] - u_old[g]], p_e);
                    let d = k - r[g];
                    let mut tmp = a[eta].clone();
                    if tmp.len() < a_old[g].len() + d {
                        tmp.resize(a_old[g].len() + d, 0);
                    }
                    for j in 0..a_old[g].len() {
                        tmp[j + d] = modsub(tmp[j + d], modmul(m, a_old[g][j], p_e), p_e);
                    }
                    while tmp.last() == Some(&0) {
                        tmp.pop();
                    }
                    a_new[eta] = tmp;
                    let mut tmp = b[eta].clone();
                    if tmp.len() < b_old[g].len() + d {
                        tmp.resize(b_old[g].len() + d, 0);
                    }
                    for j in 0..b_old[g].len() {
                        tmp[j + d] = modsub(tmp[j + d], modmul(m, b_old[g][j], p_e), p_e);
                    }
                    while tmp.last() == Some(&0) {
                        tmp.pop();
                    }
                    b_new[eta] = tmp;
                }
            }
        }
    }

    // Extract output
    let mut out = vec![];
    for i in 1..=l(&a_new[0], &b_new[0]) {
        out.push(if i < a_new[0].len() {
            modsub(0, a_new[0][i], p_e)
        } else {
            0
        });
    }
    out
}

/// Finds a minimal length linear recurrence for `first_terms`
/// under modulo `modulo`, via the Reeds-Sloane algorithm.
///
/// Note that `modulo` of `0` is interpreted as `2**64`.
pub fn reeds_sloane(first_terms: &[u64], modulo: u64) -> Vec<u64> {
    if first_terms.len() <= 1 {
        return vec![];
    }
    if modulo == 1 {
        return vec![0];
    }
    if modulo == 0 {
        // We deal with 2**64 first, to ensure modulo > 0 below.
        return reeds_sloane_prime_power(first_terms, 2, 64);
    }

    let factors = {
        let factors = factorize(modulo);
        let mut out = vec![];
        let (mut prev, mut cnt) = (0, 0usize);
        for f in factors {
            if f == prev {
                cnt += 1;
            } else {
                if prev != 0 {
                    out.push((prev, cnt));
                }
                (prev, cnt) = (f, 1);
            }
        }
        if prev != 0 {
            out.push((prev, cnt));
        }
        out
    };
    let mut out_prime = vec![];
    let mut max_len = 0;
    for &(p, e) in factors.iter() {
        let val = reeds_sloane_prime_power(first_terms, p, e);
        max_len = max(val.len(), max_len);
        out_prime.push((val, p.pow(e as u32)));
    }
    for v in out_prime.iter_mut() {
        v.0.resize(max_len, 0);
    }
    let mut out = vec![0; max_len];
    let mut cumul_mod = 1;
    for (v, cur_mod) in out_prime {
        if cumul_mod == 1 {
            out = v;
        } else {
            let (p, q) = (cumul_mod, cur_mod);
            let (pinv, qinv) = (modinv(p, q).unwrap(), modinv(q, p).unwrap());
            for i in 0..max_len {
                // out[i] mod cumul_mod, v[i] mod cur_mod
                // No overflow since we have ensured p*q < 2**64
                let mp = modmul(out[i], qinv, p);
                let mq = modmul(v[i], pinv, q);
                out[i] = modadd(q * mp, p * mq, p * q);
            }
        }
        cumul_mod *= cur_mod;
    }
    out
}

/// This function is an alias for the function `reeds_sloane`.
///
/// Finds a minimal length linear recurrence for `first_terms`
/// under modulo `modulo`, via the Reeds-Sloane algorithm.
///
/// Note that `modulo` of `0` is interpreted as `2**64`.
pub fn linear_fit(first_terms: &[u64], modulo: u64) -> Vec<u64> {
    reeds_sloane(first_terms, modulo)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_reeds_sloane_fibosqsum() {
        let mut first_terms = [
            0,
            1,
            1,
            2 * 2,
            3 * 3,
            5 * 5,
            8 * 8,
            13 * 13,
            21 * 21,
            34 * 34,
            55 * 55,
            89 * 89,
            144 * 144,
            233 * 233,
            377 * 377,
            610 * 610,
            987 * 987,
            1597 * 1597,
        ];
        for i in 1..first_terms.len() {
            first_terms[i] += first_terms[i - 1];
        }
        let modulo = 1_000_000_007;
        let coeff = linear_fit(&first_terms, modulo);
        assert!(coeff == vec![2, 2, modulo - 1]);
        let coeff = linear_fit(&first_terms, 0);
        assert!(coeff == vec![2, 2, u64::MAX]);
    }

    #[test]
    fn check_reeds_sloane_example_in_paper() {
        let first_terms = [6, 3, 1, 5, 6];
        let modulo = 9;
        let coeff = linear_fit(&first_terms, modulo);
        assert!(coeff == vec![5, 2, 8]);
    }

    #[test]
    fn check_reeds_sloane_many_zeros() {
        let first_terms = [0, 0, 0, 0, 1];
        let modulo = 998_244_353;
        let coeff = linear_fit(&first_terms, modulo);
        assert!(coeff == vec![0, 0, 0, 0, 0]);
    }
}
