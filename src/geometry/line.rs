use super::{BaseType, NAN, EPSILON, Point};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    p1: Point,
    p2: Point,
}


#[allow(dead_code)]
impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Line {
        Line { 
            p1: p1.clone(),
            p2: p2.clone(),
        }
    }

    pub fn uninitialized() -> Line {
        Line { 
            p1: Point{x: NAN, y: NAN },
            p2: Point{x: NAN, y: NAN },
        }
    }

    pub fn points(&self) -> (&Point, &Point) {
        (&self.p1, &self.p2)
    }

    pub fn length(&self) -> BaseType {
        self.p1.distance(&self.p2)
    }

    pub fn is_intersect(&self, line: &Line) -> bool {
        let vec1 = &self.p2 - &self.p1;
        let vec2 = &line.p1 - &self.p1;
        let vec3 = &line.p2 - &self.p1;

        if vec1.cross(&vec2) * vec1.cross(&vec3) > 0.0 {
            return false;
        }

        let vec1 = &line.p2 - &line.p1;
        let vec2 = &self.p1 - &line.p1;
        let vec3 = &self.p2 - &line.p1;
        vec1.cross(&vec2) * vec1.cross(&vec3) <= 0.0
    }

    pub fn is_orthogonal(&self, line: &Line) -> bool {
        let vec1 = &self.p2 - &self.p1;
        let vec2 = &line.p2 - &line.p1;
        vec1.dot(&vec2).abs() < EPSILON
    }

    pub fn is_parallel(&self, line: &Line) -> bool {
        let vec1 = &self.p2 - &self.p1;
        let vec2 = &line.p2 - &line.p1;
        vec1.cross(&vec2).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let l1 = Line::new(&Point::new(1.0, 2.0), &Point::new(3.0, 4.0));
        let l2 = Line::new(&Point::new(2.0, 3.0), &Point::new(4.0, 5.0));
        assert_eq!(l1, l1);
        assert_eq!(l2, l2);
        assert_ne!(l1, l2);
    }

    #[test]
    fn test_length() {
        let l = Line::new(&Point::new(1.0, 2.0), &Point::new(4.0, 6.0));
        assert_eq!(5.0, l.length());
    }

    #[test]
    fn test_is_intersect() {
        let l1 = Line::new(&Point::new(1.0, 2.0), &Point::new(3.0, 4.0));
        let l2 = Line::new(&Point::new(1.0, 3.0), &Point::new(3.0, 5.0));
        let l3 = Line::new(&Point::new(1.0, 3.0), &Point::new(3.0, 2.0));
        assert_eq!(false, l1.is_intersect(&l2));
        assert_eq!(true, l1.is_intersect(&l3));
        assert_eq!(true, l2.is_intersect(&l3));
    }

    #[test]
    fn test_is_parallel() {
        let l1 = Line::new(&Point::new(0.0, 0.0), &Point::new(1.0, 1.0));
        let l2 = Line::new(&Point::new(1.0, 3.0), &Point::new(2.0, 4.0));
        let l3 = Line::new(&Point::new(1.0, 3.0), &Point::new(3.0, 2.0));
        assert_eq!(true, l1.is_parallel(&l2));
        assert_eq!(false, l1.is_parallel(&l3));
        assert_eq!(false, l2.is_parallel(&l3));
    }

    #[test]
    fn test_is_orthogonal() {
        let l1 = Line::new(&Point::new(0.0, 0.0), &Point::new(0.0, 1.0));
        let l2 = Line::new(&Point::new(0.0, 0.0), &Point::new(1.0, 0.0));
        let l3 = Line::new(&Point::new(0.0, 0.0), &Point::new(1.0, 1.0));
        assert_eq!(true, l1.is_orthogonal(&l2));
        assert_eq!(false, l1.is_orthogonal(&l3));
        assert_eq!(false, l2.is_orthogonal(&l3));
    }
}