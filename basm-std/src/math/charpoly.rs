#![allow(clippy::needless_range_loop)]

use crate::math::modint_dynamic::FastModOps;
use alloc::{vec, vec::Vec};

fn swap_row(x: &mut [Vec<u64>], i: usize, j: usize) {
    // swap rows
    x.swap(i, j);
}
fn swap_col(x: &mut [Vec<u64>], i: usize, j: usize) {
    // swap columns
    for k in 0..x.len() {
        x[k].swap(i, j);
    }
}

fn mul2x2_row(x: &mut [Vec<u64>], src: usize, dst: usize, mul: [[u64; 2]; 2], ops: &FastModOps) {
    assert!(src != dst);

    // add rows
    let [[p, q], [r, s]] = mul;
    let (p, q, r, s) = (ops.premul(p), ops.premul(q), ops.premul(r), ops.premul(s));
    for k in 0..x.len() {
        let (a, b) = (x[src][k], x[dst][k]);
        x[src][k] = ops.add(p(a), q(b));
        x[dst][k] = ops.add(r(a), s(b));
    }
}
fn mul2x2_col(x: &mut [Vec<u64>], src: usize, dst: usize, mul: [[u64; 2]; 2], ops: &FastModOps) {
    assert!(src != dst);

    // add columns
    let [[p, q], [r, s]] = mul;
    let (p, q, r, s) = (ops.premul(p), ops.premul(q), ops.premul(r), ops.premul(s));
    for k in 0..x.len() {
        let (a, b) = (x[k][src], x[k][dst]);
        x[k][src] = ops.add(p(a), r(b));
        x[k][dst] = ops.add(q(a), s(b));
    }
}

/// Returns the matrix which takes [a, b] to [g, 0],
/// where g = gcd(a, b). Note that `modulo` of zero is treated as `2**64``.
///
/// `modulo` is used for modular reduction of the returned matrix.
///
/// Note that we assume `modulo` does not equal 1.
fn egcd_matrix(mut a: u64, mut b: u64, ops: &FastModOps) -> [[u64; 2]; 2] {
    let (mut c, mut parity) = if a < b {
        (a, b) = (b, a);
        ([0, 1, 1, 0], true)
    } else {
        ([1, 0, 0, 1], false)
    };
    // Invariant: a>=b
    while b != 0 {
        let (q, r) = (a / b, a % b);
        (a, b) = (b, r);
        c = [
            c[2],
            c[3],
            ops.fnmadd(ops.canonicalize(q), c[2], c[0]),
            ops.fnmadd(ops.canonicalize(q), c[3], c[1]),
        ];
        parity = !parity;
    }
    if parity {
        c[2] = ops.neg(c[2]);
        c[3] = ops.neg(c[3]);
    }
    [[c[0], c[1]], [c[2], c[3]]]
}

/// Computes the characteristic polynomial of the given square matrix `x`.
///
/// The returned `Vec` has length `n+1` if `x` is an n-by-n matrix.
/// This function will panic if `n` is zero.
///
/// If `modulo` equals 0, it is treated as `2**64`. Note that `modulo` does not need to be a prime.
///
/// Example:
/// `charpoly_u64(&[[5, 4], [1, 8]], 10000)` returns `vec![36, 9987, 1]`.
pub fn charpoly_u64<T>(x: &[T], modulo: u64) -> Vec<u64>
where
    T: AsRef<[u64]>,
{
    let n = x.len();
    assert!(n > 0);
    for i in 0..n {
        assert_eq!(n, x[i].as_ref().len());
    }
    if modulo == 1 {
        return vec![0; n + 1];
    }

    let ops = FastModOps::new(modulo);
    let mut m = Vec::with_capacity(n);
    for i in 0..n {
        m.push(x[i].as_ref().to_vec());
        for v in m[i].iter_mut() {
            // Negate the numbers and canonicalize and the modulo representation, in case it is not.
            *v = ops.neg(ops.canonicalize(*v));
        }
    }

    // Step 1: Compute Hessenberg matrix
    for c in 0..n - 1 {
        let mut k = c + 1;
        while k < n {
            if m[k][c] != 0 {
                break;
            }
            k += 1;
        }
        if k == n {
            // No non-zero entry found in this column below diagonal
            continue;
        }

        // Swap rows c+1 and k
        // (and columns too, to keep the invariant, which does not incur a violation)
        if c + 1 != k {
            swap_row(&mut m, c + 1, k);
            swap_col(&mut m, c + 1, k);
        }

        // Reduce rows below c+1
        for r in c + 2..n {
            let mul = egcd_matrix(m[c + 1][c], m[r][c], &ops);
            let invmul = [
                [mul[1][1], ops.neg(mul[0][1])],
                [ops.neg(mul[1][0]), mul[0][0]],
            ];
            mul2x2_row(&mut m, c + 1, r, mul, &ops);
            mul2x2_col(&mut m, c + 1, r, invmul, &ops);
        }
    }

    // Step 2: Compute the characteristic polynomial via DP
    // When we have examined up to column c,
    //   dp[i] = (sum of determinants with row i missing) (0 <= i <= c)
    //   dp[c+1] = (determinant of upper left matrix m[0..=c][0..=c])
    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = if n >= 2 { m[1][0] } else { 0 };
    dp[1][0] = m[0][0];
    dp[1][1] = 1;
    for c in 1..n {
        // dp[c+1]
        for r in 0..=c {
            let inv_cnt = c - r;
            let mut mul = m[r][c];
            if inv_cnt % 2 == 1 {
                mul = ops.neg(mul);
            }
            for j in 0..=c + 1 {
                dp[c + 1][j] = ops.fmadd(dp[r][j], mul, dp[c + 1][j]);
            }
            if r == c {
                for j in 1..=c + 1 {
                    dp[c + 1][j] = ops.add(dp[c + 1][j], dp[r][j - 1]);
                }
            }
        }
        // dp[0..=c]
        if c + 1 < n {
            let y = m[c + 1][c];
            for r in 0..=c {
                for j in 0..=c + 1 {
                    dp[r][j] = ops.mul(dp[r][j], y);
                }
            }
        }
    }
    dp.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_charpoly_u64() {
        assert_eq!(vec![u64::MAX, 1], charpoly_u64(&[[1]], 0));
        assert_eq!(vec![u64::MAX - 1, 1], charpoly_u64(&[[1]], u64::MAX));
        assert_eq!(vec![36, 9987, 1], charpoly_u64(&[[5, 4], [1, 8]], 10000));
    }
}
