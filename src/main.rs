mod geometry;
mod geography;
mod tile;
mod xml;
mod cache;

use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use cache::{Cache, MemoryCache};
use geography::ShapeFile;
use geometry::{Shape, Point};
use tile::{PolygonProps, PolylineProps, SvgTile, TextProps, Tile};

use crate::{geometry::Rectangle, tile::{tile::svg_to_png, Proj}};

#[allow(dead_code)]
pub mod colors {
    pub const REGION_FILL_COLOR: &str = "#d3f8e2";
    pub const REGION_BORDER_COLOR: &str = "#5491f5";
    pub const REGION_TEXT_COLOR: &str = "#5491f5";

    pub const WATER_FILL_COLOR: &str = "#90daee";
    pub const WATER_BORDER_COLOR: &str = "#90daee";
    pub const WATER_TEXT_COLOR: &str = "#000000";

    pub const LAND_FILL_COLOR: &str = "#f5f0e5";
    pub const LAND_BORDER_COLOR: &str = "#aab9c9";
    pub const LAND_TEXT_COLOR: &str = "#000000";

    pub const ROAD_FILL_COLOR: &str = "#aab9c9";
    pub const ROAD_BORDER_COLOR: &str = "#78909c";
    pub const ROAD_TEXT_COLOR: &str = "#000000";

    pub const BUILDING_FILL_COLOR: &str = "#e8e9ed";
    pub const BUILDING_BORDER_COLOR: &str = "#aab9c9";
    pub const BUILDING_TEXT_COLOR: &str = "#000000";
}

#[allow(dead_code)]
pub mod priorities {
    pub const REGION_FILL_PRIORITY: i32 = 10;
    pub const REGION_BORDER_PRIORITY: i32 = 20;
    pub const REGION_TEXT_PRIORITY: i32 = 30;

    pub const WATER_FILL_PRIORITY: i32 = 10;
    pub const WATER_BORDER_PRIORITY: i32 = 20;
    pub const WATER_TEXT_PRIORITY: i32 = 30;

    pub const LAND_FILL_PRIORITY: i32 = 10;
    pub const LAND_BORDER_PRIORITY: i32 = 20;
    pub const LAND_TEXT_PRIORITY: i32 = 30;

    pub const ROAD_FILL_PRIORITY: i32 = 10;
    pub const ROAD_BORDER_PRIORITY: i32 = 20;
    pub const ROAD_TEXT_PRIORITY: i32 = 30;

    pub const BUILDING_FILL_PRIORITY: i32 = 10;
    pub const BUILDING_BORDER_PRIORITY: i32 = 20;
    pub const BUILDING_TEXT_PRIORITY: i32 = 30;
}

#[allow(dead_code)]
pub mod zooms {
    pub const SHOW_POLYLINE: u64 = 10;
    pub const SHOW_TEXT: u64 = 10;
}

struct AppState {
    region: Mutex<ShapeFile>,
    water: Mutex<ShapeFile>,
    land: Mutex<ShapeFile>,
    road: Mutex<ShapeFile>,
    building: Mutex<ShapeFile>,
    cache: Mutex<MemoryCache>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { 
            region: Mutex::new(ShapeFile::new()), 
            water: Mutex::new(ShapeFile::new()), 
            land: Mutex::new(ShapeFile::new()),
            road: Mutex::new(ShapeFile::new()), 
            building: Mutex::new(ShapeFile::new()),
            cache: Mutex::new(MemoryCache::new()),
        }
    }

    pub fn load_region(&mut self, file: &str) {
        self.region.get_mut().unwrap().load(file);
    }

    pub fn load_water(&mut self, file: &str) {
        self.water.get_mut().unwrap().load(file);
    }

    pub fn load_land(&mut self, file: &str) {
        self.land.get_mut().unwrap().load(file);
    }

    pub fn load_road(&mut self, file: &str) {
        self.road.get_mut().unwrap().load(file);
    }

    pub fn load_building(&mut self, file: &str) {
        self.building.get_mut().unwrap().load(file);
    }

    pub fn draw_tile(&self, tile: &mut SvgTile, shapes: &ShapeFile, polygon_props: &PolygonProps, polyline_props: &PolylineProps, text_props: &TextProps) {
        for node in shapes.nodes() {
            let rect = Rectangle::new(
                &Point{
                    x: tile.proj().longitude_to_x(node.info.rect.min.x, tile.z() as f64),
                    y: tile.proj().latitude_to_y(node.info.rect.max.y, tile.z() as f64),
                },
                &Point {
                    x: tile.proj().longitude_to_x(node.info.rect.max.x, tile.z() as f64),
                    y: tile.proj().latitude_to_y(node.info.rect.min.y, tile.z() as f64),
                }
            );
    
            if !rect.is_intersect(&tile.rect()) || rect.area() < 10.0 {
                continue;
            }

            match &node.shape {
                Shape::Polyline(polyline) => {
                    if tile.z() > zooms::SHOW_TEXT && !node.info.name.is_empty() {
                        tile.append_text_path(polyline, &node.info.name, polyline_props, text_props);
                    } else if tile.z() > zooms::SHOW_POLYLINE {
                        tile.append_polyline(polyline, polyline_props);
                    }
                },

                Shape::Polygon(polygon) => {
                    tile.append_polygon(polygon, polygon_props);
                    let text_size = SvgTile::text_size(&node.info.name, text_props);
                    if !node.info.name.is_empty() && rect.width() > text_size.width() && rect.height() > text_size.height() {
                        tile.append_text(&node.info.rect.center(), &node.info.name, text_props);
                    }
                },
    
                _ => {},
            }
        }
    }

    
}


