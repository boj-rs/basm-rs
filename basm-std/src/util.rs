use core::{cmp::Ordering, f64::consts, mem, num::FpCategory};

/// Trait for f64 operations. This trait is for easy-to-use f64 functions without explicitly
/// declaring `libm` functions every single time trying to use methods with `f64`.
///
/// The function implementations are from rust standard implementations, with a few tweaks to match
/// with needs for basm-rs:
/// * https://doc.rust-lang.org/src/std/f64.rs.html
/// * https://doc.rust-lang.org/src/core/num/f64.rs
/// * https://doc.rust-lang.org/src/core/intrinsics.rs.html
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
    /// Returns the largest integer less than or equal to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.7_f64;
    /// let g = 3.0_f64;
    /// let h = -3.7_f64;
    ///
    /// assert_eq!(f.floor(), 3.0);
    /// assert_eq!(g.floor(), 3.0);
    /// assert_eq!(h.floor(), -4.0);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn floor(self) -> f64 {
        libm::floor(self)
    }

    /// Returns the smallest integer greater than or equal to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.01_f64;
    /// let g = 4.0_f64;
    ///
    /// assert_eq!(f.ceil(), 4.0);
    /// assert_eq!(g.ceil(), 4.0);
    /// ```
    #[doc(alias = "ceiling")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn ceil(self) -> f64 {
        libm::ceil(self)
    }

    /// Returns the nearest integer to `self`. If a value is half-way between two
    /// integers, round away from `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.3_f64;
    /// let g = -3.3_f64;
    /// let h = -3.7_f64;
    /// let i = 3.5_f64;
    /// let j = 4.5_f64;
    ///
    /// assert_eq!(f.round(), 3.0);
    /// assert_eq!(g.round(), -3.0);
    /// assert_eq!(h.round(), -4.0);
    /// assert_eq!(i.round(), 4.0);
    /// assert_eq!(j.round(), 5.0);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn round(self) -> f64 {
        libm::round(self)
    }

    /// Returns the nearest integer to a number. Rounds half-way cases to the number
    /// with an even least significant digit.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(round_ties_even)]
    ///
    /// let f = 3.3_f64;
    /// let g = -3.3_f64;
    /// let h = 3.5_f64;
    /// let i = 4.5_f64;
    ///
    /// assert_eq!(f.round_ties_even(), 3.0);
    /// assert_eq!(g.round_ties_even(), -3.0);
    /// assert_eq!(h.round_ties_even(), 4.0);
    /// assert_eq!(i.round_ties_even(), 4.0);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn round_ties_even(self) -> f64 {
        libm::rint(self)
    }

    /// Returns the integer part of `self`.
    /// This means that non-integer numbers are always truncated towards zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.7_f64;
    /// let g = 3.0_f64;
    /// let h = -3.7_f64;
    ///
    /// assert_eq!(f.trunc(), 3.0);
    /// assert_eq!(g.trunc(), 3.0);
    /// assert_eq!(h.trunc(), -3.0);
    /// ```
    #[doc(alias = "truncate")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn trunc(self) -> f64 {
        libm::trunc(self)
    }

    /// Returns the fractional part of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.6_f64;
    /// let y = -3.6_f64;
    /// let abs_difference_x = (x.fract() - 0.6).abs();
    /// let abs_difference_y = (y.fract() - (-0.6)).abs();
    ///
    /// assert!(abs_difference_x < 1e-10);
    /// assert!(abs_difference_y < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn fract(self) -> f64 {
        self - self.trunc()
    }

    /// Computes the absolute value of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.5_f64;
    /// let y = -3.5_f64;
    ///
    /// let abs_difference_x = (x.abs() - x).abs();
    /// let abs_difference_y = (y.abs() - (-y)).abs();
    ///
    /// assert!(abs_difference_x < 1e-10);
    /// assert!(abs_difference_y < 1e-10);
    ///
    /// assert!(f64::NAN.abs().is_nan());
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn abs(self) -> f64 {
        libm::fabs(self)
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - NaN if the number is NaN
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.5_f64;
    ///
    /// assert_eq!(f.signum(), 1.0);
    /// assert_eq!(f64::NEG_INFINITY.signum(), -1.0);
    ///
    /// assert!(f64::NAN.signum().is_nan());
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn signum(self) -> f64 {
        if self.is_nan() {
            Self::NAN
        } else {
            1.0_f64.copysign(self)
        }
    }

    /// Returns a number composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise
    /// equal to `-self`. If `self` is a NaN, then a NaN with the sign bit of
    /// `sign` is returned. Note, however, that conserving the sign bit on NaN
    /// across arithmetical operations is not generally guaranteed.
    /// See [explanation of NaN as a special value](primitive@f32) for more info.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.5_f64;
    ///
    /// assert_eq!(f.copysign(0.42), 3.5_f64);
    /// assert_eq!(f.copysign(-0.42), -3.5_f64);
    /// assert_eq!((-f).copysign(0.42), 3.5_f64);
    /// assert_eq!((-f).copysign(-0.42), -3.5_f64);
    ///
    /// assert!(f64::NAN.copysign(1.0).is_nan());
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn copysign(self, sign: f64) -> f64 {
        libm::copysign(self, sign)
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if
    /// the target architecture has a dedicated `fma` CPU instruction. However,
    /// this is not always true, and will be heavily dependant on designing
    /// algorithms with specific target hardware in mind.
    ///
    /// # Examples
    ///
    /// ```
    /// let m = 10.0_f64;
    /// let x = 4.0_f64;
    /// let b = 60.0_f64;
    ///
    /// // 100.0
    /// let abs_difference = (m.mul_add(x, b) - ((m * x) + b)).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn mul_add(self, a: f64, b: f64) -> f64 {
        libm::fma(self, a, b)
    }

    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// This computes the integer `n` such that
    /// `self = n * rhs + self.rem_euclid(rhs)`.
    /// In other words, the result is `self / rhs` rounded to the integer `n`
    /// such that `self >= n * rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a: f64 = 7.0;
    /// let b = 4.0;
    /// assert_eq!(a.div_euclid(b), 1.0); // 7.0 > 4.0 * 1.0
    /// assert_eq!((-a).div_euclid(b), -2.0); // -7.0 >= 4.0 * -2.0
    /// assert_eq!(a.div_euclid(-b), -1.0); // 7.0 >= -4.0 * -1.0
    /// assert_eq!((-a).div_euclid(-b), 2.0); // -7.0 >= -4.0 * 2.0
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn div_euclid(self, rhs: f64) -> f64 {
        let q = (self / rhs).trunc();
        if self % rhs < 0.0 {
            return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == rhs.abs()`, violating the mathematical definition, if
    /// `self` is much smaller than `rhs.abs()` in magnitude and `self < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `self == self.div_euclid(rhs) * rhs + self.rem_euclid(rhs)`
    /// approximately.
    ///
    /// # Examples
    ///
    /// ```
    /// let a: f64 = 7.0;
    /// let b = 4.0;
    /// assert_eq!(a.rem_euclid(b), 3.0);
    /// assert_eq!((-a).rem_euclid(b), 1.0);
    /// assert_eq!(a.rem_euclid(-b), 3.0);
    /// assert_eq!((-a).rem_euclid(-b), 1.0);
    /// // limitation due to round-off error
    /// assert!((-f64::EPSILON).rem_euclid(3.0) != 0.0);
    /// ```
    #[doc(alias = "modulo", alias = "mod")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn rem_euclid(self, rhs: f64) -> f64 {
        let r = self % rhs;
        if r < 0.0 {
            r + rhs.abs()
        } else {
            r
        }
    }

    /// Raises a number to an integer power.
    ///
    /// Using this function is generally faster than using `powf`.
    /// It might have a different sequence of rounding operations than `powf`,
    /// so the results are not guaranteed to agree.
    ///
    /// # Precision
    /// The rust compiler's implementation uses "rust-intrinsic" function `intrinsic::powif64`,
    /// which is not available on `libm`. In the context of competitive programming, the point
    /// where the user is using `f64` means that the precision is not a huge concern. For this
    /// reason, for now this function is equivalent to `self.powf(n as f64)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f64;
    /// let abs_difference = (x.powi(2) - (x * x)).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn powi(self, n: i32) -> f64 {
        self.powf(n as f64)
    }

    /// Raises a number to a floating point power.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f64;
    /// let abs_difference = (x.powf(2.0) - (x * x)).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn powf(self, n: f64) -> f64 {
        libm::pow(self, n)
    }

    /// Returns the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number other than `-0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let positive = 4.0_f64;
    /// let negative = -4.0_f64;
    /// let negative_zero = -0.0_f64;
    ///
    /// let abs_difference = (positive.sqrt() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// assert!(negative.sqrt().is_nan());
    /// assert!(negative_zero.sqrt() == negative_zero);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn sqrt(self) -> f64 {
        libm::sqrt(self)
    }

    /// Returns `e^(self)`, (the exponential function).
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0_f64;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn exp(self) -> f64 {
        libm::exp(self)
    }

    /// Returns `2^(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 2.0_f64;
    ///
    /// // 2^2 - 4 == 0
    /// let abs_difference = (f.exp2() - 4.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn exp2(self) -> f64 {
        libm::exp2(self)
    }

    /// Returns the natural logarithm of the number.
    ///
    /// # Note
    /// Despite not being target for basm-rs, provided log functions in Solaris/Illumos have
    /// non-standard behavior (e.g., log(-n) returns -Inf instead of NaN). The rust standard
    /// library is covering this case with a special `log_wrapper` function, but for the sake of
    /// the purpose of basm-rs, this is not implemented and log functions defined here are simply
    /// exposing APIs of libm.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0_f64;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn ln(self) -> f64 {
        libm::log(self)
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation details;
    /// `self.log2()` can produce more accurate results for base 2, and
    /// `self.log10()` can produce more accurate results for base 10.
    ///
    /// # Examples
    ///
    /// ```
    /// let twenty_five = 25.0_f64;
    ///
    /// // log5(25) - 2 == 0
    /// let abs_difference = (twenty_five.log(5.0) - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn log(self, base: f64) -> f64 {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// # Note
    /// Despite not being target for basm-rs, provided log functions in Solaris/Illumos have
    /// non-standard behavior (e.g., log(-n) returns -Inf instead of NaN). The rust standard
    /// library is covering this case with a special `log_wrapper` function, but for the sake of
    /// the purpose of basm-rs, this is not implemented and log functions defined here are simply
    /// exposing APIs of libm.
    ///
    /// # Examples
    ///
    /// ```
    /// let four = 4.0_f64;
    ///
    /// // log2(4) - 2 == 0
    /// let abs_difference = (four.log2() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn log2(self) -> f64 {
        // The original implementation of log requires `log_wrapper` due to Solaris/Illumos
        // having non-standard behavior (i.e, log(-n) returns -Inf instead of expected NaN).
        // However, this will be ignored as those are not basm-rs's targets.
        libm::log2(self)
    }

    /// Returns the base 10 logarithm of the number.
    ///
    /// # Note
    /// Despite not being target for basm-rs, provided log functions in Solaris/Illumos have
    /// non-standard behavior (e.g., log(-n) returns -Inf instead of NaN). The rust standard
    /// library is covering this case with a special `log_wrapper` function, but for the sake of
    /// the purpose of basm-rs, this is not implemented and log functions defined here are simply
    /// exposing APIs of libm.
    ///
    /// # Examples
    ///
    /// ```
    /// let hundred = 100.0_f64;
    ///
    /// // log10(100) - 2 == 0
    /// let abs_difference = (hundred.log10() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn log10(self) -> f64 {
        libm::log10(self)
    }

    /// The positive difference of two numbers.
    ///
    /// * If `self <= other`: `0.0`
    /// * Else: `self - other`
    ///
    /// # Deprecated
    /// You probably meant `(self - other).abs()`. This operation is `(self - other).max(0.0)`
    /// except that `abs_sub` also propagates NaNs (also known as `fdim` in C).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.0_f64;
    /// let y = -3.0_f64;
    ///
    /// let abs_difference_x = (x.abs_sub(1.0) - 2.0).abs();
    /// let abs_difference_y = (y.abs_sub(1.0) - 0.0).abs();
    ///
    /// assert!(abs_difference_x < 1e-10);
    /// assert!(abs_difference_y < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn abs_sub(self, other: f64) -> f64 {
        libm::fdim(self, other)
    }

    /// Returns the cube root of a number.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 8.0_f64;
    ///
    /// // x^(1/3) - 2 == 0
    /// let abs_difference = (x.cbrt() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn cbrt(self) -> f64 {
        libm::cbrt(self)
    }

    /// Compute the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, compute the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and
    /// `y.abs()`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f64;
    /// let y = 3.0_f64;
    ///
    /// // sqrt(x^2 + y^2)
    /// let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn hypot(self, other: f64) -> f64 {
        libm::hypot(self, other)
    }

    /// Computes the sine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f64::consts::FRAC_PI_2;
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn sin(self) -> f64 {
        libm::sin(self)
    }

    /// Computes the cosine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0 * std::f64::consts::PI;
    ///
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn cos(self) -> f64 {
        libm::cos(self)
    }

    /// Computes the tangent of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let abs_difference = (x.tan() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-14);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn tan(self) -> f64 {
        libm::tan(self)
    }

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f64::consts::FRAC_PI_2;
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - std::f64::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[doc(alias = "arcsin")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn asin(self) -> f64 {
        libm::asin(self)
    }

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f64::consts::FRAC_PI_4;
    ///
    /// // acos(cos(pi/4))
    /// let abs_difference = (f.cos().acos() - std::f64::consts::FRAC_PI_4).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[doc(alias = "arccos")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn acos(self) -> f64 {
        libm::acos(self)
    }

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 1.0_f64;
    ///
    /// // atan(tan(1))
    /// let abs_difference = (f.tan().atan() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[doc(alias = "arctan")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn atan(self) -> f64 {
        libm::atan(self)
    }

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// # Examples
    ///
    /// ```
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1 = 3.0_f64;
    /// let y1 = -3.0_f64;
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2 = -3.0_f64;
    /// let y2 = 3.0_f64;
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - (-std::f64::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * std::f64::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 < 1e-10);
    /// assert!(abs_difference_2 < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn atan2(self, other: f64) -> f64 {
        libm::atan2(self, other)
    }

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 < 1e-10);
    /// assert!(abs_difference_1 < 1e-10);
    /// ```
    #[doc(alias = "sincos")]
    #[inline]
    fn sin_cos(self) -> (f64, f64) {
        (self.sin(), self.cos())
    }

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1e-16_f64;
    ///
    /// // for very small x, e^x is approximately 1 + x + x^2 / 2
    /// let approx = x + x * x / 2.0;
    /// let abs_difference = (x.exp_m1() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-20);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn exp_m1(self) -> f64 {
        libm::expm1(self)
    }

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1e-16_f64;
    ///
    /// // for very small x, ln(1 + x) is approximately x - x^2 / 2
    /// let approx = x - x * x / 2.0;
    /// let abs_difference = (x.ln_1p() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-20);
    /// ```
    #[doc(alias = "log1p")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn ln_1p(self) -> f64 {
        libm::log1p(self)
    }

    /// Hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let x = 1.0_f64;
    ///
    /// let f = x.sinh();
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn sinh(self) -> f64 {
        libm::sinh(self)
    }

    /// Hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let x = 1.0_f64;
    /// let f = x.cosh();
    /// // Solving cosh() at 1 gives this result
    /// let g = ((e * e) + 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// // Same result
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn cosh(self) -> f64 {
        libm::cosh(self)
    }

    /// Hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let x = 1.0_f64;
    ///
    /// let f = x.tanh();
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn tanh(self) -> f64 {
        libm::tanh(self)
    }

    /// Inverse hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1.0_f64;
    /// let f = x.sinh().asinh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    #[doc(alias = "arcsinh")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn asinh(self) -> f64 {
        let ax = self.abs();
        let ix = 1.0 / ax;
        (ax + (ax / (Self::hypot(1.0, ix) + ix)))
            .ln_1p()
            .copysign(self)
    }

    /// Inverse hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1.0_f64;
    /// let f = x.cosh().acosh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    #[doc(alias = "arccosh")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn acosh(self) -> f64 {
        if self < 1.0 {
            Self::NAN
        } else {
            (self + ((self - 1.0).sqrt() * (self + 1.0).sqrt())).ln()
        }
    }

    /// Inverse hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let f = e.tanh().atanh();
    ///
    /// let abs_difference = (f - e).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    #[doc(alias = "arctanh")]
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn atanh(self) -> f64 {
        0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    }

    /// Gamma function.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(float_gamma)]
    /// let x = 5.0f64;
    ///
    /// let abs_difference = (x.gamma() - 24.0).abs();
    ///
    /// assert!(abs_difference <= f64::EPSILON);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn gamma(self) -> f64 {
        libm::tgamma(self)
    }

    /// Natural logarithm of the absolute value of the gamma function
    ///
    /// The integer part of the tuple indicates the sign of the gamma function.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(float_gamma)]
    /// let x = 2.0f64;
    ///
    /// let abs_difference = (x.ln_gamma().0 - 0.0).abs();
    ///
    /// assert!(abs_difference <= f64::EPSILON);
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn ln_gamma(self) -> (f64, i32) {
        libm::lgamma_r(self)
    }

    /// Not a Number (NaN).
    ///
    /// Note that IEEE 754 doesn't define just a single NaN value;
    /// a plethora of bit patterns are considered to be NaN.
    /// Furthermore, the standard makes a difference
    /// between a "signaling" and a "quiet" NaN,
    /// and allows inspecting its "payload" (the unspecified bits in the bit pattern).
    /// This constant isn't guaranteed to equal to any specific NaN bitpattern,
    /// and the stability of its representation over Rust versions
    /// and target platforms isn't guaranteed.
    const NAN: f64 = 0.0_f64 / 0.0_f64;
    /// Infinity (∞).
    const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    /// Negative infinity (−∞).
    const NEG_INFINITY: f64 = -1.0_f64 / 0.0_f64;

    /// Returns `true` if this value is NaN.
    ///
    /// ```
    /// let nan = f64::NAN;
    /// let f = 7.0_f64;
    ///
    /// assert!(nan.is_nan());
    /// assert!(!f.is_nan());
    /// ```
    #[must_use]
    #[inline]
    fn is_nan(self) -> bool {
        self != self
    }

    /// Returns `true` if this value is positive infinity or negative infinity, and
    /// `false` otherwise.
    ///
    /// ```
    /// let f = 7.0f64;
    /// let inf = f64::INFINITY;
    /// let neg_inf = f64::NEG_INFINITY;
    /// let nan = f64::NAN;
    ///
    /// assert!(!f.is_infinite());
    /// assert!(!nan.is_infinite());
    ///
    /// assert!(inf.is_infinite());
    /// assert!(neg_inf.is_infinite());
    /// ```
    #[must_use]
    #[inline]
    fn is_infinite(self) -> bool {
        // Getting clever with transmutation can result in incorrect answers on some FPUs
        // FIXME: alter the Rust <-> Rust calling convention to prevent this problem.
        // See https://github.com/rust-lang/rust/issues/72327
        (self == f64::INFINITY) | (self == f64::NEG_INFINITY)
    }

    /// Returns `true` if this number is neither infinite nor NaN.
    ///
    /// ```
    /// let f = 7.0f64;
    /// let inf: f64 = f64::INFINITY;
    /// let neg_inf: f64 = f64::NEG_INFINITY;
    /// let nan: f64 = f64::NAN;
    ///
    /// assert!(f.is_finite());
    ///
    /// assert!(!nan.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// ```
    #[must_use]
    #[inline]
    fn is_finite(self) -> bool {
        // There's no need to handle NaN separately: if self is NaN,
        // the comparison is not true, exactly as desired.
        self.abs() < Self::INFINITY
    }

    /// Returns `true` if the number is [subnormal].
    ///
    /// ```
    /// let min = f64::MIN_POSITIVE; // 2.2250738585072014e-308_f64
    /// let max = f64::MAX;
    /// let lower_than_min = 1.0e-308_f64;
    /// let zero = 0.0_f64;
    ///
    /// assert!(!min.is_subnormal());
    /// assert!(!max.is_subnormal());
    ///
    /// assert!(!zero.is_subnormal());
    /// assert!(!f64::NAN.is_subnormal());
    /// assert!(!f64::INFINITY.is_subnormal());
    /// // Values between `0` and `min` are Subnormal.
    /// assert!(lower_than_min.is_subnormal());
    /// ```
    /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
    #[must_use]
    #[inline]
    fn is_subnormal(self) -> bool {
        matches!(self.classify(), FpCategory::Subnormal)
    }

    /// Returns `true` if the number is neither zero, infinite,
    /// [subnormal], or NaN.
    ///
    /// ```
    /// let min = f64::MIN_POSITIVE; // 2.2250738585072014e-308f64
    /// let max = f64::MAX;
    /// let lower_than_min = 1.0e-308_f64;
    /// let zero = 0.0f64;
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!f64::NAN.is_normal());
    /// assert!(!f64::INFINITY.is_normal());
    /// // Values between `0` and `min` are Subnormal.
    /// assert!(!lower_than_min.is_normal());
    /// ```
    /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
    #[must_use]
    #[inline]
    fn is_normal(self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    ///
    /// ```
    /// use std::num::FpCategory;
    ///
    /// let num = 12.4_f64;
    /// let inf = f64::INFINITY;
    ///
    /// assert_eq!(num.classify(), FpCategory::Normal);
    /// assert_eq!(inf.classify(), FpCategory::Infinite);
    /// ```
    fn classify(self) -> FpCategory {
        const unsafe fn partial_classify(val: f64) -> FpCategory {
            // This doesn't actually return a right answer for NaN on purpose,
            // seeing as how it cannot correctly discern between a floating point NaN,
            // and some normal floating point numbers truncated from an x87 FPU.
            const EXP_MASK: u64 = 0x7ff0000000000000;
            const MAN_MASK: u64 = 0x000fffffffffffff;

            // SAFETY: The caller is not asking questions for which this will tell lies.
            let b = unsafe { mem::transmute::<f64, u64>(val) };
            match (b & MAN_MASK, b & EXP_MASK) {
                (0, EXP_MASK) => FpCategory::Infinite,
                (0, 0) => FpCategory::Zero,
                (_, 0) => FpCategory::Subnormal,
                _ => FpCategory::Normal,
            }
        }

        // A previous implementation tried to only use bitmask-based checks,
        // using f64::to_bits to transmute the float to its bit repr and match on that.
        // Unfortunately, floating point numbers can be much worse than that.
        // This also needs to not result in recursive evaluations of f64::to_bits.
        //
        // On some processors, in some cases, LLVM will "helpfully" lower floating point ops,
        // in spite of a request for them using f32 and f64, to things like x87 operations.
        // These have an f64's mantissa, but can have a larger than normal exponent.
        // FIXME(jubilee): Using x87 operations is never necessary in order to function
        // on x86 processors for Rust-to-Rust calls, so this issue should not happen.
        // Code generation should be adjusted to use non-C calling conventions, avoiding this.
        //
        // Thus, a value may compare unequal to infinity, despite having a "full" exponent mask.
        // And it may not be NaN, as it can simply be an "overextended" finite value.
        if self.is_nan() {
            FpCategory::Nan
        } else {
            // However, std can't simply compare to zero to check for zero, either,
            // as correctness requires avoiding equality tests that may be Subnormal == -0.0
            // because it may be wrong under "denormals are zero" and "flush to zero" modes.
            // Most of std's targets don't use those, but they are used for thumbv7neon.
            // So, this does use bitpattern matching for the rest.

            // SAFETY: f64 to u64 is fine. Usually.
            // If control flow has gotten this far, the value is definitely in one of the categories
            // that f64::partial_classify can correctly analyze.
            unsafe { partial_classify(self) }
        }
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0`, NaNs with
    /// positive sign bit and positive infinity. Note that IEEE 754 doesn't assign any
    /// meaning to the sign bit in case of a NaN, and as Rust doesn't guarantee that
    /// the bit pattern of NaNs are conserved over arithmetic operations, the result of
    /// `is_sign_positive` on a NaN might produce an unexpected result in some cases.
    /// See [explanation of NaN as a special value](f32) for more info.
    ///
    /// ```
    /// let f = 7.0_f64;
    /// let g = -7.0_f64;
    ///
    /// assert!(f.is_sign_positive());
    /// assert!(!g.is_sign_positive());
    /// ```
    #[must_use]
    #[inline]
    fn is_sign_positive(self) -> bool {
        !self.is_sign_negative()
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0`, NaNs with
    /// negative sign bit and negative infinity. Note that IEEE 754 doesn't assign any
    /// meaning to the sign bit in case of a NaN, and as Rust doesn't guarantee that
    /// the bit pattern of NaNs are conserved over arithmetic operations, the result of
    /// `is_sign_negative` on a NaN might produce an unexpected result in some cases.
    /// See [explanation of NaN as a special value](f32) for more info.
    ///
    /// ```
    /// let f = 7.0_f64;
    /// let g = -7.0_f64;
    ///
    /// assert!(!f.is_sign_negative());
    /// assert!(g.is_sign_negative());
    /// ```
    #[must_use]
    #[inline]
    fn is_sign_negative(self) -> bool {
        // IEEE754 says: isSignMinus(x) is true if and only if x has negative sign. isSignMinus
        // applies to zeros and NaNs as well.
        // SAFETY: This is just transmuting to get the sign bit, it's fine.
        unsafe { mem::transmute::<f64, u64>(self) & 0x8000_0000_0000_0000 != 0 }
    }

    /// Returns the least number greater than `self`.
    ///
    /// Let `TINY` be the smallest representable positive `f64`. Then,
    ///  - if `self.is_nan()`, this returns `self`;
    ///  - if `self` is [`NEG_INFINITY`], this returns [`MIN`];
    ///  - if `self` is `-TINY`, this returns -0.0;
    ///  - if `self` is -0.0 or +0.0, this returns `TINY`;
    ///  - if `self` is [`MAX`] or [`INFINITY`], this returns [`INFINITY`];
    ///  - otherwise the unique least value greater than `self` is returned.
    ///
    /// The identity `x.next_up() == -(-x).next_down()` holds for all non-NaN `x`. When `x`
    /// is finite `x == x.next_up().next_down()` also holds.
    ///
    /// ```rust
    /// #![feature(float_next_up_down)]
    /// // f64::EPSILON is the difference between 1.0 and the next number up.
    /// assert_eq!(1.0f64.next_up(), 1.0 + f64::EPSILON);
    /// // But not for most numbers.
    /// assert!(0.1f64.next_up() < 0.1 + f64::EPSILON);
    /// assert_eq!(9007199254740992f64.next_up(), 9007199254740994.0);
    /// ```
    ///
    /// [`NEG_INFINITY`]: Self::NEG_INFINITY
    /// [`INFINITY`]: Self::INFINITY
    /// [`MIN`]: Self::MIN
    /// [`MAX`]: Self::MAX
    fn next_up(self) -> Self {
        // We must use strictly integer arithmetic to prevent denormals from
        // flushing to zero after an arithmetic operation on some platforms.
        const TINY_BITS: u64 = 0x1; // Smallest positive f64.
        const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

        let bits = self.to_bits();
        if self.is_nan() || bits == Self::INFINITY.to_bits() {
            return self;
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            TINY_BITS
        } else if bits == abs {
            bits + 1
        } else {
            bits - 1
        };
        Self::from_bits(next_bits)
    }

    /// Returns the greatest number less than `self`.
    ///
    /// Let `TINY` be the smallest representable positive `f64`. Then,
    ///  - if `self.is_nan()`, this returns `self`;
    ///  - if `self` is [`INFINITY`], this returns [`MAX`];
    ///  - if `self` is `TINY`, this returns 0.0;
    ///  - if `self` is -0.0 or +0.0, this returns `-TINY`;
    ///  - if `self` is [`MIN`] or [`NEG_INFINITY`], this returns [`NEG_INFINITY`];
    ///  - otherwise the unique greatest value less than `self` is returned.
    ///
    /// The identity `x.next_down() == -(-x).next_up()` holds for all non-NaN `x`. When `x`
    /// is finite `x == x.next_down().next_up()` also holds.
    ///
    /// ```rust
    /// #![feature(float_next_up_down)]
    /// let x = 1.0f64;
    /// // Clamp value into range [0, 1).
    /// let clamped = x.clamp(0.0, 1.0f64.next_down());
    /// assert!(clamped < 1.0);
    /// assert_eq!(clamped.next_up(), 1.0);
    /// ```
    ///
    /// [`NEG_INFINITY`]: Self::NEG_INFINITY
    /// [`INFINITY`]: Self::INFINITY
    /// [`MIN`]: Self::MIN
    /// [`MAX`]: Self::MAX
    fn next_down(self) -> Self {
        // We must use strictly integer arithmetic to prevent denormals from
        // flushing to zero after an arithmetic operation on some platforms.
        const NEG_TINY_BITS: u64 = 0x8000_0000_0000_0001; // Smallest (in magnitude) negative f64.
        const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

        let bits = self.to_bits();
        if self.is_nan() || bits == Self::NEG_INFINITY.to_bits() {
            return self;
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            NEG_TINY_BITS
        } else if bits == abs {
            bits - 1
        } else {
            bits + 1
        };
        Self::from_bits(next_bits)
    }

    /// Takes the reciprocal (inverse) of a number, `1/x`.
    ///
    /// ```
    /// let x = 2.0_f64;
    /// let abs_difference = (x.recip() - (1.0 / x)).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    fn recip(self) -> f64 {
        1.0 / self
    }

    /// Converts radians to degrees.
    ///
    /// ```
    /// let angle = std::f64::consts::PI;
    ///
    /// let abs_difference = (angle.to_degrees() - 180.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    fn to_degrees(self) -> f64 {
        // The division here is correctly rounded with respect to the true
        // value of 180/π. (This differs from f32, where a constant must be
        // used to ensure a correctly rounded result.)
        self * (180.0f64 / consts::PI)
    }

    /// Converts degrees to radians.
    ///
    /// ```
    /// let angle = 180.0_f64;
    ///
    /// let abs_difference = (angle.to_radians() - std::f64::consts::PI).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    fn to_radians(self) -> f64 {
        let value: f64 = consts::PI;
        self * (value / 180.0)
    }

    /// Returns the maximum of the two numbers, ignoring NaN.
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    /// This follows the IEEE 754-2008 semantics for maxNum, except for handling of signaling NaNs;
    /// this function handles all NaNs the same way and avoids maxNum's problems with associativity.
    /// This also matches the behavior of libm’s fmax.
    ///
    /// ```
    /// let x = 1.0_f64;
    /// let y = 2.0_f64;
    ///
    /// assert_eq!(x.max(y), y);
    /// ```
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    #[inline]
    fn max(self, other: f64) -> f64 {
        libm::fmax(self, other)
    }

    /// Returns the minimum of the two numbers, ignoring NaN.
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    /// This follows the IEEE 754-2008 semantics for minNum, except for handling of signaling NaNs;
    /// this function handles all NaNs the same way and avoids minNum's problems with associativity.
    /// This also matches the behavior of libm’s fmin.
    ///
    /// ```
    /// let x = 1.0_f64;
    /// let y = 2.0_f64;
    ///
    /// assert_eq!(x.min(y), x);
    /// ```
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    #[inline]
    fn min(self, other: f64) -> f64 {
        libm::fmin(self, other)
    }

    /// Returns the maximum of the two numbers, propagating NaN.
    ///
    /// This returns NaN when *either* argument is NaN, as opposed to
    /// [`f64::max`] which only returns NaN when *both* arguments are NaN.
    ///
    /// ```
    /// #![feature(float_minimum_maximum)]
    /// let x = 1.0_f64;
    /// let y = 2.0_f64;
    ///
    /// assert_eq!(x.maximum(y), y);
    /// assert!(x.maximum(f64::NAN).is_nan());
    /// ```
    ///
    /// If one of the arguments is NaN, then NaN is returned. Otherwise this returns the greater
    /// of the two numbers. For this operation, -0.0 is considered to be less than +0.0.
    /// Note that this follows the semantics specified in IEEE 754-2019.
    ///
    /// Also note that "propagation" of NaNs here doesn't necessarily mean that the bitpattern of a NaN
    /// operand is conserved; see [explanation of NaN as a special value](f32) for more info.
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    #[inline]
    fn maximum(self, other: f64) -> f64 {
        if self > other {
            self
        } else if other > self {
            other
        } else if self == other {
            if self.is_sign_positive() && other.is_sign_negative() {
                self
            } else {
                other
            }
        } else {
            self + other
        }
    }

    /// Returns the minimum of the two numbers, propagating NaN.
    ///
    /// This returns NaN when *either* argument is NaN, as opposed to
    /// [`f64::min`] which only returns NaN when *both* arguments are NaN.
    ///
    /// ```
    /// #![feature(float_minimum_maximum)]
    /// let x = 1.0_f64;
    /// let y = 2.0_f64;
    ///
    /// assert_eq!(x.minimum(y), x);
    /// assert!(x.minimum(f64::NAN).is_nan());
    /// ```
    ///
    /// If one of the arguments is NaN, then NaN is returned. Otherwise this returns the lesser
    /// of the two numbers. For this operation, -0.0 is considered to be less than +0.0.
    /// Note that this follows the semantics specified in IEEE 754-2019.
    ///
    /// Also note that "propagation" of NaNs here doesn't necessarily mean that the bitpattern of a NaN
    /// operand is conserved; see [explanation of NaN as a special value](f32) for more info.
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    #[inline]
    fn minimum(self, other: f64) -> f64 {
        if self < other {
            self
        } else if other < self {
            other
        } else if self == other {
            if self.is_sign_negative() && other.is_sign_positive() {
                self
            } else {
                other
            }
        } else {
            // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
            self + other
        }
    }

    /// Calculates the middle point of `self` and `rhs`.
    ///
    /// This returns NaN when *either* argument is NaN or if a combination of
    /// +inf and -inf is provided as arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(num_midpoint)]
    /// assert_eq!(1f64.midpoint(4.0), 2.5);
    /// assert_eq!((-5.5f64).midpoint(8.0), 1.25);
    /// ```
    fn midpoint(self, other: f64) -> f64 {
        const LO: f64 = f64::MIN_POSITIVE * 2.;
        const HI: f64 = f64::MAX / 2.;

        let (a, b) = (self, other);
        let abs_a = a.abs();
        let abs_b = b.abs();

        if abs_a <= HI && abs_b <= HI {
            // Overflow is impossible
            (a + b) / 2.
        } else if abs_a < LO {
            // Not safe to halve a
            a + (b / 2.)
        } else if abs_b < LO {
            // Not safe to halve b
            (a / 2.) + b
        } else {
            // Not safe to halve a and b
            (a / 2.) + (b / 2.)
        }
    }

    /// Raw transmutation to `u64`.
    ///
    /// This is currently identical to `transmute::<f64, u64>(self)` on all platforms.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// Note that this function is distinct from `as` casting, which attempts to
    /// preserve the *numeric* value, and not the bitwise value.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!((1f64).to_bits() != 1f64 as u64); // to_bits() is not casting!
    /// assert_eq!((12.5f64).to_bits(), 0x4029000000000000);
    ///
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    fn to_bits(self) -> u64 {
        // SAFETY: `u64` is a plain old datatype so we can always transmute to it.
        // ...sorta.
        //
        // See the SAFETY comment in f64::from_bits for more.
        unsafe { mem::transmute::<f64, u64>(self) }
    }

    /// Raw transmutation from `u64`.
    ///
    /// This is currently identical to `transmute::<u64, f64>(v)` on all platforms.
    /// It turns out this is incredibly portable, for two reasons:
    ///
    /// * Floats and Ints have the same endianness on all supported platforms.
    /// * IEEE 754 very precisely specifies the bit layout of floats.
    ///
    /// However there is one caveat: prior to the 2008 version of IEEE 754, how
    /// to interpret the NaN signaling bit wasn't actually specified. Most platforms
    /// (notably x86 and ARM) picked the interpretation that was ultimately
    /// standardized in 2008, but some didn't (notably MIPS). As a result, all
    /// signaling NaNs on MIPS are quiet NaNs on x86, and vice-versa.
    ///
    /// Rather than trying to preserve signaling-ness cross-platform, this
    /// implementation favors preserving the exact bits. This means that
    /// any payloads encoded in NaNs will be preserved even if the result of
    /// this method is sent over the network from an x86 machine to a MIPS one.
    ///
    /// If the results of this method are only manipulated by the same
    /// architecture that produced them, then there is no portability concern.
    ///
    /// If the input isn't NaN, then there is no portability concern.
    ///
    /// If you don't care about signaling-ness (very likely), then there is no
    /// portability concern.
    ///
    /// Note that this function is distinct from `as` casting, which attempts to
    /// preserve the *numeric* value, and not the bitwise value.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = f64::from_bits(0x4029000000000000);
    /// assert_eq!(v, 12.5);
    /// ```
    #[must_use]
    #[inline]
    fn from_bits(v: u64) -> Self {
        // It turns out the safety issues with sNaN were overblown! Hooray!
        // SAFETY: `u64` is a plain old datatype so we can always transmute from it
        // ...sorta.
        //
        // It turns out that at runtime, it is possible for a floating point number
        // to be subject to floating point modes that alter nonzero subnormal numbers
        // to zero on reads and writes, aka "denormals are zero" and "flush to zero".
        // This is not a problem usually, but at least one tier2 platform for Rust
        // actually exhibits an FTZ behavior by default: thumbv7neon
        // aka "the Neon FPU in AArch32 state"
        //
        // Even with this, not all instructions exhibit the FTZ behaviors on thumbv7neon,
        // so this should load the same bits if LLVM emits the "correct" instructions,
        // but LLVM sometimes makes interesting choices about float optimization,
        // and other FPUs may do similar. Thus, it is wise to indulge luxuriously in caution.
        //
        // In addition, on x86 targets with SSE or SSE2 disabled and the x87 FPU enabled,
        // i.e. not soft-float, the way Rust does parameter passing can actually alter
        // a number that is "not infinity" to have the same exponent as infinity,
        // in a slightly unpredictable manner.
        //
        // And, of course evaluating to a NaN value is fairly nondeterministic.
        // More precisely: when NaN should be returned is knowable, but which NaN?
        // So far that's defined by a combination of LLVM and the CPU, not Rust.
        // This function, however, allows observing the bitstring of a NaN,
        // thus introspection on CTFE.
        //
        // In order to preserve, at least for the moment, const-to-runtime equivalence,
        // reject any of these possible situations from happening.
        //
        // SAFETY: `u64` is a plain old datatype so we can always... uh...
        // ...look, just pretend you forgot what you just read.
        // Stability concerns.
        unsafe { mem::transmute::<u64, f64>(v) }
    }

    /// Return the memory representation of this floating point number as a byte array in
    /// big-endian (network) byte order.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// # Examples
    ///
    /// ```
    /// let bytes = 12.5f64.to_be_bytes();
    /// assert_eq!(bytes, [0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    fn to_be_bytes(self) -> [u8; 8] {
        self.to_bits().to_be_bytes()
    }

    /// Return the memory representation of this floating point number as a byte array in
    /// little-endian byte order.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// # Examples
    ///
    /// ```
    /// let bytes = 12.5f64.to_le_bytes();
    /// assert_eq!(bytes, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40]);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    fn to_le_bytes(self) -> [u8; 8] {
        self.to_bits().to_le_bytes()
    }

    /// Return the memory representation of this floating point number as a byte array in
    /// native byte order.
    ///
    /// As the target platform's native endianness is used, portable code
    /// should use [`to_be_bytes`] or [`to_le_bytes`], as appropriate, instead.
    ///
    /// [`to_be_bytes`]: f64::to_be_bytes
    /// [`to_le_bytes`]: f64::to_le_bytes
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// # Examples
    ///
    /// ```
    /// let bytes = 12.5f64.to_ne_bytes();
    /// assert_eq!(
    ///     bytes,
    ///     if cfg!(target_endian = "big") {
    ///         [0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    ///     } else {
    ///         [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40]
    ///     }
    /// );
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    fn to_ne_bytes(self) -> [u8; 8] {
        self.to_bits().to_ne_bytes()
    }

    /// Create a floating point value from its representation as a byte array in big endian.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// # Examples
    ///
    /// ```
    /// let value = f64::from_be_bytes([0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// assert_eq!(value, 12.5);
    /// ```
    #[must_use]
    #[inline]
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_be_bytes(bytes))
    }

    /// Create a floating point value from its representation as a byte array in little endian.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// # Examples
    ///
    /// ```
    /// let value = f64::from_le_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40]);
    /// assert_eq!(value, 12.5);
    /// ```
    #[must_use]
    #[inline]
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_le_bytes(bytes))
    }

    /// Create a floating point value from its representation as a byte array in native endian.
    ///
    /// As the target platform's native endianness is used, portable code
    /// likely wants to use [`from_be_bytes`] or [`from_le_bytes`], as
    /// appropriate instead.
    ///
    /// [`from_be_bytes`]: f64::from_be_bytes
    /// [`from_le_bytes`]: f64::from_le_bytes
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// # Examples
    ///
    /// ```
    /// let value = f64::from_ne_bytes(if cfg!(target_endian = "big") {
    ///     [0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    /// } else {
    ///     [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40]
    /// });
    /// assert_eq!(value, 12.5);
    /// ```
    #[must_use]
    #[inline]
    fn from_ne_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_ne_bytes(bytes))
    }

    /// Return the ordering between `self` and `other`.
    ///
    /// Unlike the standard partial comparison between floating point numbers,
    /// this comparison always produces an ordering in accordance to
    /// the `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
    /// floating point standard. The values are ordered in the following sequence:
    ///
    /// - negative quiet NaN
    /// - negative signaling NaN
    /// - negative infinity
    /// - negative numbers
    /// - negative subnormal numbers
    /// - negative zero
    /// - positive zero
    /// - positive subnormal numbers
    /// - positive numbers
    /// - positive infinity
    /// - positive signaling NaN
    /// - positive quiet NaN.
    ///
    /// The ordering established by this function does not always agree with the
    /// [`PartialOrd`] and [`PartialEq`] implementations of `f64`. For example,
    /// they consider negative and positive zero equal, while `total_cmp`
    /// doesn't.
    ///
    /// The interpretation of the signaling NaN bit follows the definition in
    /// the IEEE 754 standard, which may not match the interpretation by some of
    /// the older, non-conformant (e.g. MIPS) hardware implementations.
    ///
    /// # Example
    ///
    /// ```
    /// struct GoodBoy {
    ///     name: String,
    ///     weight: f64,
    /// }
    ///
    /// let mut bois = vec![
    ///     GoodBoy { name: "Pucci".to_owned(), weight: 0.1 },
    ///     GoodBoy { name: "Woofer".to_owned(), weight: 99.0 },
    ///     GoodBoy { name: "Yapper".to_owned(), weight: 10.0 },
    ///     GoodBoy { name: "Chonk".to_owned(), weight: f64::INFINITY },
    ///     GoodBoy { name: "Abs. Unit".to_owned(), weight: f64::NAN },
    ///     GoodBoy { name: "Floaty".to_owned(), weight: -5.0 },
    /// ];
    ///
    /// bois.sort_by(|a, b| a.weight.total_cmp(&b.weight));
    /// # assert!(bois.into_iter().map(|b| b.weight)
    /// #     .zip([-5.0, 0.1, 10.0, 99.0, f64::INFINITY, f64::NAN].iter())
    /// #     .all(|(a, b)| a.to_bits() == b.to_bits()))
    /// ```
    #[must_use]
    #[inline]
    fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {
        let mut left = self.to_bits() as i64;
        let mut right = other.to_bits() as i64;

        // In case of negatives, flip all the bits except the sign
        // to achieve a similar layout as two's complement integers
        //
        // Why does this work? IEEE 754 floats consist of three fields:
        // Sign bit, exponent and mantissa. The set of exponent and mantissa
        // fields as a whole have the property that their bitwise order is
        // equal to the numeric magnitude where the magnitude is defined.
        // The magnitude is not normally defined on NaN values, but
        // IEEE 754 totalOrder defines the NaN values also to follow the
        // bitwise order. This leads to order explained in the doc comment.
        // However, the representation of magnitude is the same for negative
        // and positive numbers – only the sign bit is different.
        // To easily compare the floats as signed integers, we need to
        // flip the exponent and mantissa bits in case of negative numbers.
        // We effectively convert the numbers to "two's complement" form.
        //
        // To do the flipping, we construct a mask and XOR against it.
        // We branchlessly calculate an "all-ones except for the sign bit"
        // mask from negative-signed values: right shifting sign-extends
        // the integer, so we "fill" the mask with sign bits, and then
        // convert to unsigned to push one more zero bit.
        // On positive values, the mask is all zeros, so it's a no-op.
        left ^= (((left >> 63) as u64) >> 1) as i64;
        right ^= (((right >> 63) as u64) >> 1) as i64;

        left.cmp(&right)
    }

    /// Restrict a value to a certain interval unless it is NaN.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise this returns `self`.
    ///
    /// Note that this function returns NaN if the initial value was NaN as
    /// well.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`, `min` is NaN, or `max` is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!((-3.0f64).clamp(-2.0, 1.0) == -2.0);
    /// assert!((0.0f64).clamp(-2.0, 1.0) == 0.0);
    /// assert!((2.0f64).clamp(-2.0, 1.0) == 1.0);
    /// assert!((f64::NAN).clamp(-2.0, 1.0).is_nan());
    /// ```
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn clamp(mut self, min: f64, max: f64) -> f64 {
        assert!(
            min <= max,
            "min > max, or either was NaN. min = {min:?}, max = {max:?}"
        );
        if self < min {
            self = min;
        }
        if self > max {
            self = max;
        }
        self
    }
}
