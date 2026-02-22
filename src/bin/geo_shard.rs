// Two-level hierarchical sharding:
// Level 1: Geographic buckets (71 lat × 59 lon × 47 alt/size)
// Level 2: Within each bucket, subdivide by smaller primes (41, 31, 29, ..., 2)

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
struct GeoBucket {
    lat_block: u8,      // 0-70 (71 blocks)
    lon_block: u8,      // 0-58 (59 blocks)
    size_block: u8,     // 0-46 (47 blocks by area)
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    piece_count: usize,
    node_count: usize,
    wikidata_count: usize,
    sub_shards: HashMap<u8, SubShard>,
}

#[derive(Debug, Serialize)]
struct SubShard {
    prime: u8,          // Which prime subdivides this (41, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2)
    shard_id: u16,      // ID within this prime's subdivision
    piece_ids: Vec<u32>,
    node_count: usize,
    wikidata_count: usize,
}

// Calculate geographic bucket (71 × 59 × 47)
fn calculate_geo_bucket(piece: &PieceIndex) -> (u8, u8, u8) {
    let center_lat = (piece.min_lat + piece.max_lat) / 2.0;
    let center_lon = (piece.min_lon + piece.max_lon) / 2.0;
    
    // Lat: -90 to +90 → 71 blocks
    let lat_block = (((center_lat + 90.0) / 180.0) * 71.0).clamp(0.0, 70.0) as u8;
    
    // Lon: -180 to +180 → 59 blocks
    let lon_block = (((center_lon + 180.0) / 360.0) * 59.0).clamp(0.0, 58.0) as u8;
    
    // Size: area → 47 blocks (logarithmic)
    let lat_span = piece.max_lat - piece.min_lat;
    let lon_span = piece.max_lon - piece.min_lon;
    let area = lat_span * lon_span;
    let log_area = if area > 0.0 { area.log10() } else { -10.0 };
    let size_block = ((log_area + 10.0) / 20.0 * 47.0).clamp(0.0, 46.0) as u8;
    
    (lat_block, lon_block, size_block)
}

// Subdivide within bucket using smaller primes
fn calculate_sub_shard(piece: &PieceIndex, bucket_pieces: usize) -> (u8, u16) {
    // Choose prime based on bucket density
    let prime = if bucket_pieces < 10 {
        2   // Binary subdivision
    } else if bucket_pieces < 50 {
        3   // Ternary
    } else if bucket_pieces < 100 {
        5   // Quinary
    } else if bucket_pieces < 200 {
        7   // Septenary
    } else if bucket_pieces < 500 {
        11  // Base-11
    } else if bucket_pieces < 1000 {
        13  // Base-13
    } else if bucket_pieces < 2000 {
        17  // Base-17
    } else if bucket_pieces < 5000 {
        19  // Base-19
    } else if bucket_pieces < 10000 {
        23  // Base-23
    } else if bucket_pieces < 20000 {
        29  // Base-29
    } else if bucket_pieces < 50000 {
        31  // Base-31
    } else {
        41  // Base-41
    };
    
    // Calculate shard ID within this prime's subdivision
    let center_lat = (piece.min_lat + piece.max_lat) / 2.0;
    let center_lon = (piece.min_lon + piece.max_lon) / 2.0;
    
    let lat_norm = (center_lat + 90.0) / 180.0;
    let lon_norm = (center_lon + 180.0) / 360.0;
    
    let lat_idx = (lat_norm * prime as f64) as u16;
    let lon_idx = (lon_norm * prime as f64) as u16;
    
    let shard_id = lat_idx * prime as u16 + lon_idx;
    
    (prime, shard_id)
}

