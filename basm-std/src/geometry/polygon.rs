use core::ops::{Add, Div, Mul, Rem, Sub};

use crate::alloc::vec::Vec;
use crate::geometry::point::Point;
use crate::utils::F64Ops;

use super::point::PointOP;

/// A 2D polygon represented as a vector of points.
///
/// The polygon is assumed to be simple (no self-intersections),
/// and vertices should be listed in either clockwise or counter-clockwise order.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon<T> {
    /// Vertices of the polygon in order.
    pub verticles: Vec<Point<T>>,
}

macro_rules! impl_polygon_ops {
    ($t: ty, $acc: ty) => {
        impl Polygon<$t> {
            /// Computes the (unsigned) area of the polygon using the shoelace formula.
            ///
            /// Returns `0.0` if the polygon has fewer than 3 vertices.
            ///
            /// # Example
            /// ```
            /// let poly = Polygon::<i32> { verticles: vec![Point::new(0, 0), Point::new(1, 0), Point::new(0, 1)] };
            /// let area = poly.area();
            /// assert_eq!(area, 0.5);
            /// ```
            pub fn area(&self) -> f64 {
                let n = self.verticles.len();
                if n < 3 {
                    return 0.0;
                }
                let mut sum: $acc = 0;
                for i in 0..n {
                    let Point { x: x0, y: y0 } = self.verticles[i];
                    let Point { x: x1, y: y1 } = self.verticles[(i + 1) % n];
                    sum += (x0 as $acc) * (y1 as $acc) - (x1 as $acc) * (y0 as $acc);
                }
                (sum.abs() as f64) * 0.5
            }
        }
    };

    (float $t: ty) => {
        impl Polygon<$t> {
            /// Computes the (unsigned) area of the polygon using the shoelace formula.
            ///
            /// Returns `0.0` if the polygon has fewer than 3 vertices.
            pub fn area(&self) -> f64 {
                let n = self.verticles.len();
                if n < 3 {
                    return 0.0;
                }
                let mut sum = 0.0;
                for i in 0..n {
                    let Point { x: x0, y: y0 } = self.verticles[i];
                    let Point { x: x1, y: y1 } = self.verticles[(i + 1) % n];
                    sum += x0 * y1 - x1 * y0;
                }
                (sum.abs() * 0.5).into()
            }
        }
    }
}

impl_polygon_ops!(i32, i64);
impl_polygon_ops!(i64, i128);
impl_polygon_ops!(float f32);
impl_polygon_ops!(float f64);

impl<T> Polygon<T>
where
    T: PointOP + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    /// Computes the perimeter (total edge length) of the polygon.
    ///
    /// Returns `0.0` if there are fewer than 2 points.
    pub fn perimeter(&self) -> f64 {
        let n = self.verticles.len();
        let mut perimeter = 0.0;
        for i in 0..n {
            let p1 = self.verticles[i];
            let p2 = self.verticles[(i + 1) % n];
            perimeter += p1.dist(p2);
        }
        perimeter
    }
}

macro_rules! impl_signed_area {
    ($t:ty) => {
        impl Polygon<$t> {
            /// Computes the signed area of the polygon.
            ///
            /// The sign of the area indicates the orientation:
            /// - Positive if the vertices are ordered counter-clockwise (CCW)
            /// - Negative if clockwise (CW)
            ///
            /// Returns `0` if the polygon has fewer than 3 vertices.
            ///
            /// # Example
            /// ```
            /// let poly = Polygon::<$t> {
            ///     verticles: vec![
            ///         Point::new(0, 0),
            ///         Point::new(1, 0),
            ///         Point::new(0, 1),
            ///     ]
            /// };
            /// let signed = poly.signed_area();
            /// assert!(signed > 0);
            /// ```
            pub fn signed_area(&self) -> $t {
                let n = self.verticles.len();
                if n < 3 {
                    return 0;
                }
                let mut sum: $t = 0;
                for i in 0..n {
                    let Point { x: x0, y: y0 } = self.verticles[i];
                    let Point { x: x1, y: y1 } = self.verticles[(i + 1) % n];
                    sum += x0 * y1 - x1 * y0;
                }
                sum
            }
        }
    };
}

impl_signed_area!(i128);
impl_signed_area!(i64);
impl_signed_area!(i32);

impl Polygon<f64> {
    /// Computes the signed area of a floating-point polygon.
    ///
    /// See `signed_area` for integer types for more details.
    pub fn signed_area(&self) -> f64 {
        let n = self.verticles.len();
        if n < 3 {
            return 0.0;
        }
        let mut sum: f64 = 0.0;
        for i in 0..n {
            let Point { x: x0, y: y0 } = self.verticles[i];
            let Point { x: x1, y: y1 } = self.verticles[(i + 1) % n];
            sum += x0 * y1 - x1 * y0;
        }
        sum
    }
}

impl Polygon<f32> {
    /// Computes the signed area of a floating-point polygon.
    ///
    /// See `signed_area` for integer types for more details.
    pub fn signed_area(&self) -> f32 {
        let n = self.verticles.len();
        if n < 3 {
            return 0.0;
        }
        let mut sum: f32 = 0.0;
        for i in 0..n {
            let Point { x: x0, y: y0 } = self.verticles[i];
            let Point { x: x1, y: y1 } = self.verticles[(i + 1) % n];
            sum += x0 * y1 - x1 * y0;
        }
        sum
    }
}
