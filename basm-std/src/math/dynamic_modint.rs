// Currently, no work has been done for preventing correctness issues from overflowing and
// performance problems. This should only be considered as a sort of skeleton code.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Modulo(pub u64);

impl Modulo {
    pub fn new(modulo: u64) -> Self {
        debug_assert!(modulo >= 2);
        Self(modulo)
    }

    pub fn add(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.0
    }

    pub fn sub(&self, a: u64, b: u64) -> u64 {
        (self.0 + a - b) % self.0
    }

    pub fn mul(&self, a: u64, b: u64) -> u64 {
        (a * b) % self.0
    }

    pub fn div(&self, a: u64, b: u64) -> u64 {
        let inv = super::egcd(b as i128, self.0 as i128).1;
        let inv = (self.0 as i128 + inv) as u64 % self.0;
        (a * inv) % self.0
    }
}
