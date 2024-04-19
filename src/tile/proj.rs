use std::f64::consts::PI;

pub enum Proj {
    WGS84,
}

#[allow(dead_code)]
impl Proj {
    pub fn default() -> Proj {
        Proj::WGS84
    }

    pub fn longitude_to_x(&self, v: f64, zoom: f64) -> f64 {
        match self {
            Proj::WGS84 => 256.0 / (2.0*PI) * (v.to_radians() + PI) * 2.0f64.powf(zoom),
        }
    }
    
    pub fn latitude_to_y(&self,v: f64, zoom: f64) -> f64 {
        match self {
            Proj::WGS84 => 256.0 / (2.0*PI)* (PI - (PI/4.0 + v.to_radians()/2.0).tan().ln()) * 2.0f64.powf(zoom),
        }
    }
}
