use super::Point;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Polyline {
    points: Vec<Point>
}

#[allow(dead_code)]
impl Polyline {
    pub fn new() -> Polyline {
        Polyline { points: Vec::new() }
    }

    pub fn points_count(&self) -> usize {
        self.points.len()
    }

    pub fn append(&mut self, p: &Point) -> &Polyline {
        self.points.push(Point{x:p.x, y:p.y});
        self
    }

    pub fn point(&self, i: usize) -> &Point {
        &self.points[i]
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn center(&self) -> &Point {
        &self.points[self.points.len()/2]
    }
}