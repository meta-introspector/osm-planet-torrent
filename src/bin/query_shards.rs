// Query sharded spatial index to find pieces containing a location
use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Shard {
    level: u8,
    shard_id: u64,
    piece_ids: Vec<u32>,
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    node_count: usize,
    wikidata_count: usize,
    is_boundary: bool,
}

fn morton_encode(lat: f64, lon: f64, bits: u8) -> u64 {
    let lat_norm = (lat + 90.0) / 180.0;
    let lon_norm = (lon + 180.0) / 360.0;
    
    let max_val = (1u64 << bits) - 1;
    let lat_int = (lat_norm * max_val as f64).clamp(0.0, max_val as f64) as u64;
    let lon_int = (lon_norm * max_val as f64).clamp(0.0, max_val as f64) as u64;
    
    let mut code = 0u64;
    for i in 0..bits {
        code |= ((lat_int >> i) & 1) << (2 * i);
        code |= ((lon_int >> i) & 1) << (2 * i + 1);
    }
    
    code
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <lat> <lon>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} 10.9617 79.3881  # Kumbakonam, India", args[0]);
        std::process::exit(1);
    }
    
    let lat: f64 = args[1].parse()?;
    let lon: f64 = args[2].parse()?;
    
    println!("🎯 Querying location: {}, {}", lat, lon);
    println!();
    
    // Try each level from finest to coarsest
    for level in 0..=4 {
        let bits = match level {
            0 => 23,
            1 => 10,
            2 => 10,
            3 => 8,
            _ => 6,
        };
        
        let shard_id = morton_encode(lat, lon, bits);
        let shard_file = format!("shards/level_{}/shard_{:016x}.json", level, shard_id);
        
        if let Ok(data) = fs::read_to_string(&shard_file) {
            let shard: Shard = serde_json::from_str(&data)?;
            
            // Check if location is actually in this shard
            if lat >= shard.min_lat && lat <= shard.max_lat &&
               lon >= shard.min_lon && lon <= shard.max_lon {
                
                let level_names = ["2^46 (binary)", "3^20 (ternary)", "5^9 (quinary)", 
                                   "7^6 (septenary)", "11^2 (base-11)"];
                
                println!("✓ Found in Level {} - {}", level, level_names[level]);
                println!("  Shard ID: 0x{:016x}", shard.shard_id);
                println!("  BBox: lat [{:.4}, {:.4}], lon [{:.4}, {:.4}]",
                    shard.min_lat, shard.max_lat, shard.min_lon, shard.max_lon);
                println!("  Pieces: {} pieces", shard.piece_ids.len());
                println!("  Nodes: {}", shard.node_count);
                println!("  Wikidata: {}", shard.wikidata_count);
                println!("  Boundary: {}", if shard.is_boundary { "yes" } else { "no" });
                println!();
                println!("📦 Piece IDs to download:");
                for (i, piece_id) in shard.piece_ids.iter().enumerate() {
                    if i < 20 {
                        println!("  - Piece {}", piece_id);
                    } else if i == 20 {
                        println!("  ... and {} more", shard.piece_ids.len() - 20);
                        break;
                    }
                }
                
                return Ok(());
            }
        }
    }
    
    println!("✗ No shard found for this location");
    println!("  This might be in an ocean or unpopulated area");
    
    Ok(())
}
