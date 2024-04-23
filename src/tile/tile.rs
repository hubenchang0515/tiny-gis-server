use crate::geometry::Rectangle;
use super::Proj;

pub trait Tile {
    fn x(&self) -> u64;
    fn y(&self) -> u64;
    fn z(&self) -> u64;
    fn proj(&self) -> &Proj;
    fn local(&self, longitude_latitude: (f64, f64)) -> (f64, f64);
    fn rect(&self) -> Rectangle;
    fn dump(&self) -> Vec<u8>;
}
