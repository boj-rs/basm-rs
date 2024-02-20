use super::nttcore::*;
use alloc::{vec, vec::Vec};

fn mul_two_primes(out: &mut [u64], b: &[u64], c: &[u64], min_len: usize, l: usize, r: usize, modulo: u64) {
    let plan_x = NttPlan::build::<P2>(min_len);
    let plan_y = NttPlan::build::<P3>(min_len);
    let mut x = vec![0u64; plan_x.g + plan_x.n];
    let mut y = vec![0u64; plan_y.g + plan_y.n];
    let mut s = vec![0u64; max(x.len(), y.len())];

    /* convolution with modulo P2 */
    for i in 0..b.len() { x[plan_x.g + i] = if b[i] >= P2 { b[i] - P2 } else { b[i] }; }
    for i in 0..c.len() { s[plan_x.g + i] = if c[i] >= P2 { c[i] - P2 } else { c[i] }; }
    conv::<P2>(&plan_x, &mut x, b.len(), &mut s[..plan_x.g+plan_x.n], c.len(), arith::invmod(P3, P2));

    /* convolution with modulo P3 */
    for i in 0..b.len() { y[plan_y.g + i] = if b[i] >= P3 { b[i] - P3 } else { b[i] }; }
    for i in 0..c.len() { s[plan_y.g + i] = if c[i] >= P3 { c[i] - P3 } else { c[i] }; }
    (&mut s[plan_y.g..])[c.len()..plan_y.n].fill(0u64);
    conv::<P3>(&plan_y, &mut y, b.len(), &mut s[..plan_y.g+plan_y.n], c.len(), Arith::<P3>::submod(0, arith::invmod(P2, P3)));

    /* merge the results in {x, y} into acc */
    if modulo == 0 {
        let p2p3 = (P2 as u128 * P3 as u128) as u64;
        for i in l..r {
            /* extract the convolution result */
            let (v, overflow) = (x[i] as u128 * P3 as u128).overflowing_sub(y[i] as u128 * P2 as u128);
            let v = v as u64;
            out[i - l] = if overflow { v.wrapping_add(p2p3) } else { v };
        }
    } else {
        for i in l..r {
            /* extract the convolution result */
            let (mut v, overflow) = (x[i] as u128 * P3 as u128).overflowing_sub(y[i] as u128 * P2 as u128);
            if overflow { v = v.wrapping_add(P2 as u128 * P3 as u128); }
            out[i - l] = (v % modulo as u128) as u64;
        }
    }
}

