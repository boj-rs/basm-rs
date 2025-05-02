pub mod f64;
pub use f64::*;

pub trait ToI128 {
    fn to_i128(self) -> i128;
}

impl ToI128 for i128 {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}
impl ToI128 for usize {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}
impl ToI128 for f32 {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}
impl ToI128 for f64 {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}
impl ToI128 for i32 {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}
impl ToI128 for i64 {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}
impl ToI128 for i16 {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}
impl ToI128 for i8 {
    fn to_i128(self) -> i128 {
        return self as i128;
    }
}