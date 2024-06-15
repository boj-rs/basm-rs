use super::nttcore::*;
use alloc::vec;
use alloc::vec::Vec;

fn mac3_two_primes(acc: &mut [u64], b: &[u64], c: &[u64], bits: u64) {
    assert!(bits < 63);
    let b_len = ((64 * b.len() as u64 + bits - 1) / bits) as usize;
    let c_len = ((64 * c.len() as u64 + bits - 1) / bits) as usize;
    let min_len = b_len + c_len;
    let plan_x = NttPlan::build::<P2>(min_len);
    let plan_y = NttPlan::build::<P3>(min_len);

    let mut x = vec![0u64; plan_x.g + plan_x.n];
    let mut y = vec![0u64; plan_y.g + plan_y.n];
    let mut r = vec![0u64; plan_x.g + plan_x.n];
    let mut s = vec![0u64; plan_y.g + plan_y.n];
    pack_into(b, &mut x[plan_x.g..], &mut y[plan_y.g..], bits);
    pack_into(c, &mut r[plan_x.g..], &mut s[plan_y.g..], bits);
    conv::<P2>(
        &plan_x,
        &mut x,
        b_len,
        &mut r[..plan_x.g + plan_x.n],
        c_len,
        arith::invmod(P3, P2),
    );
    conv::<P3>(
        &plan_y,
        &mut y,
        b_len,
        &mut s[..plan_y.g + plan_y.n],
        c_len,
        Arith::<P3>::submod(0, arith::invmod(P2, P3)),
    );

    /* merge the results in {x, y} into r (process carry along the way) */
    let mask = (1u64 << bits) - 1;
    let mut carry: u128 = 0;
    let (mut j, mut p) = (0usize, 0u64);
    let mut s: u64 = 0;
    let mut carry_acc: u64 = 0;
    for i in 0..min_len {
        /* extract the convolution result */
        let (a, b) = (x[i], y[i]);
        let (mut v, overflow) =
            (a as u128 * P3 as u128 + carry).overflowing_sub(b as u128 * P2 as u128);
        if overflow {
            v = v.wrapping_add(P2 as u128 * P3 as u128);
        }
        carry = v >> bits;

        /* write to s */
        let out = (v as u64) & mask;
        s |= out << p;
        p += bits;
        if p >= 64 {
            /* flush s to the output buffer */
            let (w, overflow1) = s.overflowing_add(carry_acc);
            let (w, overflow2) = acc[j].overflowing_add(w);
            acc[j] = w;
            carry_acc = u64::from(overflow1 || overflow2);

            /* roll-over */
            (j, p) = (j + 1, p - 64);
            s = out >> (bits - p);
        }
    }
    // Process remaining carries. The addition carry_acc + s should not overflow
    //   since s is underfilled and carry_acc is always 0 or 1.
    propagate_carry(&mut acc[j..], carry_acc + s);
}

fn mac3_three_primes(acc: &mut [u64], b: &[u64], c: &[u64]) {
    let min_len = b.len() + c.len();
    let plan_x = NttPlan::build::<P1>(min_len);
    let plan_y = NttPlan::build::<P2>(min_len);
    let plan_z = NttPlan::build::<P3>(min_len);
    let mut x = vec![0u64; plan_x.g + plan_x.n];
    let mut y = vec![0u64; plan_y.g + plan_y.n];
    let mut z = vec![0u64; plan_z.g + plan_z.n];
    let mut r = vec![0u64; max(x.len(), max(y.len(), z.len()))];

    /* convolution with modulo P1 */
    for i in 0..b.len() {
        x[plan_x.g + i] = if b[i] >= P1 { b[i] - P1 } else { b[i] };
    }
    for i in 0..c.len() {
        r[plan_x.g + i] = if c[i] >= P1 { c[i] - P1 } else { c[i] };
    }
    conv::<P1>(
        &plan_x,
        &mut x,
        b.len(),
        &mut r[..plan_x.g + plan_x.n],
        c.len(),
        1,
    );

    /* convolution with modulo P2 */
    for i in 0..b.len() {
        y[plan_y.g + i] = if b[i] >= P2 { b[i] - P2 } else { b[i] };
    }
    for i in 0..c.len() {
        r[plan_y.g + i] = if c[i] >= P2 { c[i] - P2 } else { c[i] };
    }
    (&mut r[plan_y.g..])[c.len()..plan_y.n].fill(0u64);
    conv::<P2>(
        &plan_y,
        &mut y,
        b.len(),
        &mut r[..plan_y.g + plan_y.n],
        c.len(),
        1,
    );

    /* convolution with modulo P3 */
    for i in 0..b.len() {
        z[plan_z.g + i] = if b[i] >= P3 { b[i] - P3 } else { b[i] };
    }
    for i in 0..c.len() {
        r[plan_z.g + i] = if c[i] >= P3 { c[i] - P3 } else { c[i] };
    }
    (&mut r[plan_z.g..])[c.len()..plan_z.n].fill(0u64);
    conv::<P3>(
        &plan_z,
        &mut z,
        b.len(),
        &mut r[..plan_z.g + plan_z.n],
        c.len(),
        1,
    );

    /* merge the results in {x, y, z} into acc (process carry along the way) */
    let mut carry: u128 = 0;
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
        let (out_01, overflow) = carry.overflowing_add(v + P1P2_LO as u128 * vcmv as u128);
        let out_0 = out_01 as u64;
        let out_12 = P1P2_HI as u128 * vcmv as u128
            + (out_01 >> 64)
            + if overflow { 1u128 << 64 } else { 0 };

        let (v, overflow) = acc[i].overflowing_add(out_0);
        acc[i] = v;
        carry = out_12 + u128::from(overflow);
    }
    propagate_carry(&mut acc[min_len..], carry as u64);
}

