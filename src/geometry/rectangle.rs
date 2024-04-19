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
        return  vec.x * vec.y;
    }
}