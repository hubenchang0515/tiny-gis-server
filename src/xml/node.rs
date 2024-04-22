use std::{cmp::Ordering, collections::HashMap};


#[allow(dead_code)]
pub struct XmlNode {
    tag: String,
    text: String,
    attrs: HashMap<String, String>,
    nodes: Vec<XmlNode>,
}

#[allow(dead_code)]
impl XmlNode {
    pub fn new(tag: &str, text: &str) -> XmlNode {
        XmlNode { 
            tag: String::from(tag), 
            text: String::from(text), 
            attrs: HashMap::new(), 
            nodes: Vec::new(),
         }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn to_string(&self) -> String {
        let mut xml = format!("<{}", &self.tag);
        for (key, value) in &self.attrs {
            xml.push_str(&format!(r#" {}="{}""#, key, value));
        }
        xml.push_str(&format!(">{}", &self.text));
        for node in &self.nodes {
            xml.push_str(&node.to_string());
        }
        xml.push_str(&format!("</{}>", &self.tag));
        xml
    }

    pub fn set_attr(&mut self, key: &str, value: &str) {
        self.attrs.insert(String::from(key), String::from(value));
    }

    pub fn add_node(&mut self, node: XmlNode) {
        self.nodes.push(node);
    }

    pub fn sort_nodes<F>(&mut self, compare: F)
    where
        F: FnMut(&XmlNode, &XmlNode) -> Ordering,
    {
        self.nodes.sort_by(compare)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_node() {
        let mut svg = XmlNode::new("SVG", "");
        svg.set_attr("xmlns", "http://www.w3.org/2000/svg");
        svg.set_attr("width", "256");
        svg.set_attr("height", "256");
        svg.set_attr("viewBox", "0 0 256 256");

        let mut path = XmlNode::new("path", "");
        path.set_attr("id", "PATH_0");
        path.set_attr("stroke", "red");
        path.set_attr("fill", "none");
        path.set_attr("d", "M10,90 Q90,90 90,45 Q90,10 50,10 Q10,10 10,40 Q10,70 45,70 Q70,70 75,50");

        let mut text_path = XmlNode::new("textPath", "测试文本");
        text_path.set_attr("href", "#PATH_0");
        text_path.set_attr("lengthAdjust", "spacingAndGlyphs");

        let mut text = XmlNode::new("text", "");
        text.add_node(text_path);

        svg.add_node(path);
        svg.add_node(text);
        println!("{}", svg.to_string());
    }
}