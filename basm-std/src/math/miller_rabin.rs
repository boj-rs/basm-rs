// This is a Rust port of the implementation originally written in C++ by wider93.

// Seperate type to distinguish montgomery forms and normal integers
#[derive(Clone, Copy)]
pub struct M<T> {
    pub v: T,
}

pub struct OddMont<T> {
    n: T,
    ni: T,
    r2: T,
    r1: T,
}

macro_rules! impl_mont {
    ($ty: ident, $ty_large: ident) => {
        impl OddMont<$ty> {
            pub fn new(a: $ty) -> OddMont<$ty> {
                let n = a;
                let ni = (0 as $ty).wrapping_sub(Self::inv_word(a));
                let r2 = (0 as $ty_large).wrapping_sub(a as $ty_large) % a as $ty_large;
                Self {
                    n: a,
                    ni,
                    r2: r2 as $ty,
                    r1: Self::redc_given(r2, n, ni)
                }
            }

            // inverse of 1 << wordsize. a must be odd.
            const fn inv_word(a: $ty) -> $ty {
                let mut ans: $ty = a;
                let mut i = 0;
                while i < 6 {
                    ans = ans.wrapping_mul((2 as $ty).wrapping_sub(ans.wrapping_mul(a)));
                    i += 1;
                }
                ans
            }

            fn redc_given(a: $ty_large, n: $ty, ni: $ty) -> $ty {
                let m = (a as $ty).wrapping_mul(ni);
                let mn_neg = ((0 as $ty).wrapping_sub(m) as $ty_large).wrapping_mul(n as $ty_large);
                if a >= mn_neg {
                    ((a - mn_neg) >> $ty::BITS) as $ty
                } else {
                    ((a + (m as $ty_large * n as $ty_large)) >> $ty::BITS) as $ty
                }
            }

            pub fn redc(&self, a: $ty_large) -> M<$ty> {
                M { v: Self::redc_given(a, self.n, self.ni) }
            }
        
            pub fn mul(&self, a: M<$ty>, b: M<$ty>) -> M<$ty> {
                self.redc(a.v as $ty_large * b.v as $ty_large)
            }
        
            pub fn to_mont(&self, a: $ty) -> M<$ty> {
                self.redc(self.r2 as $ty_large * a as $ty_large)
            }

            #[allow(dead_code)]
            fn powmul(&self, base: M<$ty>, exp: $ty, v: $ty) -> $ty {
                let mut ans = M { v };
                let mut base = base;
                let mut exp = exp;
                while exp > 0 {
                    if exp & 1 != 0 {
                        ans = self.mul(ans, base);
                    }
                    base = self.mul(base, base);
                    exp >>= 1;
                }
                ans.v
            }
        }
    }
}
impl_mont!(u32, u64);
impl_mont!(u64, u128);

mod details {
    use super::*;
    pub const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    pub const THRESHOLD_ARRAY: [(u64, usize); 5] = [
        (3825123056546413051, 9),
        (341550071728321, 7),
        (3474749660383, 6),
        (2152302898747, 5),
        (3215031751, 4),
    ];

    pub fn thres(k: u64) -> usize {
        let mut t = 12;
        for &(i, j) in &THRESHOLD_ARRAY {
            if k < i {
                t = j;
            } else {
                break;
            }
        }
        t
    }

    pub fn miller_rabin_base(k: &OddMont<u64>, s: i32, odd: u64, base: u64) -> bool {
        let mut x = M { v: k.powmul(k.to_mont(base), odd, k.r1) };
        for _ in 0..s {
            if x.v == k.r1 { break; }
            let y = k.mul(x, x);
            if y.v == k.r1 && x.v != k.n - k.r1 {
                return false;
            }
            x = y;
        }
        x.v == k.r1
    }
    
    pub fn miller_rabin(k: u64) -> bool {
        if k % 2 == 0 {
            return k == 2;
        }
        let mut s = 0;
        let mut odd = k - 1;
        while odd & 1 == 0 {
            odd >>= 1;
            s += 1;
        }
        let modd = OddMont::<u64>::new(k);
        for &bases_i in BASES.iter().take(thres(k)) {
            if bases_i == k {
                return true;
            }
            if !miller_rabin_base(&modd, s, odd, bases_i) {
                return false;
            }
        }
        true
    }
}

pub fn is_prime_u32(x: u32) -> bool {
    is_prime_u64(x as u64)
}

pub fn is_prime_u64(x: u64) -> bool {
    if x < 1000 {
        if x < 2 { return false; }
        if x % 2 == 0 { return x == 2; }
        let mut q = 3;
        while q * q <= x {
            if x % q == 0 { return false; }
            q += 2;
        }
        true
    } else {
        details::miller_rabin(x)
    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn check_is_prime_u32() {
        assert_eq!(true, is_prime_u32(101));
        assert_eq!(false, is_prime_u32(906810173));
        assert_eq!(false, is_prime_u32(598963177));
    }

    #[test]
    fn check_is_prime_u64() {
        assert_eq!(true, is_prime_u64(101));
        assert_eq!(false, is_prime_u64(906810173));
        assert_eq!(false, is_prime_u64(598963177));
        assert_eq!(false, is_prime_u64(162319020967));
    }
}