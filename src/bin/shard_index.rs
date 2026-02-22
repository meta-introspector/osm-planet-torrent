// Shard the spatial index using 2^46 binary subdivision
// Implements Z-order curve (Morton code) for spatial locality
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Deserialize)]
struct PieceIndex {
    piece_id: u32,
    byte_offset: u64,
    min_node_id: u64,
    max_node_id: u64,
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    node_count: usize,
    wikidata_count: usize,
    monster_lat_block: u8,
    monster_lon_block: u8,
}

#[derive(Debug, Serialize)]
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

// Calculate Z-order (Morton code) for spatial indexing
fn morton_encode(lat: f64, lon: f64, bits: u8) -> u64 {
    // Normalize to [0, 1]
    let lat_norm = (lat + 90.0) / 180.0;
    let lon_norm = (lon + 180.0) / 360.0;
    
    // Convert to integer coordinates
    let max_val = (1u64 << bits) - 1;
    let lat_int = (lat_norm * max_val as f64).clamp(0.0, max_val as f64) as u64;
    let lon_int = (lon_norm * max_val as f64).clamp(0.0, max_val as f64) as u64;
    
    // Interleave bits
    let mut code = 0u64;
    for i in 0..bits {
        code |= ((lat_int >> i) & 1) << (2 * i);
        code |= ((lon_int >> i) & 1) << (2 * i + 1);
    }
    
    code
}

// Determine appropriate level based on piece size
fn determine_level(piece: &PieceIndex) -> u8 {
    let lat_span = piece.max_lat - piece.min_lat;
    let lon_span = piece.max_lon - piece.min_lon;
    let area = lat_span * lon_span;
    
    // Monster prime levels:
    // 0: 2^46 - point-like (< 0.0001°²)
    // 1: 3^20 - small (< 0.01°²)
    // 2: 5^9 - medium (< 1°²)
    // 3: 7^6 - large (< 10°²)
    // 4: 11^2 - very large (>= 10°²)
    
    if area < 0.0001 {
        0
    } else if area < 0.01 {
        1
    } else if area < 1.0 {
        2
    } else if area < 10.0 {
        3
    } else {
        4
    }
}

// Calculate which shards a piece overlaps
fn calculate_shards(piece: &PieceIndex, level: u8) -> Vec<u64> {
    let bits = match level {
        0 => 23,  // 2^46 = 2^(23*2)
        1 => 10,  // 3^20 ≈ 2^20
        2 => 10,  // 5^9 ≈ 2^20
        3 => 8,   // 7^6 ≈ 2^16
        _ => 6,   // 11^2 ≈ 2^12
    };
    
    // Calculate corners
    let min_code = morton_encode(piece.min_lat, piece.min_lon, bits);
    let max_code = morton_encode(piece.max_lat, piece.max_lon, bits);
    
    // If same shard, return single shard
    if min_code == max_code {
        return vec![min_code];
    }
    
    // Otherwise, return all overlapping shards
    // For simplicity, return min and max (boundary case)
    vec![min_code, max_code]
}

fn main() -> Result<()> {
    let index_file = "complete_spatial_index.jsonl";
    let output_dir = "shards";
    
    println!("🗺️  Sharding spatial index with 2^46 binary subdivision");
    println!("📦 Input: {}", index_file);
    println!("💾 Output: {}/", output_dir);
    println!();
    
    // Create output directories
    for level in 0..=4 {
        fs::create_dir_all(format!("{}/level_{}", output_dir, level))?;
    }
    fs::create_dir_all(format!("{}/boundary_arrows", output_dir))?;
    
    // Read index and group by shards
    let file = File::open(index_file)?;
    let reader = BufReader::new(file);
    
    let mut shards: HashMap<(u8, u64), Shard> = HashMap::new();
    let mut boundary_count = 0;
    let mut total_pieces = 0;
    
    for line in reader.lines() {
        let line = line?;
        let piece: PieceIndex = serde_json::from_str(&line)?;
        total_pieces += 1;
        
        let level = determine_level(&piece);
        let shard_ids = calculate_shards(&piece, level);
        let is_boundary = shard_ids.len() > 1;
        
        if is_boundary {
            boundary_count += 1;
        }
        
        for shard_id in shard_ids {
            let shard = shards.entry((level, shard_id)).or_insert_with(|| Shard {
                level,
                shard_id,
                piece_ids: Vec::new(),
                min_lat: f64::MAX,
                max_lat: f64::MIN,
                min_lon: f64::MAX,
                max_lon: f64::MIN,
                node_count: 0,
                wikidata_count: 0,
                is_boundary,
            });
            
            shard.piece_ids.push(piece.piece_id);
            shard.min_lat = shard.min_lat.min(piece.min_lat);
            shard.max_lat = shard.max_lat.max(piece.max_lat);
            shard.min_lon = shard.min_lon.min(piece.min_lon);
            shard.max_lon = shard.max_lon.max(piece.max_lon);
            shard.node_count += piece.node_count;
            shard.wikidata_count += piece.wikidata_count;
        }
        
        if total_pieces % 1000 == 0 {
            println!("📊 Processed {} pieces, {} shards, {} boundary", 
                total_pieces, shards.len(), boundary_count);
        }
    }
    
    println!();
    println!("✅ Processed {} pieces into {} shards", total_pieces, shards.len());
    println!("📊 Boundary pieces: {}", boundary_count);
    println!();
    
    // Write shards to files
    let mut level_counts = [0; 5];
    
    for ((level, shard_id), shard) in shards {
        let shard_file = format!("{}/level_{}/shard_{:016x}.json", 
            output_dir, level, shard_id);
        let json = serde_json::to_string_pretty(&shard)?;
        fs::write(shard_file, json)?;
        level_counts[level as usize] += 1;
    }
    
    println!("📊 Shards per level:");
    let level_names = ["2^46 (binary)", "3^20 (ternary)", "5^9 (quinary)", 
                       "7^6 (septenary)", "11^2 (base-11)"];
    for (level, count) in level_counts.iter().enumerate() {
        if *count > 0 {
            println!("   Level {}: {} shards ({})", level, count, level_names[level]);
        }
    }
    
    println!();
    println!("✅ Sharding complete!");
    println!("💾 Shards saved to: {}/", output_dir);
    
    Ok(())
}
