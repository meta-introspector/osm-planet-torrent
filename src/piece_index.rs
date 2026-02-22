// Parquet index for OSM torrent pieces
// Phase 1: Index - Build geographic index of all pieces
// Phase 2: Fetch - Download only needed pieces (MINIMAL - max 200KB)
// Sharding: mod (71 * 59 * 47) = mod 196,883 (Monster Group order)

use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const SHARD_MODULO: u32 = 71 * 59 * 47; // 196,883

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub priority: u8,  // 0=highest, 255=lowest
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
            priority: 255,
        }
    }
    
    pub fn intersects_bbox(&self, min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> bool {
        !(self.lat_max < min_lat || self.lat_min > max_lat ||
          self.lon_max < min_lon || self.lon_min > max_lon)
    }
    
    pub fn distance_to_center(&self, center_lat: f64, center_lon: f64) -> f64 {
        let piece_lat = (self.lat_min + self.lat_max) / 2.0;
        let piece_lon = (self.lon_min + self.lon_max) / 2.0;
        ((piece_lat - center_lat).powi(2) + (piece_lon - center_lon).powi(2)).sqrt()
    }
}

impl Ord for PieceIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Min-heap
    }
}

impl PartialOrd for PieceIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PieceIndex {}
impl PartialEq for PieceIndex {
    fn eq(&self, other: &Self) -> bool {
        self.piece_id == other.piece_id
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

// Phase 2: Plan minimal fetch (max 200KB)
// Uses existing ramanujan-location-index.json for piece→shard mapping
pub fn plan_minimal_fetch(
    pieces: &[PieceIndex],
    viewport: (f64, f64, f64, f64), // (min_lat, min_lon, max_lat, max_lon)
    max_bytes: u32,
) -> Vec<PieceIndex> {
    let (min_lat, min_lon, max_lat, max_lon) = viewport;
    let center_lat = (min_lat + max_lat) / 2.0;
    let center_lon = (min_lon + max_lon) / 2.0;
    
    // Load existing index if available
    let known_pieces = load_ramanujan_index().unwrap_or_default();
    
    let mut heap = BinaryHeap::new();
    
    for piece in pieces {
        if piece.intersects_bbox(min_lat, min_lon, max_lat, max_lon) {
            let mut p = piece.clone();
            let dist = p.distance_to_center(center_lat, center_lon);
            p.priority = (dist * 10.0).min(255.0) as u8;
            
            // Boost priority if we have location data for this piece
            if known_pieces.contains(&p.piece_id) {
                p.priority = p.priority.saturating_sub(50); // Higher priority
            }
            
            heap.push(p);
        }
    }
    
    let mut plan = Vec::new();
    let mut total_bytes = 0u32;
    
    while let Some(piece) = heap.pop() {
        if total_bytes + piece.byte_length > max_bytes {
            break;
        }
        total_bytes += piece.byte_length;
        plan.push(piece);
    }
    
    println!("📋 Fetch plan: {} pieces, {}KB total", plan.len(), total_bytes / 1024);
    plan
}

fn load_ramanujan_index() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    use std::fs;
    let data = fs::read_to_string("ramanujan-location-index.json")?;
    let json: serde_json::Value = serde_json::from_str(&data)?;
    
    let pieces = json["locations"]
        .as_array()
        .ok_or("No locations array")?
        .iter()
        .filter_map(|loc| loc["piece"].as_u64().map(|p| p as u32))
        .collect();
    
    Ok(pieces)
}
        
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
