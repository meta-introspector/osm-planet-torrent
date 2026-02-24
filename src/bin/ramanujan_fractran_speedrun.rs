// Ramanujan's Life as FRACTRAN Speedrun
// His journey through OSM space encoded in Monster primes

use std::fs::File;
use std::io::{Read, Seek, Write};
use serde_json::json;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

// Ramanujan's life locations from ramanujan-location-index.json
const RAMANUJAN_JOURNEY: [(&str, f64, f64, u32, u32); 8] = [
    ("Kumbakonam_Birth", 10.9617, 79.3881, 13668, 36),
    ("Ramanujan_House", 10.9617, 79.3881, 13668, 36),
    ("Namagiri_Temple", 11.2189, 78.1677, 13645, 13),
    ("Chennai_College", 13.0827, 80.2707, 14137, 8),
    ("Ramanujan_Museum", 13.0827, 80.2707, 14137, 8),
    ("London_Hardy", 51.5074, -0.1278, 16793, 37),
    ("Cambridge_Trinity", 52.2053, 0.1218, 16945, 47),
    ("Trinity_College", 52.2067, 0.1165, 16945, 47),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    
    eprintln!("🎭 Ramanujan's Life as FRACTRAN Speedrun");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let mut file = File::open(planet_file)?;
    
    let mut journey_states = Vec::new();
    let mut total_fractran: u128 = 1;
    
    for (name, lat, lon, piece, shard) in &RAMANUJAN_JOURNEY {
        eprintln!("\n📍 {} ({}, {})", name, lat, lon);
        eprintln!("   Piece: {}, Shard: {}", piece, shard);
        
        // Extract piece
        let piece_size = 4 * 1024 * 1024;
        let offset = *piece as u64 * piece_size;
        
        file.seek(std::io::SeekFrom::Start(offset))?;
        let mut data = vec![0u8; piece_size as usize];
        let bytes_read = file.read(&mut data)?;
        data.truncate(bytes_read);
        
        // Encode to FRACTRAN
        let fractran_state = encode_location(&data, *lat, *lon, *shard);
        eprintln!("   FRACTRAN: {} (log10: {:.2})", 
            fractran_state, (fractran_state as f64).log10());
        
        // Accumulate journey
        total_fractran = total_fractran.wrapping_mul(fractran_state % (u64::MAX as u128)) as u128;
        
        journey_states.push(json!({
            "location": name,
            "lat": lat,
            "lon": lon,
            "piece": piece,
            "shard": shard,
            "fractran_state": fractran_state.to_string(),
            "monster_prime": MONSTER_PRIMES[*shard as usize % 15],
        }));
    }
    
    let final_shard = (total_fractran % 71) as u64;
    
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🎯 Journey complete!");
    eprintln!("   Total FRACTRAN: {}", total_fractran);
    eprintln!("   Final shard: {} (Ramanujan's number: 1729 mod 71 = {})", 
        final_shard, 1729 % 71);
    eprintln!("   Time: {}ms", start.elapsed().as_millis());
    
    // Create speedrun output
    let output = json!({
        "type": "RamanujanFractranSpeedrun",
        "mathematician": "Srinivasa Ramanujan",
        "birth": "1887-12-22",
        "death": "1920-04-26",
        "journey": journey_states,
        "total_fractran_state": total_fractran.to_string(),
        "final_shard": final_shard,
        "ramanujan_number": 1729,
        "ramanujan_mod_71": 1729 % 71,
        "speedrun_ms": start.elapsed().as_millis(),
        "semantic_encoding": {
            "birth_to_death": format!("2^{} × 71^{}", 1887, 1920),
            "taxi_number": "1729 = 1³ + 12³ = 9³ + 10³",
            "monster_connection": "71 | (Monster order / 808017424794512875886459904961710757005754368000000000)",
        }
    });
    
    let mut out = File::create("/tmp/ramanujan_fractran_speedrun.json")?;
    serde_json::to_writer_pretty(&mut out, &output)?;
    
    eprintln!("\n✅ /tmp/ramanujan_fractran_speedrun.json");
    Ok(())
}

fn encode_location(data: &[u8], lat: f64, lon: f64, shard: u32) -> u128 {
    let mut state: u128 = 2;
    
    // Encode lat/lon as prime powers
    let lat_int = ((lat + 90.0) * 1000000.0) as u64;
    let lon_int = ((lon + 180.0) * 1000000.0) as u64;
    
    state *= MONSTER_PRIMES[0].pow((lat_int % 8) as u32) as u128;
    state *= MONSTER_PRIMES[1].pow((lon_int % 8) as u32) as u128;
    state *= MONSTER_PRIMES[2].pow(shard) as u128;
    
    // Add data signature
    let mut count = 3;
    for &byte in data.iter().take(100) {
        if byte > 0 && count < 15 {
            state *= MONSTER_PRIMES[count].pow((byte % 4) as u32) as u128;
            count += 1;
        }
    }
    
    state
}
