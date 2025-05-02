use crate::utils::ToF64;
use crate::utils::F64Ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: ToF64 + Copy> Point<T> {
    pub fn dist(self, other: Point<T>) -> f64 {
        let dx = self.x.to_f64() - other.x.to_f64();
        let dy = self.y.to_f64() - other.y.to_f64();
        (dx * dx + dy * dy).sqrt()
    }
}

#[test]
fn test_distance_i128() {
    let p1 = Point::<i128>::new(0, 0);
    let p2 = Point::<i128>::new(3, 4);
    assert!((p1.dist(p2) - 5.0).abs() < 1e-9);
}
