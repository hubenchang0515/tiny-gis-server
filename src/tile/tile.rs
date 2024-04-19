use crate::geometry::Rectangle;
use resvg::{tiny_skia::Pixmap, usvg::{fontdb::Database, Options, Transform, Tree}};
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

pub fn svg_to_png(svg: &Vec<u8>) -> Option<Vec<u8>> {
    let pixmap = Pixmap::new(255, 255);
    let mut fontdb = Database::new();
    fontdb.load_system_fonts();
    let tree = Tree::from_data(svg, &Options::default(), &fontdb);
    if pixmap.is_some() && tree.is_ok() {
        let tree = tree.unwrap();
        let mut pixmap = pixmap.unwrap();
        resvg::render(&tree, Transform::from_rotate(0.0), &mut pixmap.as_mut());
        Some(pixmap.encode_png().unwrap())
    } else {
        None
    }
}