use std::cmp::Ordering;

use crate::{geometry::{Point, Polygon, Polyline, Rectangle}, xml::XmlNode};
use super::{Proj, Tile};



#[allow(dead_code)]
pub struct SvgTile {
    x: u64,
    y: u64,
    z: u64,
    proj: Proj,
    xml: XmlNode,
    id_count: usize,
}

#[allow(dead_code)]
impl SvgTile {
    pub fn new(x: u64, y: u64, z: u64, proj: Proj) -> SvgTile {
        let mut xml = XmlNode::new("svg", "");
        xml.set_attr("xmlns", "http://www.w3.org/2000/svg");
        xml.set_attr("width", "255");
        xml.set_attr("height", "255");
        xml.set_attr("viewBox", "0, 0, 255, 255");
        SvgTile { x, y, z, proj, xml, id_count:0 }
    }

    pub fn sort_tags(&mut self) {
        self.xml.sort_nodes(|a, b|{
            if a.tag() == "text" {
                Ordering::Greater
            } else if b.tag() == "text" {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    }

    pub fn append_text(&mut self, point: &Point, text: &str) {
        let mut node = XmlNode::new("text", text);
        let (x, y) = self.local((point.x, point.y));
        node.set_attr("x", &x.to_string());
        node.set_attr("y", &y.to_string());
        self.xml.add_node(node);
    }

    pub fn append_text_path(&mut self, polyline: &Polyline, text: &str, line_color: &str, line_width: usize) {
        let mut data = String::new();
        let mut first = true;
        let mut prev = Point::uninitialized();
        let mut path_length = 0.0;
        for point in polyline.points() {
            let (x, y) = self.local((point.x, point.y));
            if first {
                data.push_str(&format!("M{},{}", x, y));
                first = false;
                prev.x = x;
                prev.y = y;
            } else {
                data.push_str(&format!(" L{},{}", x, y));
                let current = Point::new(x, y);
                path_length += prev.distance(&current);
                prev = current;
            }
        }
        
        self.id_count = self.id_count + 1;
        let id: String = format!("ID_{}", self.id_count);
        let mut path = XmlNode::new("path", "");
        path.set_attr("id", &id);
        path.set_attr("fill", "none");
        path.set_attr("stroke", line_color);
        path.set_attr("stroke-width", &line_width.to_string());
        path.set_attr("d", &data);
        self.xml.add_node(path);

        let offset = path_length / 2.0;
        let mut text_path = XmlNode::new("textPath", &text);
        text_path.set_attr("href", &format!("#{}", &id));
        text_path.set_attr("startOffset", &offset.to_string());
        text_path.set_attr("font-size", "20");
        text_path.set_attr("font-weight", "700");
        text_path.set_attr("fill", "black");
        
        let mut text = XmlNode::new("text", "");
        text.add_node(text_path);
        self.xml.add_node(text);
    }

    pub fn append_polyline(&mut self, polyline: &Polyline, color: &str, width: usize) {
        let mut node = XmlNode::new("polyline", "");
        let mut points = Vec::new();
        for point in polyline.points() {
            let (x, y) = self.local((point.x, point.y));
            points.push(format!("{},{}", x, y));
        }
        let points = points.join(" ");
        node.set_attr("points", &points);

        let style = format!("fill:none;stroke:{};stroke-width:{}", color, width);
        node.set_attr("style", &style);
        self.xml.add_node(node);
    }

    pub fn append_polygon(&mut self, polygon: &Polygon, fill_color: &str, line_color: &str, line_width: usize) {
        let mut node = XmlNode::new("polygon", "");
        let mut points = Vec::new();
        for point in polygon.points() {
            let (x, y) = self.local((point.x, point.y));
            points.push(format!("{},{}", x, y));
        }
        let points = points.join(" ");
        node.set_attr("points", &points);

        let style = format!("fill:{};stroke:{};stroke-width:{}", fill_color, line_color, line_width);
        node.set_attr("style", &style);
        self.xml.add_node(node);
    }
}

impl Tile for SvgTile {
    fn x(&self) -> u64 {
        self.x
    }

    fn y(&self) -> u64 {
        self.y
    }

    fn z(&self) -> u64 {
        self.z
    }

    fn proj(&self) -> &super::Proj {
        &self.proj
    }

    fn local(&self, longitude_latitude: (f64, f64)) -> (f64, f64) {
        let x = self.proj.longitude_to_x(longitude_latitude.0, self.z() as f64) - self.rect().min.x;
        let y = self.proj.latitude_to_y(longitude_latitude.1, self.z() as f64) - self.rect().min.y;
        (x, y)
    }

    fn rect(&self) -> Rectangle {
        let min = Point::new(self.x as f64 * 255.0, self.y as f64 * 255.0);
        let max = Point::new((self.x+1) as f64 * 255.0, (self.y+1) as f64 * 255.0);
        Rectangle::new(&min, &max)
    }

    fn dump(&self) -> Vec<u8> {
        self.xml.to_string().as_bytes().to_vec()
    }
}