fn main() -> Result<()> {
    let index_file = "complete_spatial_index.jsonl";
    let output_dir = "geo_shards";
    
    println!("🗺️  Two-level hierarchical sharding");
    println!("📦 Level 1: 71 × 59 × 47 = {} geographic buckets", 71 * 59 * 47);
    println!("📦 Level 2: Subdivide by primes (41, 31, 29, ..., 2)");
    println!();
    
    fs::create_dir_all(&output_dir)?;
    
    // First pass: group by geographic buckets
    let file = File::open(index_file)?;
    let reader = BufReader::new(file);
    
    let mut buckets: HashMap<(u8, u8, u8), Vec<PieceIndex>> = HashMap::new();
    let mut total_pieces = 0;
    
    println!("📊 Pass 1: Grouping into geographic buckets...");
    
    for line in reader.lines() {
        let line = line?;
        let piece: PieceIndex = serde_json::from_str(&line)?;
        total_pieces += 1;
        
        let bucket_key = calculate_geo_bucket(&piece);
        buckets.entry(bucket_key).or_insert_with(Vec::new).push(piece);
        
        if total_pieces % 1000 == 0 {
            println!("  Processed {} pieces into {} buckets", total_pieces, buckets.len());
        }
    }
    
    println!();
    println!("✅ Created {} geographic buckets from {} pieces", buckets.len(), total_pieces);
    println!();
    println!("📊 Pass 2: Subdividing buckets by smaller primes...");
    
    let mut total_sub_shards = 0;
    let mut bucket_count = 0;
    let total_buckets = buckets.len();
    
    for ((lat_block, lon_block, size_block), pieces) in buckets {
        bucket_count += 1;
        
        // Calculate bucket bounds
        let min_lat = (lat_block as f64 / 71.0) * 180.0 - 90.0;
        let max_lat = ((lat_block + 1) as f64 / 71.0) * 180.0 - 90.0;
        let min_lon = (lon_block as f64 / 59.0) * 360.0 - 180.0;
        let max_lon = ((lon_block + 1) as f64 / 59.0) * 360.0 - 180.0;
        
        // Subdivide this bucket
        let mut sub_shards: HashMap<(u8, u16), Vec<u32>> = HashMap::new();
        let bucket_size = pieces.len();
        
        for piece in &pieces {
            let (prime, shard_id) = calculate_sub_shard(piece, bucket_size);
            sub_shards.entry((prime, shard_id))
                .or_insert_with(Vec::new)
                .push(piece.piece_id);
        }
        
        // Create GeoBucket
        let mut geo_bucket = GeoBucket {
            lat_block,
            lon_block,
            size_block,
            min_lat,
            max_lat,
            min_lon,
            max_lon,
            piece_count: pieces.len(),
            node_count: pieces.iter().map(|p| p.node_count).sum(),
            wikidata_count: pieces.iter().map(|p| p.wikidata_count).sum(),
            sub_shards: HashMap::new(),
        };
        
        for ((prime, shard_id), piece_ids) in sub_shards {
            let node_count: usize = pieces.iter()
                .filter(|p| piece_ids.contains(&p.piece_id))
                .map(|p| p.node_count)
                .sum();
            let wikidata_count: usize = pieces.iter()
                .filter(|p| piece_ids.contains(&p.piece_id))
                .map(|p| p.wikidata_count)
                .sum();
            
            geo_bucket.sub_shards.insert(shard_id as u8, SubShard {
                prime,
                shard_id,
                piece_ids,
                node_count,
                wikidata_count,
            });
            
            total_sub_shards += 1;
        }
        
        // Save bucket
        let bucket_file = format!("{}/bucket_{}_{}_{}_{}.json", 
            output_dir, lat_block, lon_block, size_block, geo_bucket.sub_shards.len());
        let json = serde_json::to_string_pretty(&geo_bucket)?;
        fs::write(bucket_file, json)?;
        
        if bucket_count % 10 == 0 {
            println!("  Processed {} buckets, {} sub-shards", bucket_count, total_sub_shards);
        }
    }
    
    println!();
    println!("✅ Sharding complete!");
    println!("📊 Level 1: {} geographic buckets (71×59×47)", total_buckets);
    println!("📊 Level 2: {} sub-shards (by primes)", total_sub_shards);
    println!("💾 Saved to: {}/", output_dir);
    
    // Statistics
    println!();
    println!("📈 Statistics:");
    println!("  Avg pieces per bucket: {:.1}", total_pieces as f64 / total_buckets as f64);
    println!("  Avg sub-shards per bucket: {:.1}", total_sub_shards as f64 / total_buckets as f64);
    
    Ok(())
}
