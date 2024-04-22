use std::f64::consts::PI;

use crate::geometry::{Point, Rectangle};

#[derive(Debug, PartialEq)]
pub enum Proj {
    WGS84,
}

#[allow(dead_code)]
pub mod consts {
    pub const LATITUDE_MIN: f64 = -85.051129;
    pub const LATITUDE_MAX: f64 = 85.051129;
    pub const LONGITUDE_MIN: f64 = -180.0;
    pub const LONGITUDE_MAX: f64 = 180.0;
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
            Proj::WGS84 => 256.0 / (2.0*PI) * 2.0f64.powf(zoom) * (PI - (PI/4.0 + v.to_radians()/2.0).tan().ln()),
        }
    }

    pub fn tile_rect(&self, x: u64, y: u64) -> Rectangle {
        let min = Point::new((x * 256) as f64, (y * 256) as f64);
        let max = Point::new(((x+1) * 256) as f64, ((y+1) * 256) as f64);
        Rectangle::new(&min, &max)
    }

    pub fn tile_local(&self, x: u64, y: u64, z: u64, longitude_latitude: (f64, f64)) -> (f64, f64) {
        let local_x = self.longitude_to_x(longitude_latitude.0, z as f64) - self.tile_rect(x, y).min.x;
        let local_y = self.latitude_to_y(longitude_latitude.1, z as f64) - self.tile_rect(x, y).min.y;
        (local_x, local_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::consts::*;

    #[test]
    fn test_to_xy() {
        let proj = Proj::default();
        assert_eq!(proj, Proj::WGS84);
        assert_eq!(proj.longitude_to_x(LONGITUDE_MIN, 0.0), 0.0);
        assert_eq!(proj.longitude_to_x(0.0, 0.0), 128.0);
        assert_eq!(proj.longitude_to_x(LONGITUDE_MAX, 0.0), 256.0);
        assert_eq!(proj.latitude_to_y(0.0, 0.0), 128.0);
        assert!((proj.latitude_to_y(LATITUDE_MAX, 0.0) - 0.0).abs() < 1e-5);
        assert!((proj.latitude_to_y(LATITUDE_MIN, 0.0) - 256.0).abs()< 1e-5);
    }

    #[test]
    fn test_tile_rect() {
        let proj = Proj::default();
        let rect = proj.tile_rect(11, 17);
        assert_eq!(rect.min, Point{x:11.0 * 256.0, y: 17.0 * 256.0});
        assert_eq!(rect.max, Point{x:12.0 * 256.0, y: 18.0 * 256.0});
    }

    #[test]
    fn test_tile_local() {
        let proj = Proj::default();
        let pos = proj.tile_local(13398, 6724, 14, (114.40,30.67));
        assert_eq!((120.6044444446452, 77.42093603871763), pos);
    }
}