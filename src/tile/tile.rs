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
    let pixmap = Pixmap::new(256, 256);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_svg_to_png() {
        let svg = r#"
        <svg xmlns="http://www.w3.org/2000/svg" width="300" height="200">
            <rect width="100%" height="100%" stroke="cyan" stroke-width="4" fill="red" />
            <circle cx="150" cy="100" r="80" fill="green" />
            <text x="150" y="115" font-size="16" text-anchor="middle" fill="white">文本测试</text>
        </svg>
        "#.as_bytes().to_vec();
        
        println!("{:?}", svg_to_png(&svg).unwrap());
    }
}