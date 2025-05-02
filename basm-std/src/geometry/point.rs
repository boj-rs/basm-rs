use core::ops::{Add, Sub, Mul, Div, Rem};
use core::cmp::PartialOrd;

/// A trait defining common mathematical operations for types used in geometric points.
///
/// This trait allows implementation of geometry functions that require basic arithmetic
/// and square root operations. Implemented for common numeric types like `i32`, `i128`, `f64`, etc.
pub trait PointOP: Copy {
    /// The output type for floating-point conversion.
    type Output: Into<f64>;

    /// Returns zero of the type.
    fn zero() -> Self;

    /// Subtraction operation.
    fn sub(self, other: Self) -> Self;

    /// Multiplication operation.
    fn mul(self, other: Self) -> Self;

    /// Division operation.
    fn div(self, other: Self) -> Self;

    /// Remainder (modulo) operation.
    fn rem(self, other: Self) -> Self;

    /// Returns the square root of the value as `f64`.
    fn sqrt(self) -> f64;
}

macro_rules! impl_arithmetic_ops {
    ($t:ty) => {
        impl PointOP for $t {
            type Output = f64;

            fn zero() -> Self { 0 as $t }

            fn sub(self, other: Self) -> Self { self - other }

            fn mul(self, other: Self) -> Self { self * other }

            fn div(self, other: Self) -> Self { self / other }

            fn rem(self, other: Self) -> Self { self % other }

            fn sqrt(self) -> f64 { (self as f64).sqrt() }
        }
    };
}

impl_arithmetic_ops!(i128);
impl_arithmetic_ops!(f64);
impl_arithmetic_ops!(i64);
impl_arithmetic_ops!(f32);
impl_arithmetic_ops!(i32);
impl_arithmetic_ops!(i16);

/// A generic 2D point with coordinates `x` and `y`.
///
/// Supports arithmetic and geometric operations such as distance and orientation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    /// The x-coordinate.
    pub x: T,
    /// The y-coordinate.
    pub y: T,
}

impl<T> Point<T>
where
    T: PointOP + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + PartialOrd,
{
    /// Creates a new `Point` from x and y coordinates.
    ///
    /// # Example
    /// ```
    /// let p = Point::new(1, 2);
    /// assert_eq!(p.x, 1);
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Computes the orientation of the ordered triplet (self, p1, p2).
    ///
    /// # Returns
    /// * `1` if counter-clockwise,
    /// * `-1` if clockwise,
    /// * `0` if colinear.
    ///
    /// # Example
    /// ```
    /// let a = Point::new(0, 0);
    /// let b = Point::new(1, 0);
    /// let c = Point::new(0, 1);
    /// assert_eq!(a.ccw(b, c), 1);
    /// ```
    pub fn ccw(self, p1: Point<T>, p2: Point<T>) -> i64 {
        let dx1 = PointOP::sub(p1.x, self.x);
        let dy1 = PointOP::sub(p1.y, self.y);
        let dx2 = PointOP::sub(p2.x, self.x);
        let dy2 = PointOP::sub(p2.y, self.y);
        let cross = (PointOP::mul(dx1, dy2)) - (PointOP::mul(dy1, dx2));
        
        if cross > T::zero() {
            1
        } else if cross < T::zero() {
            -1
        } else {
            0
        }
    }

    /// Returns the squared distance between `self` and `other`.
    ///
    /// This avoids computing a square root and is more efficient for comparisons.
    pub fn dist_squared(self, other: Point<T>) -> T {
        let dx = PointOP::sub(self.x, other.x);
        let dy = PointOP::sub(self.y, other.y);
        PointOP::mul(dx, dx) + PointOP::mul(dy, dy)
    }

    /// Returns the Euclidean distance between `self` and `other`.
    pub fn dist(self, other: Point<T>) -> f64 {
        self.dist_squared(other).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_i128() {
        let p1 = Point::<i128>::new(0, 0);
        let p2 = Point::<i128>::new(3, 4);
        let dist = p1.dist(p2);
        assert!((dist - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_distance_f64() {
        let p1 = Point::<f64>::new(0.0, 0.0);
        let p2 = Point::<f64>::new(3.0, 4.0);
        let dist = p1.dist(p2);
        assert!((dist - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_ccw() {
        let p0 = Point::<i128>::new(0, 0);
        let p1 = Point::<i128>::new(1, 0);
        let p2 = Point::<i128>::new(0, 1);

        let result = p0.ccw(p1, p2);

        assert_eq!(result, 1); // CounterClockwise
    }

    #[test]
    fn test_ccw_clockwise() {
        let p0 = Point::<i128>::new(0, 0);
        let p1 = Point::<i128>::new(0, 1);
        let p2 = Point::<i128>::new(1, 0);

        let result = p0.ccw(p1, p2);

        assert_eq!(result, -1); // Clockwise
    }

    #[test]
    fn test_ccw_collinear() {
        let p0 = Point::<i128>::new(0, 0);
        let p1 = Point::<i128>::new(1, 1);
        let p2 = Point::<i128>::new(2, 2);

        let result = p0.ccw(p1, p2);

        assert_eq!(result, 0); // Collinear
    }
}
