use alloc::vec;
use alloc::vec::Vec;
use super::miller_rabin::is_prime_u64;
use super::gcd_u64;

use rand::{RngCore, SeedableRng};
use rand::rngs::SmallRng;

fn rho(n: u64, small_rng: &mut SmallRng) -> u64 {
    if n % 2 == 0 { return 2; }

    let x0 = 2;
    let c = small_rng.next_u64() % n;
    let f = |x: u64| -> u64 {
        let x2 = ((x as u128 * x as u128) % n as u128) as u64;
        (x2 + c) % n
    };

    let (mut p, mut q, mut g) = (x0, x0, 1);
    while g == 1 {
        p = f(p);
        q = f(f(q));
        if p == q {
            g = n;
        } else {
            g = gcd_u64(p.abs_diff(q), n);
        }
    }
    g
}

pub fn factorize(mut x: u64) -> Vec<u64> {
    assert!(x > 0);
    let mut factors = vec![];
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while x % p == 0 {
            factors.push(p);
            x /= p;
        }
    }
    let mut q = vec![];
    if x > 1 {
        q.push(x);
    }

    let random_thing = [0; 1];
    let mut small_rng = SmallRng::seed_from_u64(random_thing.as_ptr() as usize as u64);
    while let Some(n) = q.pop() {
        if is_prime_u64(n) {
            factors.push(n);
        } else {
            let mut f;
            loop {
                f = rho(n, &mut small_rng);
                if f != 0 && f != 1 && f != n {
                    break;
                }
            }
            q.push(f);
            q.push(n / f);
        }
    }
    factors.sort_unstable();
    factors
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn check_factorize() {
        assert_eq!(vec![2, 2, 3], factorize(12));
        assert_eq!(vec![3, 3, 13, 179, 271, 1381, 2423], factorize(18991325453139));
    }
}