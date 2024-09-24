// Currently, no work has been done for preventing correctness issues from overflowing and
// performance problems. This should only be considered as a sort of skeleton code.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Modulo(pub u64);

impl Modulo {
    // NOTE: Unlike `static_modint`, the arguments of each function may be larger than or equal to
    // `self.0`, the modulus.

    // TODO: Investigate the plausibility (performance, overflow) of Lemire's algorithm
    // https://snippets.kiwiyou.dev/cdiv
    // https://lemire.me/blog/2019/02/20/more-fun-with-fast-remainders-when-the-divisor-is-a-constant/

    pub fn new(modulo: u64) -> Self {
        debug_assert!(modulo >= 2);
        Self(modulo)
    }

    // TODO: Handle the case where `a + b` overflows
    pub fn add(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.0
    }

    // TODO: Handle the case where `self.0 + a` overflows
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        (self.0 + a - b) % self.0
    }

    // TODO: Handle the case where `a * b` overflows
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        (a * b) % self.0
    }

    // TODO: Try removing `i128` cast
    pub fn div(&self, a: u64, b: u64) -> u64 {
        let inv = super::egcd(b as i128, self.0 as i128).1;
        let inv = (self.0 as i128 + inv) as u64 % self.0;
        (a * inv) % self.0
    }
}
