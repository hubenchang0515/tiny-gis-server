use super::Point;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Polygon {
    points: Vec<Point>
}

#[allow(dead_code)]
impl Polygon {
    pub fn new() -> Polygon {
        Polygon { points: Vec::new() }
    }

    pub fn points_count(&self) -> usize {
        self.points.len()
    }

    pub fn append(&mut self, p: &Point) -> &Polygon {
        self.points.push(Point{x:p.x, y:p.y});
        self
    }

    pub fn point(&self, i: usize) -> &Point {
        &self.points[i]
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon() {
        let mut polygon = Polygon::new();
        polygon.append(&Point { x: 0.0, y: 0.0 });
        polygon.append(&Point { x: 1.0, y: 0.0 });
        assert_eq!(polygon.points_count(), 2);
        polygon.append(&Point { x: 1.0, y: 1.0 });
        polygon.append(&Point { x: 0.0, y: 1.0 });
        assert_eq!(polygon.points_count(), 4);

        assert_eq!(polygon.point(0), &Point { x: 0.0, y: 0.0 });
        assert_eq!(polygon.point(1), &Point { x: 1.0, y: 0.0 });
        assert_eq!(polygon.point(2), &Point { x: 1.0, y: 1.0 });
        assert_eq!(polygon.point(3), &Point { x: 0.0, y: 1.0 });

        assert_eq!(polygon.points[0], Point { x: 0.0, y: 0.0 });
        assert_eq!(polygon.points[1], Point { x: 1.0, y: 0.0 });
        assert_eq!(polygon.points[2], Point { x: 1.0, y: 1.0 });
        assert_eq!(polygon.points[3], Point { x: 0.0, y: 1.0 });
    }
}