fn mac3_u64(acc: &mut [u64], b: &[u64], c: &[u64]) {
    let (b, c) = if b.len() < c.len() { (b, c) } else { (c, b) };
    let naive_cost = NttPlan::build::<P1>(b.len() + c.len()).cost;
    let split_cost = NttPlan::build::<P1>(b.len() + b.len()).cost * (c.len() / b.len())
        + if c.len() % b.len() > 0 {
            NttPlan::build::<P1>(b.len() + (c.len() % b.len())).cost
        } else {
            0
        };
    if b.len() >= 128 && split_cost < naive_cost {
        /* special handling for unbalanced multiplication:
        we reduce it to about `c.len()/b.len()` balanced multiplications */
        let mut i = 0usize;
        let mut carry = 0u64;
        while i < c.len() {
            let j = min(i + b.len(), c.len());
            let k = j + b.len();
            let tmp = acc[k];
            acc[k] = 0;
            mac3_u64(&mut acc[i..=k], b, &c[i..j]);
            (acc[k], carry) = (tmp, acc[k] + propagate_carry(&mut acc[j..k], carry));
            i = j;
        }
        propagate_carry(&mut acc[i + b.len()..], carry);
        return;
    }

    // We have two choices:
    //     1. NTT with two primes.
    //     2. NTT with three primes.
    // Obviously we want to do only two passes for efficiency, not three.
    // However, the number of bits per u64 we can pack for NTT
    // depends on the length of the arrays being multiplied (convolved).
    // If the arrays are too long, the resulting values may exceed the
    // modulus range P2 * P3, which leads to incorrect results.
    // Hence, we compute the number of bits required by the length of NTT,
    // and use it to determine whether to use two-prime or three-prime.
    // Since we can pack 64 bits per u64 in three-prime NTT, the effective
    // number of bits in three-prime NTT is 64/3 = 21.3333..., which means
    // two-prime NTT can only do better when at least 43 bits per u64 can
    // be packed into each u64.
    let max_cnt = max(b.len(), c.len()) as u64;
    let bits = compute_bits(max_cnt);
    if bits >= 43 {
        /* can pack more effective bits per u64 with two primes than with three primes */
        mac3_two_primes(acc, b, c, bits);
    } else {
        /* can pack at most 21 effective bits per u64, which is worse than
        64/3 = 21.3333.. effective bits per u64 achieved with three primes */
        mac3_three_primes(acc, b, c);
    }
}

/// Multiplies two integers `x` and `y`.
/// Least significant digits come first.
/// If either of the inputs is empty, the result will be an empty Vec.
/// Otherwise the output will have length equal to `x.len() + y.len()`.
///
/// Example:
/// `multiply_u64(&[1 << 32, 1 << 32], &[1 << 32])` returns `vec![0, 1, 1]`.
pub fn multiply_u64(x: &[u64], y: &[u64]) -> Vec<u64> {
    if x.is_empty() || y.is_empty() {
        Vec::<u64>::new()
    } else {
        let mut out = vec![0; x.len() + y.len() + 1];
        mac3_u64(&mut out, x, y);
        out.resize(x.len() + y.len(), 0);
        out
    }
}
