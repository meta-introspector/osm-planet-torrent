// Parquet index for OSM torrent pieces
// Phase 1: Index - Build geographic index of all pieces
// Phase 2: Fetch - Download only needed pieces
// Sharding: mod (71 * 59 * 497) = mod 2,082,203 (Monster Group primes)

use serde::{Deserialize, Serialize};

const SHARD_MODULO: u32 = 71 * 59 * 497; // 2,082,203

#[derive(Debug, Serialize, Deserialize)]
pub struct PieceIndex {
    pub piece_id: u32,
    pub byte_offset: u64,
    pub byte_length: u32,
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
    pub osm_node_count: u64,
    pub has_wikidata: bool,
    pub shard_id: u32, // piece_id % SHARD_MODULO
}

impl PieceIndex {
    pub fn new(
        piece_id: u32,
        byte_offset: u64,
        byte_length: u32,
        lat_min: f64,
        lat_max: f64,
        lon_min: f64,
        lon_max: f64,
    ) -> Self {
        Self {
            piece_id,
            byte_offset,
            byte_length,
            lat_min,
            lat_max,
            lon_min,
            lon_max,
            osm_node_count: 0,
            has_wikidata: false,
            shard_id: piece_id % SHARD_MODULO,
        }
    }
}

// Phase 1: Index
pub fn build_index_from_torrent(
    torrent_file: &str,
) -> anyhow::Result<Vec<PieceIndex>> {
    println!("📊 Phase 1: Building piece index...");
    
    let torrent = lava_torrent::torrent::v1::Torrent::read_from_file(torrent_file)?;
    let piece_length = torrent.piece_length;
    let total_length = torrent.length;
    let num_pieces = torrent.pieces.len();
    
    println!("   Total pieces: {}", num_pieces);
    println!("   Piece length: {} KB", piece_length / 1024);
    println!("   Sharding: mod {}", SHARD_MODULO);
    
    let mut pieces = Vec::new();
    
    for piece_id in 0..num_pieces {
        let byte_offset = (piece_id as u64) * (piece_length as u64);
        let byte_length = if piece_id == num_pieces - 1 {
            ((total_length as u64) - byte_offset) as u32
        } else {
            piece_length as u32
        };
        
        let piece = PieceIndex::new(
            piece_id as u32,
            byte_offset,
            byte_length,
            0.0, 0.0, 0.0, 0.0,
        );
        
        pieces.push(piece);
    }
    
    Ok(pieces)
}

// Save sharded index
pub fn save_sharded_index(
    pieces: Vec<PieceIndex>,
    output_dir: &str,
) -> anyhow::Result<()> {
    println!("💾 Saving sharded index (mod {})...", SHARD_MODULO);
    
    std::fs::create_dir_all(output_dir)?;
    
    let mut shards: std::collections::HashMap<u32, Vec<PieceIndex>> = std::collections::HashMap::new();
    
    for piece in pieces {
        shards.entry(piece.shard_id).or_insert_with(Vec::new).push(piece);
    }
    
    println!("   Total shards: {}", shards.len());
    
    for (shard_id, shard_pieces) in shards {
        let shard_file = format!("{}/shard_{:07}.json", output_dir, shard_id);
        let json = serde_json::to_string(&shard_pieces)?;
        std::fs::write(&shard_file, json)?;
    }
    
    println!("✓ Index saved to {}", output_dir);
    Ok(())
}

// Phase 2: Fetch
pub fn load_shards_for_location(
    index_dir: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
) -> anyhow::Result<Vec<u32>> {
    println!("🔍 Phase 2: Loading shards for location...");
    
    let mut needed_pieces = Vec::new();
    
    for entry in std::fs::read_dir(index_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let json = std::fs::read_to_string(&path)?;
            let pieces: Vec<PieceIndex> = serde_json::from_str(&json)?;
            
            for piece in pieces {
                if overlaps_location(&piece, lat, lon, radius_miles) {
                    needed_pieces.push(piece.piece_id);
                }
            }
        }
    }
    
    println!("✓ Found {} pieces needed", needed_pieces.len());
    Ok(needed_pieces)
}

fn overlaps_location(piece: &PieceIndex, lat: f64, lon: f64, radius_miles: f64) -> bool {
    let lat_deg_per_mile = 1.0 / 69.0;
    let lon_deg_per_mile = 1.0 / (69.0 * lat.to_radians().cos());
    
    let lat_radius = radius_miles * lat_deg_per_mile;
    let lon_radius = radius_miles * lon_deg_per_mile;
    
    let min_lat = lat - lat_radius;
    let max_lat = lat + lat_radius;
    let min_lon = lon - lon_radius;
    let max_lon = lon + lon_radius;
    
    !(piece.lat_max < min_lat || piece.lat_min > max_lat ||
      piece.lon_max < min_lon || piece.lon_min > max_lon)
}
