use crate::geometry::{Point, Polygon, Polyline, Rectangle, Shape};
use super::Info;

#[allow(dead_code)]
pub struct Node{
    pub shape: Shape,
    pub info: Info,
}

#[allow(dead_code)]
pub struct ShapeFile {
    nodes: Vec<Node>
}

#[allow(dead_code)]
impl ShapeFile {
    pub fn new() -> ShapeFile {
        ShapeFile {nodes:Vec::new()}
    }

    pub fn load(&mut self, file: &str) {
        let mut reader = shapefile::Reader::from_path(file).unwrap();
        for shape_record in reader.iter_shapes_and_records() {
            let (shape, record) = shape_record.unwrap();
            let mut info = Info{
                name: String::from(""),
                rect: Rectangle::uninitialized(),
            };
            
            if let Some(name) = record.get("name") {
                if let shapefile::dbase::FieldValue::Character(name) = name {
                    if let Some(name) = name {
                        info.name = name.to_string();
                    }
                }
            }

            match shape {
                shapefile::Shape::Point(node) => {
                    let point = Point{x: node.x, y: node.y};
                    let shape = Shape::Point(point);
                    self.nodes.push(Node { shape, info });
                },

                shapefile::Shape::Polyline(node) => {
                    let bbox = node.bbox();
                    let min = Point{x: bbox.min.x, y: bbox.min.y};
                    let max = Point{x: bbox.max.x, y: bbox.max.y};
                    info.rect  = Rectangle::new(&min, &max);
                    
                    for part in node.parts() {
                        let mut polyline = Polyline::new();
                        for point in part {
                            polyline.append(&Point { x: point.x, y: point.y });
                        }
                        let shape = Shape::Polyline(polyline);
                        self.nodes.push(Node { shape, info: info.clone() });
                    }
                },

                shapefile::Shape::Polygon(node) => {
                    let bbox = node.bbox();
                    let min = Point{x: bbox.min.x, y: bbox.min.y};
                    let max = Point{x: bbox.max.x, y: bbox.max.y};
                    info.rect  = Rectangle::new(&min, &max);

                    for ring in node.rings() {
                        if let shapefile::PolygonRing::Inner(_) = ring {
                            continue;
                        }
                        let mut polygon = Polygon::new();
                        for point in ring.points() {
                            polygon.append(&Point { x: point.x, y: point.y });
                        }
                        let shape = Shape::Polygon(polygon);
                        self.nodes.push(Node { shape, info: info.clone() });
                    }
                },

                _ => {},
            }
        }
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shapefile() {
        let mut shp = ShapeFile::new();
        shp.load("resource/wuhan/wuhan_road.shp");
        for node in shp.nodes() {
            if !node.info.name.is_empty() {
                println!("{}", node.info.name);
            }
        }
    }

}