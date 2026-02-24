// Fetch OSM data using ramanujan-location-index.json
use std::fs;
use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: fetch-from-index <location_name>");
        eprintln!("Example: fetch-from-index Kumbakonam");
        eprintln!();
        eprintln!("Available locations:");
        list_locations()?;
        std::process::exit(1);
    }
    
    let location_name = &args[1];
    
    // Load index
    let data = fs::read_to_string("ramanujan-location-index.json")?;
    let json: Value = serde_json::from_str(&data)?;
    
    // Find location
    let locations = json["locations"].as_array().ok_or("No locations")?;
    let location = locations.iter()
        .find(|loc| loc["name"].as_str().unwrap_or("").to_lowercase().contains(&location_name.to_lowercase()))
        .ok_or(format!("Location '{}' not found", location_name))?;
    
    let name = location["name"].as_str().unwrap_or("Unknown");
    let lat = location["lat"].as_f64().unwrap_or(0.0);
    let lon = location["lon"].as_f64().unwrap_or(0.0);
    let piece = location["piece"].as_u64().unwrap_or(0);
    let shard = location["shard"].as_u64().unwrap_or(0);
    let wikidata = location["wikidata"].as_str().unwrap_or("");
    
    println!("📍 Location: {}", name);
    println!("   Coordinates: [{}, {}]", lat, lon);
    println!("   Wikidata: {}", wikidata);
    println!("   Piece: {}", piece);
    println!("   Shard: {} (mod 196,883)", shard);
    println!();
    println!("🔽 To fetch this piece:");
    println!("   cargo run --bin fetch-piece {}", piece);
    println!();
    println!("🌐 Archive.org:");
    println!("   https://archive.org/download/osm-planet-ramanujan-monster/");
    
    Ok(())
}

fn list_locations() -> anyhow::Result<()> {
    let data = fs::read_to_string("ramanujan-location-index.json")?;
    let json: Value = serde_json::from_str(&data)?;
    let locations = json["locations"].as_array().ok_or("No locations")?;
    
    for loc in locations {
        let name = loc["name"].as_str().unwrap_or("Unknown");
        let piece = loc["piece"].as_u64().unwrap_or(0);
        println!("  - {} (piece {})", name, piece);
    }
    
    Ok(())
}