fn mul_three_primes(out: &mut [u64], b: &[u64], c: &[u64], min_len: usize, l: usize, r: usize, modulo: u64) {
    let plan_x = NttPlan::build::<P1>(min_len);
    let plan_y = NttPlan::build::<P2>(min_len);
    let plan_z = NttPlan::build::<P3>(min_len);
    let mut x = vec![0u64; plan_x.g + plan_x.n];
    let mut y = vec![0u64; plan_y.g + plan_y.n];
    let mut z = vec![0u64; plan_z.g + plan_z.n];
    let mut s = vec![0u64; max(x.len(), max(y.len(), z.len()))];

    /* convolution with modulo P1 */
    for i in 0..b.len() { x[plan_x.g + i] = if b[i] >= P1 { b[i] - P1 } else { b[i] }; }
    for i in 0..c.len() { s[plan_x.g + i] = if c[i] >= P1 { c[i] - P1 } else { c[i] }; }
    conv::<P1>(&plan_x, &mut x, b.len(), &mut s[..plan_x.g+plan_x.n], c.len(), 1);

    /* convolution with modulo P2 */
    for i in 0..b.len() { y[plan_y.g + i] = if b[i] >= P2 { b[i] - P2 } else { b[i] }; }
    for i in 0..c.len() { s[plan_y.g + i] = if c[i] >= P2 { c[i] - P2 } else { c[i] }; }
    (&mut s[plan_y.g..])[c.len()..plan_y.n].fill(0u64);
    conv::<P2>(&plan_y, &mut y, b.len(), &mut s[..plan_y.g+plan_y.n], c.len(), 1);

    /* convolution with modulo P3 */
    for i in 0..b.len() { z[plan_z.g + i] = if b[i] >= P3 { b[i] - P3 } else { b[i] }; }
    for i in 0..c.len() { s[plan_z.g + i] = if c[i] >= P3 { c[i] - P3 } else { c[i] }; }
    (&mut s[plan_z.g..])[c.len()..plan_z.n].fill(0u64);
    conv::<P3>(&plan_z, &mut z, b.len(), &mut s[..plan_z.g+plan_z.n], c.len(), 1);

    /* merge the results in {x, y, z} into acc */
    let modulo_p64 = if modulo == 0 { 0 } else { (1u128 << 64) % modulo as u128 };
    let modulo_p128 = if modulo == 0 { 0 } else { (modulo_p64 * modulo_p64) % modulo as u128 };
    for i in l..r {
        let (a, b, c) = (x[i], y[i], z[i]);
        // We need to solve the following system of linear congruences:
        //     x === a mod P1,
        //     x === b mod P2,
        //     x === c mod P3.
        // The first two equations are equivalent to
        //     x === a + P1 * (U * (b-a) mod P2) mod P1P2,
        // where U is the solution to
        //     P1 * U === 1 mod P2.
        let bma = Arith::<P2>::submod(b, a);
        let u = Arith::<P2>::mmulmod(bma, P1INV_R_MOD_P2);
        let v = a as u128 + P1 as u128 * u as u128;
        let v_mod_p3 = Arith::<P3>::addmod(a, Arith::<P3>::mmulmod(P1_R_MOD_P3, u));
        // Now we have reduced the congruences into two:
        //     x === v mod P1P2,
        //     x === c mod P3.
        // The solution is
        //     x === v + P1P2 * (V * (c-v) mod P3) mod P1P2P3,
        // where V is the solution to
        //     P1P2 * V === 1 mod P3.
        let cmv = Arith::<P3>::submod(c, v_mod_p3);
        let vcmv = Arith::<P3>::mmulmod(cmv, P1P2INV_R_MOD_P3);
        let ans_01 = v + P1P2_LO as u128 * vcmv as u128;
        let ans_0 = ans_01 as u64;

        if modulo == 0 { /* modulo == 2^64 */
            out[i - l] = ans_0;
        } else { /* nonzero modulo */
            let ans_12 = P1P2_HI as u128 * vcmv as u128 + (ans_01 >> 64);
            let ans_1 = ans_12 as u64;
            let ans_2 = (ans_12 >> 64) as u64;
            let mut ans = (ans_0 % modulo) as u128;
            ans = (ans + (ans_1 as u128 * modulo_p64)) % modulo as u128;
            ans = (ans + (ans_2 as u128 * modulo_p128)) % modulo as u128;
            out[i - l] = ans as u64;
        }
    }
}

