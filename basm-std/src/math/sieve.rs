use alloc::vec;
use alloc::vec::Vec;

/// A dynamically growing linear sieve.
/// We ensure amortized O(1) runtime by growing in a similar way as `Vec<T>`
/// and by using a linear sieve algorithm.
/// The growth strategy is to increase the upper bound by 50% each time.
pub struct LinearSieve {
    upto: i64,
    smallest_prime_factor: Vec<i64>,
    primes: Vec<i64>,
    mu: Vec<i8>,
    phi: Vec<i64>,
    //d: Vec<i64>,
    //s: Vec<i64>
}

impl LinearSieve {
    /// Creates a new instance of the dynamically growing linear sieve.
    pub fn new() -> Self {
        Self {
            upto: 1,
            smallest_prime_factor: vec![0, 1],
            primes: vec![],
            mu: vec![0, 1],
            phi: vec![0, 1],
            //d: vec![0, 1],
            //s: vec![0, 1]
        }
    }
    /// Returns true if and only if `x` is a prime number.
    pub fn is_prime(&mut self, x: i64) -> bool {
        let x = Self::sanitize(x);
        x > 0 && self.smallest_prime_factor(x) == x
    }
    /// Smallest prime factor of `x` (`x=75` returns `3`).
    pub fn smallest_prime_factor(&mut self, x: i64) -> i64 {
        self.ensure_upto(x);
        self.smallest_prime_factor[x as usize]
    }
    /// `n`th prime (setting `n=1` returns the 1st prime which is `2`).
    pub fn nth_prime(&mut self, n: usize) -> i64 {
        assert!(n >= 1);
        while self.primes.len() < n {
            self.ensure_upto(self.upto + 1);
        }
        self.primes[n - 1]
    }
    /// Mobius function.
    pub fn mu(&mut self, x: i64) -> i64 {
        assert!(x >= 1);
        if self.mu.len() <= x as usize {
            let upto = Self::next_len(self.mu.len() as i64 - 1, x);
            self.ensure_upto(upto);
            self.mu.resize(upto as usize + 1, 0);
            for i in 2..=upto {
                if self.is_prime(i) {
                    self.mu[i as usize] = -1;
                }
                let lp = self.smallest_prime_factor(i);
                for &p in self.primes.iter() {
                    if i * p > upto || p > lp {
                        break;
                    }
                    self.mu[(i * p) as usize] = if lp == p {
                        0
                    } else {
                        -self.mu[i as usize]
                    };
                }
            }
        }
        self.mu[x as usize] as i64
    }
    /// Euler's totient function.
    pub fn phi(&mut self, x: i64) -> i64 {
        assert!(x >= 1);
        if self.phi.len() <= x as usize {
            let upto = Self::next_len(self.phi.len() as i64 - 1, x);
            self.ensure_upto(upto);
            self.phi.resize(upto as usize + 1, 0);
            for i in 2..=upto {
                if self.is_prime(i) {
                    self.phi[i as usize] = i - 1;
                }
                let lp = self.smallest_prime_factor(i);
                for &p in self.primes.iter() {
                    if i * p > upto || p > lp {
                        break;
                    }
                    self.phi[(i * p) as usize] = if lp == p {
                        self.phi[i as usize] * p
                    } else {
                        self.phi[i as usize] * (p - 1)
                    };
                }
            }
        }
        self.phi[x as usize]
    }
    /// (Not implemented yet) Number of positive divisors of x.
    pub fn d(&mut self, x: i64) -> i64 {
        assert!(x >= 1);
        todo!();
    }
    /// (Not implemented yet) Sum of positive divisors of x.
    pub fn s(&mut self, x: i64) -> i64 {
        assert!(x >= 1);
        todo!();
    }
    /// Returns the positive divisors of x, in ascending order.
    pub fn divisors(&mut self, mut x: i64) -> Vec<i64> {
        x = Self::sanitize(x);
        if x == 1 {
            vec![1]
        } else {
            let lp = self.smallest_prime_factor(x);
            let mut lp_cnt = 0;
            while self.smallest_prime_factor(x) == lp {
                x /= lp;
                lp_cnt += 1;
            }
            let part = self.divisors(x);
            let mut out = vec![];
            for d in part {
                let mut d_times_lp_pow = d;
                for _ in 0..=lp_cnt {
                    out.push(d_times_lp_pow);
                    d_times_lp_pow *= lp;
                }
            }
            out.sort_unstable();
            out
        }
    }
    fn ensure_upto(&mut self, x: i64) {
        if x > self.upto {
            self.upto = Self::next_len(self.upto, x);
            self.smallest_prime_factor.resize(self.upto as usize + 1, 0);
            self.primes.clear();
            for i in 2..=self.upto {
                if self.smallest_prime_factor[i as usize] == 0 ||
                   self.smallest_prime_factor[i as usize] == i {
                    self.primes.push(i);
                    self.smallest_prime_factor[i as usize] = i;
                }
                for &p in self.primes.iter() {
                    if i * p > self.upto || p > self.smallest_prime_factor[i as usize] {
                        break;
                    }
                    self.smallest_prime_factor[(i * p) as usize] = p;
                }
            }
        }
    }
    fn sanitize(x: i64) -> i64 {
        let out = if x < 0 { -x } else { x };
        assert!(x > 0);
        out
    }
    fn next_len(cur_upto: i64, x: i64) -> i64 {
        let out = cur_upto + (cur_upto >> 1) + 1;
        if x > out { x } else { out }
    }
}

impl Default for LinearSieve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_mu() {
        let mut ls = LinearSieve::new();
        assert_eq!(1, ls.mu(1));
        assert_eq!(-1, ls.mu(2));
        assert_eq!(-1, ls.mu(3));
        assert_eq!(0, ls.mu(4));
        assert_eq!(1, ls.mu(6));
        assert_eq!(1, ls.mu(41879491));
        assert_eq!(-1, ls.mu(30));
        assert_eq!(0, ls.mu(20443543));
    }

    #[test]
    fn check_phi() {
        let mut ls = LinearSieve::new();
        assert_eq!(1, ls.phi(1));
        assert_eq!(1, ls.phi(2));
        assert_eq!(2, ls.phi(3));
        assert_eq!(12, ls.phi(36));
        assert_eq!(72, ls.phi(91));
        assert_eq!(1648512, ls.phi(5986008));
        assert_eq!(40, ls.phi(100));
        assert_eq!(115200, ls.phi(442800));
    }

    #[test]
    fn check_divisors() {
        let mut ls = LinearSieve::new();
        assert_eq!(vec![1], ls.divisors(1));
        assert_eq!(vec![1, 2, 3, 4, 6, 12], ls.divisors(12));
        assert_eq!(vec![1, 2, 4, 5, 10, 20, 25, 50, 100], ls.divisors(100));
    }
}