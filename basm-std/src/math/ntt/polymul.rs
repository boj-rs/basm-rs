use super::nttcore::*;
use alloc::vec;
use alloc::vec::Vec;

fn mac3mod_three_primes(acc: &mut [u64], b: &[u64], c: &[u64], modulo: u64) {
    let min_len = b.len() + c.len();
    let plan_x = NttPlan::build::<P1>(min_len);
    let plan_y = NttPlan::build::<P2>(min_len);
    let plan_z = NttPlan::build::<P3>(min_len);
    let mut x = vec![0u64; plan_x.g + plan_x.n];
    let mut y = vec![0u64; plan_y.g + plan_y.n];
    let mut z = vec![0u64; plan_z.g + plan_z.n];
    let mut r = vec![0u64; max(x.len(), max(y.len(), z.len()))];

    /* convolution with modulo P1 */
    for i in 0..b.len() { x[plan_x.g + i] = if b[i] >= P1 { b[i] - P1 } else { b[i] }; }
    for i in 0..c.len() { r[plan_x.g + i] = if c[i] >= P1 { c[i] - P1 } else { c[i] }; }
    conv::<P1>(&plan_x, &mut x, b.len(), &mut r[..plan_x.g+plan_x.n], c.len(), 1);

    /* convolution with modulo P2 */
    for i in 0..b.len() { y[plan_y.g + i] = if b[i] >= P2 { b[i] - P2 } else { b[i] }; }
    for i in 0..c.len() { r[plan_y.g + i] = if c[i] >= P2 { c[i] - P2 } else { c[i] }; }
    (&mut r[plan_y.g..])[c.len()..plan_y.n].fill(0u64);
    conv::<P2>(&plan_y, &mut y, b.len(), &mut r[..plan_y.g+plan_y.n], c.len(), 1);

    /* convolution with modulo P3 */
    for i in 0..b.len() { z[plan_z.g + i] = if b[i] >= P3 { b[i] - P3 } else { b[i] }; }
    for i in 0..c.len() { r[plan_z.g + i] = if c[i] >= P3 { c[i] - P3 } else { c[i] }; }
    (&mut r[plan_z.g..])[c.len()..plan_z.n].fill(0u64);
    conv::<P3>(&plan_z, &mut z, b.len(), &mut r[..plan_z.g+plan_z.n], c.len(), 1);

    /* merge the results in {x, y, z} into acc (process carry along the way) */
    let modulo_p64 = if modulo == 0 { 0 } else { (1u128 << 64) % modulo as u128 };
    let modulo_p128 = if modulo == 0 { 0 } else { (modulo_p64 * modulo_p64) % modulo as u128 };
    for i in 0..min_len {
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
        let out_01 = v + P1P2_LO as u128 * vcmv as u128;
        let out_0 = out_01 as u64;

        if modulo == 0 { /* modulo == 2^64 */
            acc[i] = acc[i].wrapping_add(out_0);
        } else { /* nonzero modulo */
            let out_12 = P1P2_HI as u128 * vcmv as u128 + (out_01 >> 64);
            let out_1 = out_12 as u64;
            let out_2 = (out_12 >> 64) as u64;
            let mut out = acc[i] as u128;
            out = (out + out_0 as u128) % modulo as u128;
            out = (out + (out_1 as u128 * modulo_p64)) % modulo as u128;
            out = (out + (out_2 as u128 * modulo_p128)) % modulo as u128;
            acc[i] = out as u64;
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
        // We estimate the maximum value of the convolution.
        // If they are small enough, we may reduce the number of
        // convolutions from 3 to 1 or 2. This will yield huge
        // savings in running time.
        let strategy = {
            let maxx = *x.iter().max().unwrap();
            let maxy = *y.iter().max().unwrap();
            let maxxy = maxx as u128 * maxy as u128;
            let maxlen = min(x.len(), y.len()) as u128;
            let (maxxylen, overflow) = maxxy.overflowing_mul(maxlen);
            if overflow || maxxylen >= P2 as u128 * P3 as u128 {
                3
            } else if maxxylen >= P3 as u128 {
                2
            } else {
                1
            }
        };
        let mut out = vec![];
        if strategy == 3 || strategy == 2 {
            out.resize(x.len() + y.len() + 1, 0);
            mac3mod_three_primes(&mut out, x, y, modulo);
        } else { /* strategy == 1 */
            let min_len = x.len() + y.len();
            let plan = NttPlan::build::<P3>(min_len);
            let mut r = vec![0u64; plan.g + plan.n];

            /* convolution with modulo P3 */
            out.resize(plan.g + plan.n, 0);
            for i in 0..x.len() { out[plan.g + i] = if x[i] >= P3 { x[i] - P3 } else { x[i] }; }
            for i in 0..y.len() { r[plan.g + i] = if y[i] >= P3 { y[i] - P3 } else { y[i] }; }
            conv::<P3>(&plan, &mut out[..plan.g+plan.n], x.len(), &mut r[..plan.g+plan.n], y.len(), 1);
        }
        out.resize(x.len() + y.len() - 1, 0);
        out
    }
}