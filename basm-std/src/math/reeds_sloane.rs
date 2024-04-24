use alloc::{vec, vec::Vec};
use core::cmp::max;
use crate::math::{factorize, modadd, modinv, modmul, modsub};

fn reeds_sloane_prime_power(first_terms: &[u64], p: u64, e: usize) -> Vec<u64> {
    let n = first_terms.len();
    assert!(n >= 2);
    assert!(p >= 2);
    assert!(e >= 1);

    let ppow: Vec<u64> = (0..=e).map(|x| p.pow(x as u32)).collect();

    let p_e = ppow[e];
    let s: Vec<u64> = first_terms.iter().map(|&x| if p_e > 0 { x % p_e } else { x }).collect();

    // Utility functions (modulo operations)
    let modadd_p_e = |x: u64, y: u64| -> u64 {
        if p_e == 0 { x.wrapping_add(y) } else { modadd(x, y, p_e) }
    };
    let modsub_p_e = |x: u64, y: u64| -> u64 {
        if p_e == 0 { x.wrapping_sub(y) } else { modsub(x, y, p_e) }
    };
    let modmul_p_e = |x: u64, y: u64| -> u64 {
        if p_e == 0 { x.wrapping_mul(y) } else { modmul(x, y, p_e) }
    };
    let modinv_p_e = |x: u64| -> u64 {
        if p_e == 0 { modinv(x as u128, 1u128 << 64).unwrap() as u64 } else { modinv(x, p_e).unwrap() }
    };

    // Utility functions (core logic)
    fn l(a: &[u64], b: &[u64]) -> usize {
        max(a.len() - 1, b.len())
    }
    let factor_by_p = |x: u64| -> (u64, usize) {
        // Returns (theta, u)
        let (mut lo, mut hi) = (0, e);
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if x % ppow[mid] == 0 {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        (x / ppow[lo], lo)
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
        let p_eta_s0 = modmul_p_e(p_eta, s[0]);
        b_new.push(if p_eta_s0 == 0 { vec![] } else { vec![p_eta_s0] } );
        let c = modmul_p_e(s[0], p_eta);
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
        a.clone_from_slice(&a_new);
        // Part 3
        for eta in 0..e {
            let mut c = modsub_p_e(0, if b[eta].len() > k { b[eta][k] } else { 0 });
            for j in 0..=k {
                if a[eta].len() > j {
                    c = modadd_p_e(c, modmul_p_e(s[k - j], a[eta][j]));
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
                    tmp[k] = modadd_p_e(tmp[k], modmul_p_e(theta[eta], ppow[eta]));
                    b_new[eta] = tmp;
                } else {
                    // Case IIb
                    let theta_g_old_inv = modinv_p_e(theta_old[g]);
                    let m = modmul_p_e(theta[eta], theta_g_old_inv);
                    let m = modmul_p_e(m, ppow[u[eta] - u_old[g]]);
                    let d = k - r[g];
                    let mut tmp = a[eta].clone();
                    if tmp.len() < a_old[g].len() + d {
                        tmp.resize(a_old[g].len() + d, 0);
                    }
                    for j in 0..a_old[g].len() {
                        tmp[j + d] = modsub_p_e(tmp[j + d], modmul_p_e(m, a_old[g][j]));
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
                        tmp[j + d] = modsub_p_e(tmp[j + d], modmul_p_e(m, b_old[g][j]));
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
    for i in 1..a_new[0].len() {
        out.push(modsub_p_e(0, a_new[0][i]));
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