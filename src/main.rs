mod shard;
mod userdir;
mod wikidata;
mod monster;
mod crawler;
mod download;
mod stream;
mod piece_download;
mod piece_index;

use lava_torrent::torrent::v1::Torrent;
use reqwest;
use std::fs::File;
use std::io::Write;
use std::env;
use tokio;
use userdir::load_user_locations;
use piece_index::{build_index_from_torrent, save_sharded_index};

const OSM_TORRENT_URL: &str = "https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf.torrent";

fn location_to_piece_index(lat: f64, lon: f64, num_pieces: usize) -> usize {
    // Map lat/lon to piece index using Monster primes
    // Lat: -90 to 90 → 0 to 1
    // Lon: -180 to 180 → 0 to 1
    let lat_norm = (lat + 90.0) / 180.0;
    let lon_norm = (lon + 180.0) / 360.0;
    
    // Combine using golden ratio (phi) for better distribution
    let phi = 1.618033988749895;
    let combined = (lat_norm * phi + lon_norm) % 1.0;
    
    // Map to piece index (mod 71 for Monster prime)
    let piece_idx = (combined * num_pieces as f64) as usize;
    piece_idx % num_pieces
}

fn pieces_in_radius(lat: f64, lon: f64, radius_miles: f64, num_pieces: usize, piece_length: i64) -> Vec<usize> {
    // 1 degree lat ≈ 69 miles, 1 degree lon ≈ 69 * cos(lat) miles
    let lat_deg_per_mile = 1.0 / 69.0;
    let lon_deg_per_mile = 1.0 / (69.0 * lat.to_radians().cos());
    
    let lat_radius = radius_miles * lat_deg_per_mile;
    let lon_radius = radius_miles * lon_deg_per_mile;
    
    // Sample grid points in radius
    let mut pieces = std::collections::HashSet::new();
    let steps = 20; // Grid resolution
    
    for i in 0..steps {
        for j in 0..steps {
            let dlat = (i as f64 / steps as f64 - 0.5) * 2.0 * lat_radius;
            let dlon = (j as f64 / steps as f64 - 0.5) * 2.0 * lon_radius;
            
            let sample_lat = lat + dlat;
            let sample_lon = lon + dlon;
            
            let piece = location_to_piece_index(sample_lat, sample_lon, num_pieces);
            pieces.insert(piece);
        }
    }
    
    pieces.into_iter().collect()
}

async fn index_torrent_by_location() -> Result<(), Box<dyn std::error::Error>> {
    // Load user locations (default: ramanujan)
    let user = env::args().nth(1).unwrap_or_else(|| "ramanujan".to_string());
    let user_locs = load_user_locations(&user)?;
    
    println!("🌍 Downloading OSM Planet Torrent...");
    println!("👤 User: {} (Wikidata: {})", 
        user_locs.user, 
        user_locs.wikidata_user.as_deref().unwrap_or("N/A")
    );
    
    let response = reqwest::get(OSM_TORRENT_URL).await?;
    let bytes = response.bytes().await?;
    
    let output_file = "osm-planet.torrent";
    let mut file = File::create(output_file)?;
    file.write_all(&bytes)?;
    println!("✓ Downloaded {} bytes to {}", bytes.len(), output_file);
    
    // Parse torrent
    let torrent = Torrent::read_from_bytes(&bytes)?;
    println!("\n📊 Torrent Info:");
    println!("  Name: {}", torrent.name);
    println!("  Pieces: {}", torrent.pieces.len());
    println!("  Piece length: {} bytes", torrent.piece_length);
    println!("  Total size: {} GB", torrent.length / (1024 * 1024 * 1024));
    
    // Phase 1: Build piece index
    println!("\n📊 Phase 1: Building piece index...");
    let piece_index = build_index_from_torrent(output_file)?;
    
    // Save sharded index
    save_sharded_index(piece_index, "index")?;
    
    println!("\n✓ Index complete! Shards saved to index/");
    println!("   Share this index so others can skip Phase 1");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    index_torrent_by_location().await?;
    Ok(())
}

