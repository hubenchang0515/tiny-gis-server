use std::ops;
use super::{BaseType, NAN};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: BaseType,
    pub y: BaseType,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: BaseType, y: BaseType) -> Point {
        Point {x, y}
    }

    pub fn uninitialized() -> Point {
        Point { x: NAN, y: NAN }
    }

    pub fn is_valid(&self) -> bool {
        !self.x.is_nan() && !self.y.is_nan()
    }

    pub fn to_tuple(&self) -> (BaseType, BaseType) {
        (self.x, self.y)
    }

    pub fn dot(&self, p: &Point) -> BaseType {
        let product = self * p;
        product.x + product.y
    }

    pub fn cross(&self, p: &Point) -> BaseType {
        self.x * p.y - self.y * p.x
    }

    pub fn distance(&self, p: &Point) -> BaseType {
        let delta = self - p;
        Point::dot(&delta, &delta).powf(0.5)
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;
    
    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }    
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Point;
    
    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }    
    }
}

impl ops::Mul<&Point> for &Point {
    type Output = Point;
    
    fn mul(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }    
    }
}

impl ops::Div<&Point> for &Point {
    type Output = Point;
    
    fn div(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }    
    }
}


#[cfg(test)]
mod tests {
    use crate::geometry::NAN;
    use super::*;

    #[test]
    fn test_eq() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(233.0, 666.0);
        assert_eq!(p1, p1);
        assert_eq!(p2, p2);
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_add() {
        let p1 = Point::new(3.0, 6.0);
        let p2 = Point::new(-2.0, 4.0);
        let p = &p1 + &p2;
        assert_eq!(1.0, p.x);
        assert_eq!(10.0, p.y);
    }

    #[test]
    fn test_sub() {
        let p1 = Point::new(3.0, 6.0);
        let p2 = Point::new(-2.0, 4.0);
        let p = &p1 - &p2;
        assert_eq!(5.0, p.x);
        assert_eq!(2.0, p.y);
    }

    #[test]
    fn test_mul() {
        let p1 = Point::new(3.0, 6.0);
        let p2 = Point::new(-2.0, 4.0);
        let p = &p1 * &p2;
        assert_eq!(-6.0, p.x);
        assert_eq!(24.0, p.y);
    }

    #[test]
    fn test_div() {
        let p1 = Point::new(3.0, 6.0);
        let p2 = Point::new(-2.0, 4.0);
        let p = &p1 / &p2;
        assert_eq!(-1.5, p.x);
        assert_eq!(1.5, p.y);
    }

    #[test]
    fn test_valid() {
        let p = Point::new(3.0, 6.0);
        assert_eq!(true, p.is_valid());

        let p = Point::new(3.0, NAN);
        assert_eq!(false, p.is_valid());

        let p = Point::new(NAN, 3.0);
        assert_eq!(false, p.is_valid());

        let p = Point::new(NAN, NAN);
        assert_eq!(false, p.is_valid());
    }

    #[test]
    fn test_to_tuple() {
        let p = Point::new(1.0, 2.0);
        assert_eq!((1.0, 2.0), p.to_tuple());

        let p = Point::new(233.0, 666.0);
        assert_eq!((233.0, 666.0), p.to_tuple());
    }

    #[test]
    fn test_dot() {
        let p1 = Point::new(3.0, 6.0);
        let p2 = Point::new(-2.0, 4.0);
        assert_eq!(18.0, p1.dot(&p2))
    }

    #[test]
    fn test_cross() {
        let p1 = Point::new(3.0, 6.0);
        let p2 = Point::new(-2.0, 4.0);
        assert_eq!(24.0, p1.cross(&p2))
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(1.0, -1.0);
        let p2 = Point::new(-2.0, 3.0);
        assert_eq!(5.0, p1.distance(&p2))
    }
}