mod shard;
mod userdir;
mod wikidata;

use lava_torrent::torrent::v1::Torrent;
use reqwest;
use std::fs::File;
use std::io::Write;
use std::env;
use tokio;
use userdir::load_user_locations;
use wikidata::{query_wikidata, get_linked_entities};

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
    
    // Query Wikidata for each location
    println!("\n🌐 Querying Wikidata...");
    let mut wikidata_results = File::create(format!("{}-wikidata.json", user_locs.user))?;
    writeln!(wikidata_results, "{{")?;
    writeln!(wikidata_results, r#"  "user": "{}","#, user_locs.user)?;
    writeln!(wikidata_results, r#"  "queries": ["#)?;
    
    for (i, loc) in user_locs.locations.iter().enumerate() {
        if let Some(qid) = &loc.wikidata {
            println!("  Querying {} ({})...", loc.name, qid);
            
            match query_wikidata(qid).await {
                Ok(data) => {
                    writeln!(wikidata_results, "    {{")?;
                    writeln!(wikidata_results, r#"      "location": "{}","#, loc.name)?;
                    writeln!(wikidata_results, r#"      "qid": "{}","#, qid)?;
                    writeln!(wikidata_results, r#"      "data": {}"#, serde_json::to_string_pretty(&data)?)?;
                    
                    // Get linked entities
                    if let Ok(linked) = get_linked_entities(qid).await {
                        println!("    → {} linked entities", linked.len());
                        writeln!(wikidata_results, r#"      ,"linked": {}"#, serde_json::to_string(&linked)?)?;
                    }
                    
                    write!(wikidata_results, "    }}")?;
                }
                Err(e) => {
                    println!("    ✗ Error: {}", e);
                }
            }
            
            if i < user_locs.locations.len() - 1 {
                writeln!(wikidata_results, ",")?;
            } else {
                writeln!(wikidata_results)?;
            }
        }
    }
    
    writeln!(wikidata_results, "  ]")?;
    writeln!(wikidata_results, "}}")?;
    
    println!("\n✓ Wikidata results saved to {}-wikidata.json", user_locs.user);
    
    // Index locations to pieces
    println!("\n🗺️ {} Journey → Torrent Pieces:", user_locs.user);
    let mut location_index = File::create(format!("{}-location-index.json", user_locs.user))?;
    writeln!(location_index, "{{")?;
    writeln!(location_index, r#"  "user": "{}","#, user_locs.user)?;
    if let Some(wd) = &user_locs.wikidata_user {
        writeln!(location_index, r#"  "wikidata_user": "{}","#, wd)?;
    }
    writeln!(location_index, r#"  "torrent": "{}","#, torrent.name)?;
    writeln!(location_index, r#"  "pieces": {},"#, torrent.pieces.len())?;
    writeln!(location_index, r#"  "locations": ["#)?;
    
    for (i, loc) in user_locs.locations.iter().enumerate() {
        let piece_idx = location_to_piece_index(loc.lat, loc.lon, torrent.pieces.len());
        let radius_pieces = pieces_in_radius(loc.lat, loc.lon, loc.radius_miles, torrent.pieces.len(), torrent.piece_length);
        let shard = piece_idx % 71; // Monster prime
        
        println!("  {} ({:.4}, {:.4})", loc.name, loc.lat, loc.lon);
        if let Some(wd) = &loc.wikidata {
            println!("    Wikidata: {}", wd);
        }
        println!("    → Center Piece: {}", piece_idx);
        println!("    → {}mi Radius: {} pieces ({} MB)", loc.radius_miles, radius_pieces.len(), radius_pieces.len() * 4);
        println!("    → Shard: {} (mod 71)", shard);
        
        writeln!(location_index, "    {{")?;
        writeln!(location_index, r#"      "name": "{}","#, loc.name)?;
        writeln!(location_index, r#"      "lat": {},"#, loc.lat)?;
        writeln!(location_index, r#"      "lon": {},"#, loc.lon)?;
        if let Some(wd) = &loc.wikidata {
            writeln!(location_index, r#"      "wikidata": "{}","#, wd)?;
        }
        writeln!(location_index, r#"      "piece": {},"#, piece_idx)?;
        writeln!(location_index, r#"      "shard": {}"#, shard)?;
        write!(location_index, "    }}")?;
        if i < user_locs.locations.len() - 1 {
            writeln!(location_index, ",")?;
        } else {
            writeln!(location_index)?;
        }
    }
    
    writeln!(location_index, "  ]")?;
    writeln!(location_index, "}}")?;
    
    println!("\n✓ Index saved to {}-location-index.json", user_locs.user);
    println!("\n∴ Now fetch only needed pieces! 🕉️→🎓");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    index_torrent_by_location().await?;
    Ok(())
}

