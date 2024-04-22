use super::{NAN, Point};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub min: Point,
    pub max: Point,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(min: &Point, max: &Point) -> Rectangle {
        Rectangle { 
            min: min.clone(),
            max: max.clone(),
        }
    }

    pub fn uninitialized() -> Rectangle {
        Rectangle { 
            min: Point{x: NAN, y: NAN },
            max: Point{x: NAN, y: NAN },
        }
    }

    pub fn is_intersect(&self, rect: &Rectangle) -> bool {
        self.max.x >= rect.min.x && self.max.y >= rect.min.y && self.min.x <= rect.max.x && self.min.y <= rect.max.y
    }

    pub fn area(&self) -> f64 {
        let vec = &self.max - &self.min;
        vec.x * vec.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::uninitialized();
        assert_eq!(rect.min.is_valid(), false);
        assert_eq!(rect.max.is_valid(), false);

        let rect = Rectangle::new(&Point { x: 0.0, y: 0.0 }, &Point { x: 1.0, y: 1.0 });
        assert_eq!(rect.min.is_valid(), true);
        assert_eq!(rect.max.is_valid(), true);
    }

    #[test]
    fn test_is_intersect() {
        let rect1 = Rectangle::uninitialized();
        let rect2 = Rectangle::new(&Point { x: 0.0, y: 0.0 }, &Point { x: 1.0, y: 1.0 });
        assert_eq!(rect1.is_intersect(&rect2), false);

        let rect1 = Rectangle::new(&Point { x: 0.5, y: 0.5 }, &Point { x: 1.5, y: 1.5 });
        assert_eq!(rect1.is_intersect(&rect2), true);

        let rect1 = Rectangle::new(&Point { x: 1.0, y: 1.0 }, &Point { x: 2.0, y: 2.0 });
        assert_eq!(rect1.is_intersect(&rect2), true);

        let rect1 = Rectangle::new(&Point { x: 1.5, y: 1.5 }, &Point { x: 2.5, y: 2.5 });
        assert_eq!(rect1.is_intersect(&rect2), false);
    }

    #[test]
    fn test_area()
    {
        let rect = Rectangle::uninitialized();
        assert_eq!(rect.area().is_nan(), true);

        let rect = Rectangle::new(&Point { x: -3.0, y: -2.0 }, &Point { x: 11.0, y: 13.0 });
        assert_eq!(rect.area().is_nan(), false);
        assert_eq!(rect.area(), 210.0);
    }
}