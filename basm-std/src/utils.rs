pub mod f64;
pub use f64::*;

pub trait ToF64 {
    fn to_f64(self) -> f64;
}

impl ToF64 for f64 {
    fn to_f64(self) -> f64 {
        self
    }
}

impl ToF64 for i128 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToF64 for i64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToF64 for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToF64 for u64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToF64 for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_f64_from_f64() {
        let x: f64 = 3.1415;
        assert!((x.to_f64() - 3.1415).abs() < 1e-10);
    }

    #[test]
    fn test_to_f64_from_i128() {
        let x: i128 = 42;
        assert!((x.to_f64() - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_to_f64_large_i128() {
        let x: i128 = 1_000_000_000_000_000_000;
        let y = x.to_f64();
        assert!((y - 1e18).abs() < 1e3);
    }
}