/// Multiplies two polynomials given by coefficients `x` and `y`, modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
/// 
/// This function accepts a range `[l, r)` for which the product is computed and
/// written in `out[..r - l]`. This way we reduce dynamic allocations.
/// 
/// Optimizations are applied if the range `[l, r)` is narrow enough,
/// so it can be faster than `polymul_u64` in some cases.
/// 
/// Inputs are assumed to satisfy the following conditions. Input validation is performed in dev builds only using `debug_assert!`.
///   - `0 <= l <= r <= x.len() + y.len() - 1`.
///   - `x.len() > 0` and `y.len() > 0`.
pub fn polymul_ex_u64(out: &mut [u64], x: &[u64], y: &[u64], l: usize, r: usize, modulo: u64) {
    debug_assert!(l <= r);
    if x.is_empty() || y.is_empty() { return; }

    // Output range is "l..r".
    let all_len = x.len() + y.len() - 1; // Output length without truncation
    debug_assert!(r <= all_len);

    // Handle naive cases.
    if r - l <= 40 {
        if modulo == 0 {
            for i in l..r {
                let lt_range = (i + 1).saturating_sub(y.len());
                let rt_range = min(i, x.len() - 1);
                let mut ans = 0u64;
                for j in lt_range..=rt_range {
                    ans = ans.wrapping_add(x[j].wrapping_mul(y[i - j]));
                }
                out[i - l] = ans;
            }
        } else {
            // Since modulo operation is expensive, we accumulate non-modulo-reduced data
            // and reduce only at the end.
            // Correctness:
            //   If overflow does not occur, we are all set.
            //   If overflow does occur, we have w < v.
            //     Since v <= (2^64 - 1) * (2^64 - 1) = 2^128 - 2 * 2^64 + 1,
            //       we have w < 2^128 - 2 * 2^64 + 1.
            //     Thus a second overflow cannot occur if we add to v the number
            //       pow128 = 2^128 mod modulo < modulo < 2^64 < 2 * 2^64 - 1
            //       to compensate for the missing 2^128.
            let pow128 = {
                let tmp = 0u64.wrapping_sub(modulo) % modulo;
                (tmp as u128 * tmp as u128) % modulo as u128
            };
            for i in l..r {
                let lt_range = (i + 1).saturating_sub(y.len());
                let rt_range = min(i, x.len() - 1);
                let mut ans = 0u128;
                for j in lt_range..=rt_range {
                    let v = x[j] as u128 * y[i - j] as u128;
                    let (mut w, overflow) = ans.overflowing_add(v);
                    if overflow {
                        w = w.wrapping_add(pow128);
                    }
                    ans = w;
                }
                out[i - l] = (ans % modulo as u128) as u64;
            }
        }
        return;
    }

    // No need to multiply beyond what is actually needed.
    let x = if x.len() <= r { x } else { &x[..r] };
    let y = if y.len() <= r { y } else { &y[..r] };

    // We ensure the invariants `min_len >= x.len()` and `min_len >= y.len()`.
    let min_len = max(x.len() + y.len() - 1 - l, r);

    // We estimate the maximum value of the convolution.
    // If they are small enough, we may reduce the number of
    // convolutions from 3 to 1 or 2. This will yield huge
    // savings in running time.
    let strategy = {
        let minlen = min(x.len(), y.len()) as u128;
        if modulo > 0 && modulo < 1u64 << 32 && (minlen * (modulo.wrapping_mul(modulo)) as u128) < P3 as u128 {
            1
        } else {
            let maxx = *x.iter().max().unwrap();
            let maxy = *y.iter().max().unwrap();
            let maxxy = maxx as u128 * maxy as u128;
            let (maxxylen, overflow) = maxxy.overflowing_mul(minlen);
            if overflow || maxxylen >= P2 as u128 * P3 as u128 {
                3
            } else if maxxylen >= P3 as u128 {
                2
            } else {
                1
            }
        }
    };
    if strategy == 3 {
        mul_three_primes(out, x, y, min_len, l, r, modulo);
    } else if strategy == 2 {
        mul_two_primes(out, x, y, min_len, l, r, modulo);
    } else { /* strategy == 1 */
        let plan = NttPlan::build::<P3>(min_len);
        let mut t = vec![0u64; plan.g + plan.n];
        let mut s = vec![0u64; plan.g + plan.n];

        /* Convolution with modulo P3. We don't compare with and subtract P3,
         * since we have already ensured the maximum value is less than P3.
         */
        for i in 0..x.len() { t[plan.g + i] = x[i]; }
        for i in 0..y.len() { s[plan.g + i] = y[i]; }
        conv::<P3>(&plan, &mut t[..plan.g+plan.n], x.len(), &mut s[..plan.g+plan.n], y.len(), 1);

        /* copy the result along with modular reduction */
        if modulo == 0 {
            out[..(r - l)].copy_from_slice(&t[l..r]);
        } else {
            for i in l..r {
                out[i - l] = t[i] % modulo;
            }
        }
    }
}

/// Multiplies two polynomials given by coefficients `x` and `y`, modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// If either of the inputs is empty, the result will be an empty Vec.
/// Otherwise the output will have length equal to `x.len() + y.len() - 1`.
/// 
/// Note that `modulo` does not need to be a prime.
///
/// Example:
/// `polymul_u64(&[1 << 32, 1 << 32], &[1 << 32], 0)` returns `vec![0, 0]`.
pub fn polymul_u64(x: &[u64], y: &[u64], modulo: u64) -> Vec<u64> {
    if x.is_empty() || y.is_empty() {
        Vec::<u64>::new()
    } else {
        // Naive case optimization is implemented in polymul_ex_u64.
        let out_len = x.len() + y.len() - 1;
        let mut out = vec![0; out_len];
        polymul_ex_u64(&mut out, x, y, 0, out_len, modulo);
        out
    }
}