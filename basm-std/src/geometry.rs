use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::fmt::{self, Debug, Display};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Arithmetic trait for basic operations needed in geometry
pub trait GeometricArithmetic:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + Default
{
    fn abs(self) -> Self;
    fn zero() -> Self {
        Self::default()
    }
}

// Macro to implement GeometricArithmetic for numeric types
macro_rules! impl_geometric_arithmetic {
    ($($t:ty),*) => {
        $(
            impl GeometricArithmetic for $t {
                fn abs(self) -> Self {
                    if self < Self::zero() {
                        -self
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

impl_geometric_arithmetic!(i32, i64, i128, f32, f64);

// CCW result enum for cleaner API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,
    Collinear,
    CounterClockwise,
}

impl Orientation {
    pub fn from_value<T: GeometricArithmetic>(value: T) -> Self {
        if value > T::zero() {
            Self::CounterClockwise
        } else if value < T::zero() {
            Self::Clockwise
        } else {
            Self::Collinear
        }
    }
}

// Point structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T: GeometricArithmetic> {
    pub x: T,
    pub y: T,
}

impl<T: GeometricArithmetic> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero())
    }

    // CCW as method - cleaner API
    pub fn ccw(&self, b: &Point<T>, c: &Point<T>) -> T {
        let ab = *b - *self;
        let ac = *c - *self;
        ab.cross(&ac)
    }

    // CCW returning orientation enum
    pub fn orientation(&self, b: &Point<T>, c: &Point<T>) -> Orientation {
        Orientation::from_value(self.ccw(b, c))
    }

    // Cross product (2D)
    pub fn cross(&self, other: &Point<T>) -> T {
        self.x * other.y - self.y * other.x
    }

    // Dot product
    pub fn dot(&self, other: &Point<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    // Squared magnitude
    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    // Manhattan distance
    pub fn manhattan(&self) -> T {
        self.x.abs() + self.y.abs()
    }

    // Element-wise multiplication
    pub fn hadamard(&self, other: &Point<T>) -> Point<T> {
        Point::new(self.x * other.x, self.y * other.y)
    }

    // 90-degree rotation (counter-clockwise)
    pub fn rotate_90(&self) -> Point<T> {
        Point::new(-self.y, self.x)
    }

    // Swap coordinates
    pub fn swap(&self) -> Point<T> {
        Point::new(self.y, self.x)
    }
}

// Floating point specific operations
impl<T> Point<T>
where
    T: GeometricArithmetic + Into<f64>,
{
    // CCW with epsilon tolerance for floating point
    pub fn ccw_with_epsilon(&self, b: &Point<T>, c: &Point<T>, epsilon: f64) -> Orientation {
        let ccw_val: f64 = self.ccw(b, c).into();
        if ccw_val.abs() < epsilon {
            Orientation::Collinear
        } else if ccw_val > 0.0 {
            Orientation::CounterClockwise
        } else {
            Orientation::Clockwise
        }
    }
}

// Implement standard arithmetic operations
impl<T: GeometricArithmetic> Add for Point<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: GeometricArithmetic> Sub for Point<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: GeometricArithmetic> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: GeometricArithmetic> Div<T> for Point<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: GeometricArithmetic> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: GeometricArithmetic> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: GeometricArithmetic> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<T: GeometricArithmetic> DivAssign<T> for Point<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<T: GeometricArithmetic> Neg for Point<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

// Ordering implementation
impl<T: GeometricArithmetic + Ord> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: GeometricArithmetic + Ord> Ord for Point<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            other => other,
        }
    }
}

impl<T: GeometricArithmetic> Default for Point<T> {
    fn default() -> Self {
        Self::origin()
    }
}

impl<T: GeometricArithmetic + Display> Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);

        assert_eq!(p1 + p2, Point::new(4, 6));
        assert_eq!(p2 - p1, Point::new(2, 2));
        assert_eq!(p1 * 3, Point::new(3, 6));
    }

    #[test]
    fn test_ccw() {
        let a = Point::new(0, 0);
        let b = Point::new(1, 0);
        let c = Point::new(1, 1);

        assert_eq!(a.orientation(&b, &c), Orientation::CounterClockwise);
        assert_eq!(a.ccw(&b, &c), 1);
    }

    #[test]
    fn test_floating_point_ccw() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(1.0, 1e-10);

        assert_eq!(a.orientation(&b, &c), Orientation::CounterClockwise);
        assert_eq!(a.ccw_with_epsilon(&b, &c, 1e-9), Orientation::Collinear);
    }

    #[test]
    fn test_ordering() {
        let mut points = vec![
            Point::new(1, 2),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(0, 2),
        ];

        points.sort();

        assert_eq!(
            points,
            vec![
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(1, 1),
                Point::new(1, 2),
            ]
        );
    }
}
