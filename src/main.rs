mod shard;

use lava_torrent::torrent::v1::Torrent;
use reqwest;
use std::fs::File;
use std::io::Write;
use tokio;

const OSM_TORRENT_URL: &str = "https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf.torrent";

// Ramanujan journey locations
struct Location {
    name: &'static str,
    lat: f64,
    lon: f64,
}

const LOCATIONS: &[Location] = &[
    Location { name: "Kumbakonam", lat: 10.9617, lon: 79.3881 },
    Location { name: "Chennai", lat: 13.0827, lon: 80.2707 },
    Location { name: "London", lat: 51.5074, lon: -0.1278 },
    Location { name: "Cambridge", lat: 52.2053, lon: 0.1218 },
    Location { name: "Trinity_College", lat: 52.2067, lon: 0.1165 },
];

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

async fn index_torrent_by_location() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Downloading OSM Planet Torrent...");
    
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
    
    // Index locations to pieces
    println!("\n🗺️ Ramanujan Journey → Torrent Pieces:");
    let mut location_index = File::create("ramanujan-location-index.json")?;
    writeln!(location_index, "{{")?;
    writeln!(location_index, r#"  "torrent": "{}","#, torrent.name)?;
    writeln!(location_index, r#"  "pieces": {},"#, torrent.pieces.len())?;
    writeln!(location_index, r#"  "locations": ["#)?;
    
    for (i, loc) in LOCATIONS.iter().enumerate() {
        let piece_idx = location_to_piece_index(loc.lat, loc.lon, torrent.pieces.len());
        let shard = piece_idx % 71; // Monster prime
        
        println!("  {} ({:.4}, {:.4})", loc.name, loc.lat, loc.lon);
        println!("    → Piece: {}", piece_idx);
        println!("    → Shard: {} (mod 71)", shard);
        
        writeln!(location_index, "    {{")?;
        writeln!(location_index, r#"      "name": "{}","#, loc.name)?;
        writeln!(location_index, r#"      "lat": {},"#, loc.lat)?;
        writeln!(location_index, r#"      "lon": {},"#, loc.lon)?;
        writeln!(location_index, r#"      "piece": {},"#, piece_idx)?;
        writeln!(location_index, r#"      "shard": {}"#, shard)?;
        write!(location_index, "    }}")?;
        if i < LOCATIONS.len() - 1 {
            writeln!(location_index, ",")?;
        } else {
            writeln!(location_index)?;
        }
    }
    
    writeln!(location_index, "  ]")?;
    writeln!(location_index, "}}")?;
    
    println!("\n✓ Index saved to ramanujan-location-index.json");
    println!("\n∴ Now fetch only needed pieces! 🕉️→🎓");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    index_torrent_by_location().await?;
    Ok(())
}

