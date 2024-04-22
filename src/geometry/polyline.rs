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
        &self.points[(self.points.len()+1)/2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyline() {
        let mut polyline = Polyline::new();
        polyline.append(&Point { x: 0.0, y: 0.0 });
        polyline.append(&Point { x: 1.0, y: 0.0 });
        assert_eq!(polyline.points_count(), 2);
        polyline.append(&Point { x: 1.0, y: 1.0 });
        polyline.append(&Point { x: 0.0, y: 1.0 });
        assert_eq!(polyline.points_count(), 4);

        assert_eq!(polyline.point(0), &Point { x: 0.0, y: 0.0 });
        assert_eq!(polyline.point(1), &Point { x: 1.0, y: 0.0 });
        assert_eq!(polyline.point(2), &Point { x: 1.0, y: 1.0 });
        assert_eq!(polyline.point(3), &Point { x: 0.0, y: 1.0 });

        assert_eq!(polyline.points[0], Point { x: 0.0, y: 0.0 });
        assert_eq!(polyline.points[1], Point { x: 1.0, y: 0.0 });
        assert_eq!(polyline.points[2], Point { x: 1.0, y: 1.0 });
        assert_eq!(polyline.points[3], Point { x: 0.0, y: 1.0 });

        assert_eq!(polyline.center(), polyline.point(2));
        polyline.append(&Point { x: 0.0, y: 1.0 });
        assert_eq!(polyline.center(), polyline.point(3));
    }
}