// This is a Rust port of the implementation originally written in C++ by wider93.

use crate::math::{
    gcd, is_prime_u64,
    miller_rabin::{M, OddMont},
};
use alloc::{vec, vec::Vec};

trait PollardRhoOp<T> {
    fn pollard_rho_type(r: T, init: T) -> T;
}

macro_rules! impl_pollard_rho_type {
    ($ty: ident, $ty_large: ident) => {
        impl PollardRhoOp<$ty> for $ty {
            fn pollard_rho_type(r: $ty, init: $ty) -> $ty {
                let rm = OddMont::<$ty>::new(r);
                let mut x0 = 2;
                let mut k = r;
                const STEP: u32 = 1 << 10;
                while k == r {
                    let mut y = M::<$ty> { v: x0 };
                    x0 += 1;
                    k = 1;
                    let mut itr = STEP;
                    while k == 1 && itr < (1 << 21) {
                        let mut g = M::<$ty> { v: 1 };
                        let x = y;
                        for _ in (0..itr).step_by(STEP as usize) {
                            if k != 1 {
                                break;
                            }
                            for _ in 0..STEP {
                                y = rm
                                    .redc(y.v as $ty_large * y.v as $ty_large + init as $ty_large);
                                g = rm.mul(g, rm.to_mont(x.v.abs_diff(y.v)));
                            }
                            k = gcd(g.v, r);
                            if k == r {
                                k = 1;
                                let mut py = x;
                                for _ in 0..STEP {
                                    py = rm.redc(
                                        py.v as $ty_large * py.v as $ty_large + init as $ty_large,
                                    );
                                    k = gcd(r, x.v.abs_diff(py.v));
                                    if k != 1 {
                                        break;
                                    }
                                }
                                if k == 1 {
                                    k = r;
                                }
                            }
                        }
                        itr <<= 1;
                    }
                }
                k
            }
        }
    };
}
impl_pollard_rho_type!(u32, u64);
impl_pollard_rho_type!(u64, u128);

fn pollard_rho(r: u64) -> u64 {
    if r < (1u64 << 32) {
        u32::pollard_rho_type(r as u32, 1) as u64
    } else {
        u64::pollard_rho_type(r, 1)
    }
}

/// Returns a `Vec<u64>` containing the result of prime factorization in ascending order.
///
/// `n` must be greater than zero.
/// ```ignore
/// use basm_std::math::factorize;
/// assert_eq!(vec![2, 2, 2, 3], factorize(24));
/// assert_eq!(vec![2, 2, 5, 17], factorize(340));
/// ```
pub fn factorize(mut n: u64) -> Vec<u64> {
    assert!(n != 0);

    let mut v = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while n % p == 0 {
            v.push(p);
            n /= p;
        }
    }
    if n == 1 {
        return v;
    }
    let mut q = vec![(n, 1)];
    while let Some((t, num)) = q.pop() {
        if t < 1681 || is_prime_u64(t) {
            for _ in 0..num {
                v.push(t);
            }
        } else {
            let s = (libm::sqrt(t as f64) + 0.5) as u64;
            if s * s == t {
                q.push((s, num << 1));
                continue;
            }
            let a = pollard_rho(t);
            debug_assert!(a > 1);
            debug_assert!(a < t);
            debug_assert!(t % a == 0);
            q.push((a, num));
            q.push((t / a, num));
        }
    }
    v.sort_unstable();
    v
}

/// Returns a `Vec<(u64, u32)>` containing the result of prime factorization.
/// Each tuple `(p, k)` indicates the prime `p` appears `k` times in the factorization of `n`.
/// The `Vec` is sorted by `p` in ascending order.
///
/// `n` must be greater than zero.
/// ```ignore
/// use basm_std::math::factorize;
/// assert_eq!(vec![(2, 3), (3, 1)], factorize_dedup(24));
/// assert_eq!(vec![(2, 2), (5, 1), (17, 1)], factorize_dedup(340));
/// ```
pub fn factorize_dedup(n: u64) -> Vec<(u64, u32)> {
    let factors = factorize(n);
    let mut ans: Vec<(u64, u32)> = vec![];
    for f in factors {
        if let Some(x) = ans.last_mut()
            && x.0 == f
        {
            x.1 += 1;
        } else {
            ans.push((f, 1u32));
        }
    }
    ans
}

/// Returns a `Vec<u64>` containing the positive divisors in ascending order.
///
/// `n` must be greater than zero.
/// ```ignore
/// use basm_std::math::divisors;
/// assert_eq!(vec![1, 2, 3, 4, 6, 8, 12, 24], divisors(24));
/// assert_eq!(vec![1, 2, 4, 5, 10, 17, 20, 34, 68, 85, 170, 340], divisors(340));
/// ```
pub fn divisors(n: u64) -> Vec<u64> {
    let factors_dedup = factorize_dedup(n);
    let mut ans: Vec<u64> = vec![1];
    for (p, i) in factors_dedup {
        let prev = ans.clone();
        for mut x in prev {
            for _ in 0..i {
                x *= p;
                ans.push(x);
            }
        }
    }
    ans.sort_unstable();
    ans
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn check_factorize() {
        assert_eq!(vec![2, 2, 3], factorize(12));
        assert_eq!(
            vec![3, 3, 13, 179, 271, 1381, 2423],
            factorize(18991325453139)
        );
        assert_eq!(vec![34421, 133978850655919], factorize(4611686018427387899));
        assert_eq!(
            vec![2, 2, 3, 3, 5, 5, 7, 11, 13, 31, 41, 61, 151, 331, 1321],
            factorize(4611686018427387900)
        );
        assert_eq!(
            vec![37, 9902437, 12586817029],
            factorize(4611686018427387901)
        );
        assert_eq!(vec![2, 2305843009213693951], factorize(4611686018427387902));
        assert_eq!(
            vec![3, 715827883, 2147483647],
            factorize(4611686018427387903)
        );
    }

    #[test]
    fn check_factorize_dedup() {
        assert_eq!(vec![(0u64, 0u32); 0], factorize_dedup(1));
        assert_eq!(vec![(2, 3), (3, 1)], factorize_dedup(24));
        assert_eq!(vec![(2, 2), (5, 1), (17, 1)], factorize_dedup(340));
    }

    #[test]
    fn check_divisors() {
        assert_eq!(vec![1], divisors(1));
        assert_eq!(vec![1, 2, 3, 4, 6, 8, 12, 24], divisors(24));
        assert_eq!(
            vec![1, 2, 4, 5, 10, 17, 20, 34, 68, 85, 170, 340],
            divisors(340)
        );
    }
}
