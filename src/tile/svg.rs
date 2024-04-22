use crate::{geometry::{Point, Polygon, Polyline, Rectangle}, xml::XmlNode};
use super::{Proj, Tile};


#[allow(dead_code)]
pub struct PolylineProps {
    pub color: String,
    pub width: usize,
    pub priority: i32,
}


#[allow(dead_code)]
impl PolylineProps {
    pub fn new(color: &str, width: usize, priority: i32) -> PolylineProps {
        PolylineProps {
            color: String::from(color),
            width,
            priority,
        }
    }
}


#[allow(dead_code)]
pub struct TextProps {
    pub fill_color: String,
    pub size: usize,
    pub weight: usize,
    pub priority: i32,
}

#[allow(dead_code)]
impl TextProps {
    pub fn new(fill_color: &str, size: usize, weight: usize, priority: i32) -> TextProps {
        TextProps {
            fill_color: String::from(fill_color),
            size,
            weight,
            priority,
        }
    }
}

#[allow(dead_code)]
pub struct PolygonProps {
    pub fill_color: String,
    pub border_color: String,
    pub border_width: usize,
    pub fill_priority: i32,
    pub border_priority: i32,
}

#[allow(dead_code)]
impl PolygonProps {
    pub fn new(fill_color: &str, border_color: &str, border_width: usize, fill_priority: i32, border_priority: i32) -> PolygonProps {
        PolygonProps {
            fill_color: String::from(fill_color),
            border_color: String::from(border_color),
            border_width,
            fill_priority,
            border_priority,
        }
    }
}

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
        xml.set_attr("width", "256");
        xml.set_attr("height", "256");
        xml.set_attr("viewBox", "0, 0, 256, 256");
        SvgTile { x, y, z, proj, xml, id_count:0 }
    }

    pub fn text_size(text: &str, text_props: &TextProps) -> Rectangle {
        Rectangle { 
            min: Point { 
                x: 0.0,
                y: 0.0 
            }, 
            max: Point { 
                x: (text.len() * text_props.size / 2) as f64, 
                y: text_props.size as f64
            }
        }
    }

    pub fn sort_tags(&mut self) {
        self.xml.sort();
    }

    pub fn append_text(&mut self, point: &Point, text: &str, props: &TextProps) {
        let mut node = XmlNode::new("text", text);
        let (x, y) = self.local((point.x, point.y));
        let text_size = SvgTile::text_size(text, props);
        node.set_attr("x", &(x - text_size.width()/2.0).to_string());
        node.set_attr("y", &y.to_string());
        node.set_attr("fill", &props.fill_color);
        node.set_attr("font-size", &props.size.to_string());
        node.set_attr("font-weight", &props.weight.to_string());
        node.set_priority(props.priority);
        self.xml.add_node(node);
    }

    pub fn append_text_path(&mut self, polyline: &Polyline, text: &str, line_props: &PolylineProps, text_props: &TextProps) {
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
        path.set_attr("stroke", &line_props.color);
        path.set_attr("stroke-width", &line_props.width.to_string());
        path.set_attr("d", &data);
        path.set_priority(line_props.priority);
        self.xml.add_node(path);

        let offset = (path_length - SvgTile::text_size(text, text_props).width()) / 2.0;

        if offset > 10.0 {
            let mut text_path = XmlNode::new("textPath", &text);
            text_path.set_attr("href", &format!("#{}", &id));
            text_path.set_attr("startOffset", &offset.to_string());
            text_path.set_attr("font-size", &text_props.size.to_string());
            text_path.set_attr("font-weight", &text_props.weight.to_string());
            text_path.set_attr("fill", &text_props.fill_color);
            
            let mut text = XmlNode::new("text", "");
            text.add_node(text_path);
            text.set_priority(text_props.priority);
            self.xml.add_node(text);
        }
    }

    pub fn append_polyline(&mut self, polyline: &Polyline, props: &PolylineProps) {
        let mut node = XmlNode::new("polyline", "");
        let mut points = Vec::new();
        for point in polyline.points() {
            let (x, y) = self.local((point.x, point.y));
            points.push(format!("{},{}", x, y));
        }
        let points = points.join(" ");
        node.set_attr("points", &points);

        let style = format!("fill:none;stroke:{};stroke-width:{}", &props.color, &props.width);
        node.set_attr("style", &style);
        node.set_priority(props.priority);
        self.xml.add_node(node);
    }

    pub fn append_polygon(&mut self, polygon: &Polygon, props: &PolygonProps) {
        let mut points = Vec::new();
        for point in polygon.points() {
            let (x, y) = self.local((point.x, point.y));
            points.push(format!("{},{}", x, y));
        }
        let points = points.join(" ");

        let mut polygon_node = XmlNode::new("polygon", "");
        let style = format!("fill:{};stroke:{};", &props.fill_color, &props.fill_color);
        polygon_node.set_attr("points", &points);
        polygon_node.set_attr("style", &style);
        polygon_node.set_priority(props.fill_priority);
        self.xml.add_node(polygon_node);

        let mut polyline_node = XmlNode::new("polyline", "");
        let style = format!("fill:none;stroke:{};stroke-width:{}", &props.border_color, &props.border_width);
        polyline_node.set_attr("points", &points);
        polyline_node.set_attr("style", &style);
        polyline_node.set_priority(props.border_priority);
        self.xml.add_node(polyline_node);
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
        self.proj().tile_local(self.x(), self.y(), self.z(), longitude_latitude)
    }

    fn rect(&self) -> Rectangle {
        self.proj().tile_rect(self.x(), self.y())
    }

    fn dump(&self) -> Vec<u8> {
        self.xml.to_string().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::proj::consts::{LATITUDE_MIN, LONGITUDE_MIN, LATITUDE_MAX, LONGITUDE_MAX};
    use super::*;

    #[test]
    fn test_svg_tile() {
        let proj = Proj::default();
        let svg = SvgTile::new(13398, 6724, 14, proj);
        assert_eq!(svg.x(), 13398);
        assert_eq!(svg.y(), 6724);
        assert_eq!(svg.z(), 14);
        assert_eq!(*svg.proj(), Proj::WGS84);

        let rect = svg.rect();
        assert_eq!(rect.min, Point{x: 13398.0 * 256.0, y: 6724.0 * 256.0});
        assert_eq!(rect.max, Point{x: 13399.0 * 256.0, y: 6725.0 * 256.0});

        let pos = svg.local((114.40,30.67));
        assert_eq!((120.6044444446452, 77.42093603871763), pos);
    }

    #[test]
    fn test_svg_render() {
        let proj = Proj::default();
        let mut svg = SvgTile::new(0, 0, 0, proj);
        svg.append_text(
            &Point { x: -180.0, y: 0.0 }, 
            "hello world", 
            &TextProps::new("red", 32, 700, 10)
        );
        
        let mut polyline = Polyline::new();
        polyline.append(&Point { x: LONGITUDE_MIN, y: LATITUDE_MIN });
        polyline.append(&Point { x: LONGITUDE_MAX, y: LATITUDE_MIN });
        polyline.append(&Point { x: LONGITUDE_MAX, y: LATITUDE_MAX });
        polyline.append(&Point { x: LONGITUDE_MIN, y: LATITUDE_MAX });
        svg.append_polyline(
            &polyline, 
            &PolylineProps::new("red", 5, 10)
        );
        svg.append_text_path(
            &polyline, 
            "TEXT PATH", 
            &PolylineProps::new("green", 3, 10), 
            &TextProps::new("blue", 32, 700, 10)
        );

        let mut polygon = Polygon::new();
        polygon.append(&Point { x: 0.0, y: LATITUDE_MAX });
        polygon.append(&Point { x: 0.0, y: LATITUDE_MIN });
        polygon.append(&Point { x: LONGITUDE_MAX, y: LATITUDE_MIN });
        polygon.append(&Point { x: LONGITUDE_MAX, y: LATITUDE_MAX });
        svg.append_polygon(
            &polygon,
            &PolygonProps::new("green", "cyan", 3, 5, 10)
        );

        svg.sort_tags();
        println!("SVG:{}", String::from_utf8(svg.dump()).unwrap());
    }
}