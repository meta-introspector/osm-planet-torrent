// Subdivide each geographic bucket using Monster Group prime powers
// 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

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
struct MonsterShard {
    bucket_id: String,      // "lat_lon_size"
    prime: u8,              // Which prime
    power: u8,              // Power of prime (e.g., 2^46, 3^20, etc.)
    shard_id: u64,          // Shard ID within this prime^power subdivision
    piece_ids: Vec<u32>,
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    node_count: usize,
    wikidata_count: usize,
}

// Monster Group prime powers
const MONSTER_PRIMES: [(u8, u8); 15] = [
    (2, 46),   // 2^46
    (3, 20),   // 3^20
    (5, 9),    // 5^9
    (7, 6),    // 7^6
    (11, 2),   // 11^2
    (13, 3),   // 13^3
    (17, 1),   // 17^1
    (19, 1),   // 19^1
    (23, 1),   // 23^1
    (29, 1),   // 29^1
    (31, 1),   // 31^1
    (41, 1),   // 41^1
    (47, 1),   // 47^1
    (59, 1),   // 59^1
    (71, 1),   // 71^1
];

// Calculate which Monster prime power to use based on bucket size
fn select_monster_subdivision(piece_count: usize) -> (u8, u8, u64) {
    // Select appropriate prime power based on density
    let (prime, power) = if piece_count < 2 {
        (2, 1)      // 2^1 = 2
    } else if piece_count < 4 {
        (2, 2)      // 2^2 = 4
    } else if piece_count < 8 {
        (2, 3)      // 2^3 = 8
    } else if piece_count < 16 {
        (2, 4)      // 2^4 = 16
    } else if piece_count < 32 {
        (2, 5)      // 2^5 = 32
    } else if piece_count < 64 {
        (2, 6)      // 2^6 = 64
    } else if piece_count < 128 {
        (2, 7)      // 2^7 = 128
    } else if piece_count < 256 {
        (2, 8)      // 2^8 = 256
    } else if piece_count < 512 {
        (2, 9)      // 2^9 = 512
    } else if piece_count < 1024 {
        (2, 10)     // 2^10 = 1024
    } else if piece_count < 2048 {
        (3, 7)      // 3^7 = 2187
    } else if piece_count < 4096 {
        (2, 12)     // 2^12 = 4096
    } else if piece_count < 8192 {
        (3, 8)      // 3^8 = 6561
    } else if piece_count < 16384 {
        (2, 14)     // 2^14 = 16384
    } else if piece_count < 32768 {
        (3, 10)     // 3^10 = 59049
    } else if piece_count < 65536 {
        (2, 16)     // 2^16 = 65536
    } else if piece_count < 131072 {
        (3, 11)     // 3^11 = 177147
    } else if piece_count < 262144 {
        (5, 7)      // 5^7 = 78125
    } else if piece_count < 524288 {
        (3, 12)     // 3^12 = 531441
    } else if piece_count < 1048576 {
        (2, 20)     // 2^20 = 1048576
    } else {
        (3, 13)     // 3^13 = 1594323
    };
    
    let grid_size = (prime as u64).pow(power as u32);
    (prime, power, grid_size)
}

// Calculate shard ID using prime^power grid
fn calculate_shard_id(piece: &PieceIndex, prime: u8, power: u8, grid_size: u64) -> u64 {
    let center_lat = (piece.min_lat + piece.max_lat) / 2.0;
    let center_lon = (piece.min_lon + piece.max_lon) / 2.0;
    
    // Normalize to [0, 1]
    let lat_norm = (center_lat + 90.0) / 180.0;
    let lon_norm = (center_lon + 180.0) / 360.0;
    
    // Calculate grid coordinates
    let grid_dim = (grid_size as f64).sqrt() as u64;
    let lat_idx = (lat_norm * grid_dim as f64) as u64;
    let lon_idx = (lon_norm * grid_dim as f64) as u64;
    
    // Combine into single shard ID
    lat_idx * grid_dim + lon_idx
}

