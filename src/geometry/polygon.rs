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