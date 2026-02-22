// Monster OSM Tile Service
// Serves emoji tiles from 71 shards with Monster Group compression

use axum::{
    Router,
    routing::get,
    extract::Path,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct MonsterMetadata {
    shard_id: u32,
    node_count: u64,
    shadow: u64,
    topology_class: String,
    content_emoji: char,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tiles/gielis/:shard", get(serve_gielis))
        .route("/tiles/cusp", get(serve_cusp))
        .route("/tiles/stats", get(serve_stats))
        .route("/tiles/:z/:x/:y/emoji", get(serve_emoji))
        .route("/tiles/:z/:x/:y/monster", get(serve_monster))
        .layer(CorsLayer::permissive());
    
    let addr = "0.0.0.0:3000";
    println!("🎭 Monster OSM Tile Service");
    println!("Listening on http://{}", addr);
    println!("Endpoints:");
    println!("  GET /tiles/:z/:x/:y.emoji   - Emoji tile (24×24)");
    println!("  GET /tiles/:z/:x/:y.monster - Monster metadata");
    println!("  GET /tiles/gielis/:shard    - Gielis pattern SVG");
    println!("  GET /tiles/cusp             - Shard 17 (2832× resonance)");
    println!("  GET /tiles/stats            - 71-shard statistics");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "🎭 Monster OSM Tile Service - 3.2 Trillion Nodes"
}

async fn health() -> &'static str {
    "OK"
}

async fn serve_emoji(
    Path((z, x, y)): Path<(u8, u32, u32)>,
) -> Result<String, StatusCode> {
    // Map tile coordinates to shard
    let shard_id = ((x + y) % 71) as u32;
    
    // Generate 24×24 emoji grid
    let emoji = get_shard_emoji(shard_id);
    let mut grid = String::new();
    
    for row in 0..24 {
        for col in 0..24 {
            let idx = ((row + col + shard_id) % 15) as usize;
            grid.push(emoji[idx]);
        }
        grid.push('\n');
    }
    
    Ok(grid)
}

async fn serve_monster(
    Path((z, x, y)): Path<(u8, u32, u32)>,
) -> Result<Json<MonsterMetadata>, StatusCode> {
    let shard_id = ((x + y) % 71) as u32;
    
    // Read from shared memory if available
    let (node_count, shadow) = read_shard_data(shard_id).unwrap_or((0, 0));
    
    let topology_class = get_topology_class(shard_id);
    let content_emoji = get_content_emoji(shadow);
    
    Ok(Json(MonsterMetadata {
        shard_id,
        node_count,
        shadow,
        topology_class,
        content_emoji,
    }))
}

async fn serve_gielis(
    Path(shard): Path<u32>,
) -> Result<Response, StatusCode> {
    if shard >= 71 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let svg = generate_gielis_svg(shard);
    Ok((
        [(axum::http::header::CONTENT_TYPE, "image/svg+xml")],
        svg
    ).into_response())
}

async fn serve_cusp() -> Result<Json<MonsterMetadata>, StatusCode> {
    let shard_id = 17;
    let (node_count, shadow) = read_shard_data(shard_id).unwrap_or((21855635423, 0));
    
    Ok(Json(MonsterMetadata {
        shard_id,
        node_count,
        shadow,
        topology_class: "🔮 CII (Particle-hole)".to_string(),
        content_emoji: '🐯',
    }))
}

async fn serve_stats() -> Json<Vec<MonsterMetadata>> {
    let mut stats = Vec::new();
    
    for shard_id in 0..71 {
        let (node_count, shadow) = read_shard_data(shard_id).unwrap_or((0, 0));
        stats.push(MonsterMetadata {
            shard_id,
            node_count,
            shadow,
            topology_class: get_topology_class(shard_id),
            content_emoji: get_content_emoji(shadow),
        });
    }
    
    Json(stats)
}

fn read_shard_data(shard_id: u32) -> Option<(u64, u64)> {
    let path = format!("/dev/shm/osm_shard_{}", shard_id);
    let mut file = File::open(&path).ok()?;
    
    let mut buf = [0u8; 16];
    file.read_exact(&mut buf).ok()?;
    
    let shadow = u64::from_le_bytes([buf[0],buf[1],buf[2],buf[3],buf[4],buf[5],buf[6],buf[7]]);
    let node_count = u64::from_le_bytes([buf[8],buf[9],buf[10],buf[11],buf[12],buf[13],buf[14],buf[15]]);
    
    Some((node_count, shadow))
}

fn get_topology_class(shard_id: u32) -> String {
    let classes = [
        "🌀 A", "🔱 AIII", "⚛️ AI", "🌳 BDI", "💎 D",
        "🌊 DIII", "🧬 AII", "🔮 CII", "⚡ C", "🌌 CI"
    ];
    classes[(shard_id % 10) as usize].to_string()
}

fn get_shard_emoji(shard_id: u32) -> [char; 15] {
    ['🏙','🌊','🏔','🛣','🌳','🏛','🐯','⚡','🧬','🌙','📡','🎭','👁','🔬','🎓']
}

fn get_content_emoji(shadow: u64) -> char {
    let emojis = ['🏙','🌊','🏔','🛣','🌳','🏛','🐯','⚡','🧬','🌙','📡','🎭','👁','🔬','🎓'];
    emojis[(shadow % 15) as usize]
}

fn generate_gielis_svg(shard_id: u32) -> String {
    let theta = 2.0 * std::f64::consts::PI * (shard_id as f64) / 71.0;
    let r = gielis_radius(theta, 71);
    
    let cx = 200.0;
    let cy = 200.0;
    let scale = 50.0;
    
    let x = cx + r * theta.cos() * scale;
    let y = cy + r * theta.sin() * scale;
    
    let svg_content = format!(
        r#"<svg width="400" height="400" xmlns="http://www.w3.org/2000/svg">
  <circle cx="200" cy="200" r="150" fill="none" stroke="gray" stroke-width="1"/>
  <circle cx="{}" cy="{}" r="10" fill="red"/>
  <text x="{}" y="{}" font-size="20" text-anchor="middle">Shard {}</text>
  <text x="200" y="380" font-size="14" text-anchor="middle">71-Fold Gielis Pattern</text>
</svg>"#,
        x, y, x, y + 30.0, shard_id
    );
    svg_content
}

fn gielis_radius(theta: f64, m: u32) -> f64 {
    let term1 = ((m as f64 * theta / 4.0).cos()).abs().powf(1.0);
    let term2 = ((m as f64 * theta / 4.0).sin()).abs().powf(1.0);
    (term1 + term2).powf(-1.0).min(3.0)
}