fn main() -> Result<()> {
    let index_file = "complete_spatial_index.jsonl";
    let output_dir = "monster_shards";
    
    println!("🗺️  Monster Group Hierarchical Sharding");
    println!("📦 Using prime powers: 2^46, 3^20, 5^9, 7^6, 11^2, 13^3, ...");
    println!();
    
    fs::create_dir_all(&output_dir)?;
    
    // First pass: group by geographic buckets (71×59×47)
    let file = File::open(index_file)?;
    let reader = BufReader::new(file);
    
    let mut buckets: HashMap<(u8, u8, u8), Vec<PieceIndex>> = HashMap::new();
    let mut total_pieces = 0;
    
    println!("📊 Pass 1: Grouping into geographic buckets...");
    
    for line in reader.lines() {
        let line = line?;
        let piece: PieceIndex = serde_json::from_str(&line)?;
        total_pieces += 1;
        
        let center_lat = (piece.min_lat + piece.max_lat) / 2.0;
        let center_lon = (piece.min_lon + piece.max_lon) / 2.0;
        let lat_span = piece.max_lat - piece.min_lat;
        let lon_span = piece.max_lon - piece.min_lon;
        let area = lat_span * lon_span;
        
        let lat_block = (((center_lat + 90.0) / 180.0) * 71.0).clamp(0.0, 70.0) as u8;
        let lon_block = (((center_lon + 180.0) / 360.0) * 59.0).clamp(0.0, 58.0) as u8;
        let log_area = if area > 0.0 { area.log10() } else { -10.0 };
        let size_block = ((log_area + 10.0) / 20.0 * 47.0).clamp(0.0, 46.0) as u8;
        
        buckets.entry((lat_block, lon_block, size_block))
            .or_insert_with(Vec::new)
            .push(piece);
        
        if total_pieces % 1000 == 0 {
            println!("  Processed {} pieces into {} buckets", total_pieces, buckets.len());
        }
    }
    
    println!();
    println!("✅ Created {} geographic buckets", buckets.len());
    println!();
    println!("📊 Pass 2: Subdividing with Monster prime powers...");
    
    let mut total_shards = 0;
    let mut bucket_count = 0;
    let total_buckets = buckets.len();
    
    for ((lat_block, lon_block, size_block), pieces) in buckets {
        bucket_count += 1;
        let bucket_id = format!("{}_{}_{}",lat_block, lon_block, size_block);
        
        // Select appropriate Monster prime power
        let (prime, power, grid_size) = select_monster_subdivision(pieces.len());
        
        // Subdivide pieces into shards
        let mut shards: HashMap<u64, Vec<u32>> = HashMap::new();
        
        for piece in &pieces {
            let shard_id = calculate_shard_id(piece, prime, power, grid_size);
            shards.entry(shard_id)
                .or_insert_with(Vec::new)
                .push(piece.piece_id);
        }
        
        // Save each shard
        for (shard_id, piece_ids) in shards {
            let shard_pieces: Vec<&PieceIndex> = pieces.iter()
                .filter(|p| piece_ids.contains(&p.piece_id))
                .collect();
            
            let min_lat = shard_pieces.iter().map(|p| p.min_lat).fold(f64::MAX, f64::min);
            let max_lat = shard_pieces.iter().map(|p| p.max_lat).fold(f64::MIN, f64::max);
            let min_lon = shard_pieces.iter().map(|p| p.min_lon).fold(f64::MAX, f64::min);
            let max_lon = shard_pieces.iter().map(|p| p.max_lon).fold(f64::MIN, f64::max);
            let node_count: usize = shard_pieces.iter().map(|p| p.node_count).sum();
            let wikidata_count: usize = shard_pieces.iter().map(|p| p.wikidata_count).sum();
            
            let monster_shard = MonsterShard {
                bucket_id: bucket_id.clone(),
                prime,
                power,
                shard_id,
                piece_ids,
                min_lat,
                max_lat,
                min_lon,
                max_lon,
                node_count,
                wikidata_count,
            };
            
            let shard_file = format!("{}/bucket_{}_prime{}pow{}_shard{}.json",
                output_dir, bucket_id, prime, power, shard_id);
            let json = serde_json::to_string_pretty(&monster_shard)?;
            fs::write(shard_file, json)?;
            
            total_shards += 1;
        }
        
        if bucket_count % 10 == 0 {
            println!("  Bucket {}/{}: {} pieces → {}^{} grid → {} shards",
                bucket_count, total_buckets, pieces.len(), prime, power, 
                (prime as u64).pow(power as u32));
        }
    }
    
    println!();
    println!("✅ Monster sharding complete!");
    println!("📊 Geographic buckets: {}", total_buckets);
    println!("📊 Monster shards: {}", total_shards);
    println!("💾 Saved to: {}/", output_dir);
    println!();
    println!("📈 Statistics:");
    println!("  Avg pieces per bucket: {:.1}", total_pieces as f64 / total_buckets as f64);
    println!("  Avg shards per bucket: {:.1}", total_shards as f64 / total_buckets as f64);
    
    Ok(())
}
