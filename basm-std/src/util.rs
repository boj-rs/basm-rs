use core::{cmp::Ordering, num::FpCategory};
// use core::convert::FloatToInt;

pub trait F64Ops {
    fn floor(self) -> f64;
    fn ceil(self) -> f64;
    fn round(self) -> f64;
    fn round_ties_even(self) -> f64;
    fn trunc(self) -> f64;
    fn fract(self) -> f64;
    fn abs(self) -> f64;
    fn signum(self) -> f64;
    fn copysign(self, sign: f64) -> f64;
    fn mul_add(self, a: f64, b: f64) -> f64;
    fn div_euclid(self, rhs: f64) -> f64;
    fn rem_euclid(self, rhs: f64) -> f64;
    fn powi(self, n: i32) -> f64;
    fn powf(self, n: f64) -> f64;
    fn sqrt(self) -> f64;
    fn exp(self) -> f64;
    fn exp2(self) -> f64;
    fn ln(self) -> f64;
    fn log(self, base: f64) -> f64;
    fn log2(self) -> f64;
    fn log10(self) -> f64;
    fn abs_sub(self, other: f64) -> f64;
    fn cbrt(self) -> f64;
    fn hypot(self, other: f64) -> f64;
    fn sin(self) -> f64;
    fn cos(self) -> f64;
    fn tan(self) -> f64;
    fn asin(self) -> f64;
    fn acos(self) -> f64;
    fn atan(self) -> f64;
    fn atan2(self, other: f64) -> f64;
    fn sin_cos(self) -> (f64, f64);
    fn exp_m1(self) -> f64;
    fn ln_1p(self) -> f64;
    fn sinh(self) -> f64;
    fn cosh(self) -> f64;
    fn tanh(self) -> f64;
    fn asinh(self) -> f64;
    fn acosh(self) -> f64;
    fn atanh(self) -> f64;
    fn gamma(self) -> f64;
    fn ln_gamma(self) -> (f64, i32);
    const RADIX: u32 = 2u32;
    const MANTISSA_DIGITS: u32 = 53u32;
    const DIGITS: u32 = 15u32;
    const EPSILON: f64 = 2.2204460492503131E-16f64;
    const MIN: f64 = -1.7976931348623157E+308f64;
    const MIN_POSITIVE: f64 = 2.2250738585072014E-308f64;
    const MAX: f64 = 1.7976931348623157E+308f64;
    const MIN_EXP: i32 = -1_021i32;
    const MAX_EXP: i32 = 1_024i32;
    const MIN_10_EXP: i32 = -307i32;
    const MAX_10_EXP: i32 = 308i32;
    const NAN: f64;
    const INFINITY: f64;
    const NEG_INFINITY: f64;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_subnormal(self) -> bool;
    fn is_normal(self) -> bool;
    fn classify(self) -> FpCategory;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn next_up(self) -> f64;
    fn next_down(self) -> f64;
    fn recip(self) -> f64;
    fn to_degrees(self) -> f64;
    fn to_radians(self) -> f64;
    fn max(self, other: f64) -> f64;
    fn min(self, other: f64) -> f64;
    fn maximum(self, other: f64) -> f64;
    fn minimum(self, other: f64) -> f64;
    fn midpoint(self, other: f64) -> f64;
    fn to_bits(self) -> u64;
    fn from_bits(v: u64) -> f64;
    fn to_be_bytes(self) -> [u8; 8];
    fn to_le_bytes(self) -> [u8; 8];
    fn to_ne_bytes(self) -> [u8; 8];
    fn from_be_bytes(bytes: [u8; 8]) -> f64;
    fn from_le_bytes(bytes: [u8; 8]) -> f64;
    fn from_ne_bytes(bytes: [u8; 8]) -> f64;
    fn total_cmp(&self, other: &f64) -> Ordering;
    fn clamp(self, min: f64, max: f64) -> f64;
}

impl F64Ops for f64 {
    fn floor(self) -> f64 {
        todo!()
    }

    fn ceil(self) -> f64 {
        todo!()
    }

    fn round(self) -> f64 {
        todo!()
    }

    fn round_ties_even(self) -> f64 {
        todo!()
    }

    fn trunc(self) -> f64 {
        todo!()
    }

    fn fract(self) -> f64 {
        todo!()
    }

    fn abs(self) -> f64 {
        todo!()
    }

    fn signum(self) -> f64 {
        todo!()
    }

    fn copysign(self, sign: f64) -> f64 {
        todo!()
    }

