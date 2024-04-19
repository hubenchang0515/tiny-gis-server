use super::{Point, Line, Polyline, Polygon, Rectangle};

pub type BaseType = f64;
pub const NAN: BaseType = f64::NAN;
pub const EPSILON: BaseType = std::f64::EPSILON;

#[allow(dead_code)]
pub enum Shape {
    Point(Point),
    Line(Line),
    Polyline(Polyline),
    Polygon(Polygon),
    Rectangle(Rectangle),
}