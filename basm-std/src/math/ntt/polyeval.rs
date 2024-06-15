use super::{polymul_ex_u64, polyops::polyneginv_u64};
use alloc::{vec, vec::Vec};
use core::cmp::min;

/// Computes the degree sum of the heap-style binary tree of `n` elements.
fn deg_sum(n: usize) -> usize {
    if n <= 1 {
        n
    } else {
        let k = (usize::BITS - n.leading_zeros() - 1) as usize;
        let m = 3usize << (k - 1);
        if n < m {
            let powkm1 = 1usize << (k - 1);
            n + k * powkm1 + deg_sum(n - powkm1)
        } else {
            let powk = 1usize << k;
            n + (k + 1) * powk + deg_sum(n - powk)
        }
    }
}

/// Performs multipoint evaluation of the input polynomial `poly`
/// at points specified by `query_points`. `poly[i]` should be the coefficient of `x**i`.
///
/// The time complexity is `O((n + q) lg^2 (n + q))`,
/// where `n` is degree of the input polynomial and `q` is the number of query points.
///
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
pub fn polyeval_u64(poly: &[u64], query_points: &[u64], modulo: u64) -> Vec<u64> {
    assert!(!poly.is_empty());

    let (d, n) = (poly.len() - 1, query_points.len());
    if modulo == 1 || n == 0 {
        return vec![0; n];
    }

    let tree1_len = deg_sum(n) + 2 * n - 1;
    let mut tree1 = vec![modulo.wrapping_sub(1); tree1_len];
    let mut tree1_pos = vec![0; 2 * n];
    let mut pos = 0;
    for i in (0..n).rev() {
        tree1_pos[n + i] = pos;
        tree1[pos] = query_points[i];
        pos += 2;
    }
    for i in (1..n).rev() {
        unsafe {
            tree1_pos[i] = pos;
            let tree1_ref = &*core::ptr::slice_from_raw_parts(tree1.as_ptr(), pos);
            let (ls, le) = (tree1_pos[i << 1], tree1_pos[(i << 1) - 1]);
            let (rs, re) = (tree1_pos[(i << 1) | 1], tree1_pos[i << 1]);
            let l = (le - ls) + (re - rs) - 1;
            polymul_ex_u64(
                &mut tree1[pos..],
                &tree1_ref[ls..le],
                &tree1_ref[rs..re],
                0,
                l,
                modulo,
            );
            pos += l;
        }
    }
    tree1_pos[0] = pos;

    let root = &mut tree1[tree1_pos[1]..];
    root.reverse();
    let mut inv = polyneginv_u64(root, d + 1, modulo).unwrap();
    inv.reverse();

    let (mut tree2_0, mut tree2_1) = (vec![0; n], vec![0; n]);
    polymul_ex_u64(&mut tree2_0, poly, &inv, d, d + min(d + 1, n), modulo);

    let (mut begin, mut end) = (2, min(4, 2 * n));
    loop {
        let mut pos0 = 0;
        let mut pos1 = 0;
        for i in begin..end {
            let pq_deg = tree1_pos[(i >> 1) - 1] - tree1_pos[i >> 1] - 1;
            let q = &tree1[tree1_pos[i ^ 1]..tree1_pos[(i ^ 1) - 1]];
            let q_deg = q.len() - 1;
            polymul_ex_u64(
                &mut tree2_1[pos1..],
                &tree2_0[pos0..],
                q,
                q_deg,
                pq_deg,
                modulo,
            );
            if i & 1 != 0 {
                pos0 += pq_deg;
            }
            pos1 += pq_deg - q_deg;
        }
        if end >= 2 * n {
            break;
        }
        begin *= 2;
        end = min(2 * begin, 2 * n);
        (tree2_0, tree2_1) = (tree2_1, tree2_0);
    }

    let mut out = Vec::with_capacity(n);
    let d = begin - n;
    out.extend_from_slice(&tree2_0[n - d..]);
    out.extend_from_slice(&tree2_1[..n - d]);
    out
}
