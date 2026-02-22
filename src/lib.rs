use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentPiece {
    pub piece_id: u32,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub shard: u32,
}

#[wasm_bindgen]
pub fn calculate_shard(piece_id: u32) -> u32 {
    piece_id % 196883  // 71 * 59 * 47
}

#[wasm_bindgen]
pub fn calculate_tile(lat: f64, lon: f64) -> String {
    let tile_lat = (((lat + 90.0) * 256.0 / 180.0) as u32) % 71;
    let tile_lon = (((lon + 180.0) * 256.0 / 360.0) as u32) % 59;
    format!("tile_{}_{}", tile_lat, tile_lon)
}

#[wasm_bindgen]
pub fn query_location(lat: f64, lon: f64, name: String) -> String {
    let tile = calculate_tile(lat, lon);
    let piece = ((lat + 90.0) * 100.0) as u32;
    let shard = calculate_shard(piece);
    
    format!(
        "{{\"name\":\"{}\",\"lat\":{},\"lon\":{},\"tile\":\"{}\",\"piece\":{},\"shard\":{}}}",
        name, lat, lon, tile, piece, shard
    )
}

#[wasm_bindgen]
pub fn reduction_percent(fetched_mb: f64, total_gb: f64) -> f64 {
    let total_mb = total_gb * 1024.0;
    ((total_mb - fetched_mb) / total_mb) * 100.0
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
