use crate::utils::ToI128;
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

impl<T: ToI128 + Copy> Point<T> {
    pub fn dist(self, other: Point<T>) -> i128 {
        let dx = self.x.to_i128() - other.x.to_i128();
        let dy = self.y.to_i128() - other.y.to_i128();
        dx*dx + dy*dy
    }
}

#[test]
fn test_distance_i128() {
    let p1 = Point::<i128>::new(0, 0);
    let p2 = Point::<i128>::new(3, 4);
    assert!((p1.dist(p2) - 5.0).abs() < 1e-9);
}
