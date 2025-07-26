/// Provides faster modular operations when modulo is not known at compile time.
pub struct FastModOps {
    modulo: u64,
    pow2: bool,
    data: [u64; 2],
}

impl FastModOps {
    /// Initializes the FastModOps object by precomputing magic numbers.
    pub fn new(modulo: u64) -> Self {
        if modulo & modulo.wrapping_sub(1) == 0 {
            // modulo is zero or power of 2; we save the mask in data[0].
            Self {
                modulo,
                pow2: true,
                data: [modulo.wrapping_sub(1), 0],
            }
        } else if modulo < 1u64 << 63 {
            let k = 64 - modulo.leading_zeros(); // k <= 63
            let v = ((4u128.pow(k)) / modulo as u128) as u64; // v < 2^64
            Self {
                modulo,
                pow2: false,
                data: [v, 2 * k as u64],
            }
        } else {
            let v = (u128::MAX / modulo as u128 - (1u128 << 64)) as u64;
            Self {
                modulo,
                pow2: false,
                data: [v, 0],
            }
        }
    }
    /// Canonicalizes given integer `a` into the range `[0, self.modulo)`.
    pub fn canonicalize(&self, a: u64) -> u64 {
        if self.pow2 {
            a & self.data[0]
        } else {
            a % self.modulo
        }
    }
    /// Computes `-a` mod `self.modulo`.
    pub fn neg(&self, a: u64) -> u64 {
        debug_assert!(a <= self.modulo.wrapping_sub(1));
        if self.pow2 {
            0u64.wrapping_sub(a) & self.data[0]
        } else if a == 0 {
            0
        } else {
            self.modulo.wrapping_sub(a)
        }
    }
    /// Computes `a*b` mod `self.modulo`.
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a <= self.modulo.wrapping_sub(1));
        debug_assert!(b <= self.modulo.wrapping_sub(1));
        if self.pow2 {
            a.wrapping_mul(b) & self.data[0]
        } else {
            let u = a as u128 * b as u128;
            let (d, [v, k]) = (self.modulo, self.data);
            if self.modulo < 1u64 << 42 {
                // Barrett reduction (optimized variant for small modulo)
                let q = (u * v as u128) >> k;
                let r = (u - q * d as u128) as u64;
                if r >= d { r - d } else { r }
            } else if self.modulo < 1u64 << 63 {
                // Barrett reduction
                let q = {
                    let (hi, lo) = (u >> 64, u as u64 as u128);
                    let hi_v = hi * v as u128;
                    let lo_v = (lo * v as u128) >> k;
                    (if k >= 64 {
                        hi_v >> (k - 64)
                    } else {
                        hi_v << (64 - k)
                    }) + lo_v
                };
                let r = (u - q * d as u128) as u64;
                if r >= d { r - d } else { r }
            } else {
                // See div2by1 on p. 4 in https://gmplib.org/~tege/division-paper.pdf
                let q = v as u128 * (u >> 64) + u;
                let q1 = (q >> 64) as u64 + 1;
                let q0 = q as u64;
                let mut r = (u as u64).wrapping_sub(q1.wrapping_mul(d));
                if r > q0 {
                    r = r.wrapping_add(d);
                }
                if r >= d { r - d } else { r }
            }
        }
    }
    /// Computes `a+b` mod `self.modulo`.
    pub fn add(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a <= self.modulo.wrapping_sub(1));
        debug_assert!(b <= self.modulo.wrapping_sub(1));
        if self.pow2 {
            a.wrapping_add(b) & self.data[0]
        } else {
            let neg_b = self.modulo.wrapping_sub(b);
            let (out, overflow) = a.overflowing_sub(neg_b);
            if overflow {
                out.wrapping_add(self.modulo)
            } else {
                out
            }
        }
    }
    /// Computes `a-b` mod `self.modulo`.
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a <= self.modulo.wrapping_sub(1));
        debug_assert!(b <= self.modulo.wrapping_sub(1));
        if self.pow2 {
            a.wrapping_sub(b) & self.data[0]
        } else {
            let (out, overflow) = a.overflowing_sub(b);
            if overflow {
                out.wrapping_add(self.modulo)
            } else {
                out
            }
        }
    }
    /// Computes `(a*b) + c` mod `self.modulo` (fused-multiply-add; fmadd).
    ///
    /// TODO: optimize further by exploiting fusion.
    pub fn fmadd(&self, a: u64, b: u64, c: u64) -> u64 {
        self.add(self.mul(a, b), c)
    }
    /// Computes `(a*b) - c` mod `self.modulo` (fused-multiply-sub; fmsub).
    ///
    /// TODO: optimize further by exploiting fusion.
    pub fn fmsub(&self, a: u64, b: u64, c: u64) -> u64 {
        self.sub(self.mul(a, b), c)
    }
    /// Computes `-(a*b) + c` mod `self.modulo` (fused-negated-multiply-add; fnmadd).
    ///
    /// TODO: optimize further by exploiting fusion.
    pub fn fnmadd(&self, a: u64, b: u64, c: u64) -> u64 {
        self.sub(c, self.mul(a, b))
    }
    /// Computes `-(a*b) - c` mod `self.modulo` (fused-negated-multiply-sub; fnmsub).
    ///
    /// TODO: optimize further by exploiting fusion.
    pub fn fnmsub(&self, a: u64, b: u64, c: u64) -> u64 {
        self.sub(self.neg(c), self.mul(a, b))
    }
    /// Precomputes a fast multiplication routine.
    ///
    /// TODO: optimize by actually precomputing useful values and tailoring the computation steps.
    pub fn premul(&self, a: u64) -> impl Fn(u64) -> u64 {
        move |x: u64| -> u64 { self.mul(x, a) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::*;

    #[test]
    fn check_fastmodops() {
        let it = (1..100)
            .chain(1_000_000_000..1_000_000_100)
            .chain((1u64 << 63) - 20..(1u64 << 63) + 20)
            .chain(u64::MAX - 20..=u64::MAX)
            .chain((0..=64).map(|k| if k == 64 { 0 } else { 1u64 << k }));
        for modulo in it.clone() {
            let ops = FastModOps::new(modulo);
            let near_modulo = (0u64.wrapping_sub(5)..=5u64).map(|x| modulo.wrapping_add(x));
            for x in it.clone().chain(near_modulo.clone()) {
                assert_eq!(
                    modsub(0, x, modulo),
                    ops.neg(ops.canonicalize(x)),
                    "{x} {modulo}"
                );
                for y in it.clone().chain(near_modulo.clone()) {
                    assert_eq!(
                        modadd(x, y, modulo),
                        ops.add(ops.canonicalize(x), ops.canonicalize(y)),
                        "{x} {y} {modulo}"
                    );
                    assert_eq!(
                        modsub(x, y, modulo),
                        ops.sub(ops.canonicalize(x), ops.canonicalize(y)),
                        "{x} {y} {modulo}"
                    );
                    assert_eq!(
                        modmul(x, y, modulo),
                        ops.mul(ops.canonicalize(x), ops.canonicalize(y)),
                        "{x} {y} {modulo}"
                    );
                }
            }
        }
    }
}