    fn mul_add(self, a: f64, b: f64) -> f64 {
        todo!()
    }

    fn div_euclid(self, rhs: f64) -> f64 {
        todo!()
    }

    fn rem_euclid(self, rhs: f64) -> f64 {
        todo!()
    }

    fn powi(self, n: i32) -> f64 {
        todo!()
    }

    fn powf(self, n: f64) -> f64 {
        todo!()
    }

    fn sqrt(self) -> f64 {
        todo!()
    }

    fn exp(self) -> f64 {
        todo!()
    }

    fn exp2(self) -> f64 {
        todo!()
    }

    fn ln(self) -> f64 {
        todo!()
    }

    fn log(self, base: f64) -> f64 {
        todo!()
    }

    fn log2(self) -> f64 {
        todo!()
    }

    fn log10(self) -> f64 {
        todo!()
    }

    fn abs_sub(self, other: f64) -> f64 {
        todo!()
    }

    fn cbrt(self) -> f64 {
        todo!()
    }

    fn hypot(self, other: f64) -> f64 {
        todo!()
    }

    fn sin(self) -> f64 {
        todo!()
    }

    fn cos(self) -> f64 {
        todo!()
    }

    fn tan(self) -> f64 {
        todo!()
    }

    fn asin(self) -> f64 {
        todo!()
    }

    fn acos(self) -> f64 {
        todo!()
    }

    fn atan(self) -> f64 {
        todo!()
    }

    fn atan2(self, other: f64) -> f64 {
        todo!()
    }

    fn sin_cos(self) -> (f64, f64) {
        todo!()
    }

    fn exp_m1(self) -> f64 {
        todo!()
    }

    fn ln_1p(self) -> f64 {
        todo!()
    }

    fn sinh(self) -> f64 {
        todo!()
    }

    fn cosh(self) -> f64 {
        todo!()
    }

    fn tanh(self) -> f64 {
        todo!()
    }

    fn asinh(self) -> f64 {
        todo!()
    }

    fn acosh(self) -> f64 {
        todo!()
    }

    fn atanh(self) -> f64 {
        todo!()
    }

    fn gamma(self) -> f64 {
        todo!()
    }

    fn ln_gamma(self) -> (f64, i32) {
        todo!()
    }

    const NAN: f64;

    const INFINITY: f64;

    const NEG_INFINITY: f64;

    fn is_nan(self) -> bool {
        todo!()
    }

    fn is_infinite(self) -> bool {
        todo!()
    }

    fn is_finite(self) -> bool {
        todo!()
    }

    fn is_subnormal(self) -> bool {
        todo!()
    }

    fn is_normal(self) -> bool {
        todo!()
    }

    fn classify(self) -> FpCategory {
        todo!()
    }

    fn is_sign_positive(self) -> bool {
        todo!()
    }

    fn is_sign_negative(self) -> bool {
        todo!()
    }

    fn next_up(self) -> f64 {
        todo!()
    }

    fn next_down(self) -> f64 {
        todo!()
    }

    fn recip(self) -> f64 {
        todo!()
    }

    fn to_degrees(self) -> f64 {
        todo!()
    }

    fn to_radians(self) -> f64 {
        todo!()
    }

    fn max(self, other: f64) -> f64 {
        todo!()
    }

    fn min(self, other: f64) -> f64 {
        todo!()
    }

    fn maximum(self, other: f64) -> f64 {
        todo!()
    }

    fn minimum(self, other: f64) -> f64 {
        todo!()
    }

    fn midpoint(self, other: f64) -> f64 {
        todo!()
    }

    fn to_bits(self) -> u64 {
        todo!()
    }

    fn from_bits(v: u64) -> f64 {
        todo!()
    }

    fn to_be_bytes(self) -> [u8; 8] {
        todo!()
    }

    fn to_le_bytes(self) -> [u8; 8] {
        todo!()
    }

    fn to_ne_bytes(self) -> [u8; 8] {
        todo!()
    }

    fn from_be_bytes(bytes: [u8; 8]) -> f64 {
        todo!()
    }

    fn from_le_bytes(bytes: [u8; 8]) -> f64 {
        todo!()
    }

    fn from_ne_bytes(bytes: [u8; 8]) -> f64 {
        todo!()
    }

    fn total_cmp(&self, other: &f64) -> Ordering {
        todo!()
    }

    fn clamp(self, min: f64, max: f64) -> f64 {
        todo!()
    }
}
