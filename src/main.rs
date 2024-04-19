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
use tile::{Tile, SvgTile};

use crate::{geometry::Rectangle, tile::{tile::svg_to_png, Proj}};

struct AppState {
    border: Mutex<ShapeFile>,
    water: Mutex<ShapeFile>,
    land: Mutex<ShapeFile>,
    road: Mutex<ShapeFile>,
    building: Mutex<ShapeFile>,
    cache: Mutex<MemoryCache>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { 
            border: Mutex::new(ShapeFile::new()), 
            water: Mutex::new(ShapeFile::new()), 
            land: Mutex::new(ShapeFile::new()),
            road: Mutex::new(ShapeFile::new()), 
            building: Mutex::new(ShapeFile::new()),
            cache: Mutex::new(MemoryCache::new()),
        }
    }

    pub fn load_border(&mut self, file: &str) {
        self.border.get_mut().unwrap().load(file);
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

    pub fn draw_tile(&self, tile: &mut SvgTile, shapes: &ShapeFile, fill_color: &str, line_color: &str, line_width: usize) {
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
                    if tile.z() > 12 && !node.info.name.is_empty() {
                        tile.append_text_path(polyline, &node.info.name, line_color, line_width);
                    } else {
                        tile.append_polyline(polyline, line_color, line_width);
                    }
                },
    
                Shape::Polygon(polygon) => {
                    tile.append_polygon(polygon, fill_color, line_color, line_width);

                    if tile.z() > 17 && !node.info.name.is_empty() {
                        let point = polygon.point(0);
                        tile.append_text(point, &node.info.name);
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
        HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).content_type("image/png").body(data.clone())
    } else {
        let mut tile = SvgTile::new(x, y, z, proj);

        state.draw_tile(&mut tile, &state.border.lock().unwrap(), "#4caf50", "#33eb91", 3);
        state.draw_tile(&mut tile, &state.water.lock().unwrap(), "#33eaff", "#33eaff", 1);
        state.draw_tile(&mut tile, &state.land.lock().unwrap(), "#aaaaaa", "#aaaaaa", 1);
        state.draw_tile(&mut tile, &state.road.lock().unwrap(), "#ff9100", "#ff9100", 3);
        state.draw_tile(&mut tile, &state.building.lock().unwrap(), "#5393ff", "#2196f3", 1);

        tile.sort_tags();
        let data = svg_to_png(&tile.dump()).unwrap();
        cache.save(&id, data.clone());
        HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).content_type("image/png").body(data)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut app_state: AppState = AppState::new();
        app_state.load_border("resource/wuhan/wuhan_border.shp");
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