#[get("/maps/{z}/{x}/{y}")]
async fn maps(state: web::Data<AppState>, path: web::Path<(u64, u64, u64)>) -> impl Responder {
    let proj = Proj::default();
    let (z, x, y) = path.into_inner();
    let id = format!("tile:{}-{}-{}", z, x, y);
    let mut cache = state.cache.lock().unwrap();
    if let Some(data) = cache.get(&id) {
        HttpResponse::Ok()
            .append_header(("Access-Control-Allow-Origin", "*"))
            .content_type("image/png")
            .body(data.clone())
    } else {
        let mut tile = SvgTile::new(x, y, z, proj);
        state.draw_tile(
            &mut tile, 
            &state.region.lock().unwrap(), 
            &PolygonProps::new(
                colors::REGION_FILL_COLOR, 
                colors::REGION_BORDER_COLOR, 
                3, 
                priorities::REGION_FILL_PRIORITY, 
                priorities::REGION_BORDER_PRIORITY
            ), 
            &PolylineProps::new(
                colors::REGION_BORDER_COLOR, 
                3, 
                priorities::REGION_BORDER_PRIORITY
            ), 
            &TextProps::new(
                colors::REGION_TEXT_COLOR, 
                32, 
                700, 
                priorities::REGION_TEXT_PRIORITY
            )
        );

        state.draw_tile(
            &mut tile, 
            &state.water.lock().unwrap(), 
            &PolygonProps::new(
                colors::WATER_FILL_COLOR, 
                colors::WATER_BORDER_COLOR, 
                1, 
                priorities::WATER_FILL_PRIORITY,
                priorities::WATER_BORDER_PRIORITY
            ), 
            &PolylineProps::new(
                colors::WATER_BORDER_COLOR, 
                1, 
                priorities::WATER_BORDER_PRIORITY
            ), 
            &TextProps::new(
                colors::WATER_TEXT_COLOR, 
                20, 
                700, 
                priorities::WATER_TEXT_PRIORITY
            )
        );
        
        state.draw_tile(
            &mut tile, 
            &state.land.lock().unwrap(), 
            &PolygonProps::new(
                colors::LAND_FILL_COLOR, 
                colors::LAND_BORDER_COLOR, 
                1, 
                priorities::LAND_FILL_PRIORITY,
                priorities::LAND_BORDER_PRIORITY
            ), 
            &PolylineProps::new(
                colors::LAND_BORDER_COLOR, 
                3,
                priorities::LAND_BORDER_PRIORITY
            ), 
            &TextProps::new(
                colors::LAND_TEXT_COLOR, 
                20, 
                700, 
                priorities::LAND_TEXT_PRIORITY
            )
        );

        state.draw_tile(
            &mut tile, 
            &state.road.lock().unwrap(), 
            &PolygonProps::new(
                colors::ROAD_FILL_COLOR, 
                colors::ROAD_BORDER_COLOR, 
                1, 
                priorities::REGION_FILL_PRIORITY, 
                priorities::REGION_BORDER_PRIORITY
            ), 
            &PolylineProps::new(
                colors::ROAD_BORDER_COLOR, 
                3, 
                priorities::ROAD_BORDER_PRIORITY
            ), 
            &TextProps::new(
                colors::ROAD_TEXT_COLOR, 
                20, 
                900, 
                priorities::ROAD_TEXT_PRIORITY
            )
        );

        state.draw_tile(
            &mut tile, 
            &state.building.lock().unwrap(), 
            &PolygonProps::new(
                colors::BUILDING_FILL_COLOR, 
                colors::BUILDING_BORDER_COLOR, 
                1, 
                priorities::BUILDING_FILL_PRIORITY,
                priorities::BUILDING_BORDER_PRIORITY
            ), 
            &PolylineProps::new(
                colors::BUILDING_BORDER_COLOR, 
                3, 
                priorities::BUILDING_BORDER_PRIORITY
            ), 
            &TextProps::new(
                colors::BUILDING_TEXT_COLOR, 
                20, 
                700, 
                priorities::BUILDING_TEXT_PRIORITY
            )
        );

        tile.sort_tags();
        let data = svg_to_png(&tile.dump()).unwrap();
        cache.save(&id, data.clone());
        HttpResponse::Ok()
            .append_header(("Access-Control-Allow-Origin", "*"))
            .content_type("image/png")
            .body(data)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut app_state: AppState = AppState::new();
        app_state.load_region("resource/wuhan/wuhan_region.shp");
        app_state.load_water("resource/wuhan/wuhan_water.shp");
        app_state.load_land("resource/wuhan/wuhan_land.shp");
        app_state.load_road("resource/wuhan/wuhan_road.shp");
        app_state.load_building("resource/wuhan/wuhan_building.shp");

    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(maps)
    })
    .bind(("127.0.0.1", 1995))?
    .run()
    .await
}
