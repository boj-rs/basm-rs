
use crate::utils::f64;
use crate::utils::ToF64;
pub mod point;
use crate::geometry::point::Point;

pub fn ccw<T>(a: Point<T>, b: Point<T>, c: Point<T>) -> f64
where
    T: ToF64 + Copy,
{
    let a_x = a.x.to_f64();
    let a_y = a.y.to_f64();
    let b_x = b.x.to_f64();
    let b_y = b.y.to_f64();
    let c_x = c.x.to_f64();
    let c_y = c.y.to_f64();

    (b_x - a_x) * (c_y - a_y) - (b_y - a_y) * (c_x - a_x)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_equal_f64() {
        let p1 = Point::<f64>::new(3.0, 5.0);
        let p2 = Point::<f64>::new(3.0, 5.0);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_point_equal_i128() {
        let p1 = Point::<i128>::new(3, 5);
        let p2 = Point::<i128>::new(3, 5);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_ccw_counter_clockwise_f64() {
        let a = Point::<f64>::new(0.0, 0.0);
        let b = Point::<f64>::new(1.0, 0.0);
        let c = Point::<f64>::new(1.0, 1.0);
        assert!(ccw(a, b, c) > 0.0);
    }

    #[test]
    fn test_ccw_clockwise_i128() {
        let a = Point::<i128>::new(0, 0);
        let b = Point::<i128>::new(1, 0);
        let c = Point::<i128>::new(1, -1);
        assert!(ccw(a, b, c) < 0.0);
    }

    #[test]
    fn test_distance_f64() {
        let p1 = Point::<f64>::new(0.0, 0.0);
        let p2 = Point::<f64>::new(3.0, 4.0);
        assert!((p1.dist(p2) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_distance_i128() {
        let p1 = Point::<i128>::new(0, 0);
        let p2 = Point::<i128>::new(3, 4);
        assert!((p1.dist(p2) - 5.0).abs() < 1e-9);
    }
}
