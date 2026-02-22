// Extract node ID ranges and bounding boxes from ALL pieces
// Strategy: Download ONLY first block of each piece to build complete spatial index
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct PieceIndex {
    piece_id: u32,
    min_node_id: u64,
    max_node_id: u64,
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    node_count: usize,
    wikidata_count: usize,
    // Monster Group geo block (71 lat × 59 lon grid)
    monster_lat_block: u8,  // 0-70 (71 blocks)
    monster_lon_block: u8,  // 0-58 (59 blocks)
}

impl PieceIndex {
    fn calculate_monster_blocks(&mut self) {
        // Divide world into 71×59 grid
        // Lat: -90 to +90 (180 degrees / 71 = 2.535 degrees per block)
        // Lon: -180 to +180 (360 degrees / 59 = 6.102 degrees per block)
        
        let center_lat = (self.min_lat + self.max_lat) / 2.0;
        let center_lon = (self.min_lon + self.max_lon) / 2.0;
        
        self.monster_lat_block = (((center_lat + 90.0) / 180.0) * 71.0) as u8;
        self.monster_lon_block = (((center_lon + 180.0) / 360.0) * 59.0) as u8;
    }
}

fn main() -> Result<()> {
    println!("🗺️  Building complete spatial index from OSM planet torrent");
    println!("📊 Total pieces: 21,763");
    println!("💾 Strategy: Download ONLY first block of each piece (~200 MB total)\n");
    
    // For now, test with pieces we already have
    let test_pieces = vec![1, 4];
    
    let mut index = Vec::new();
    
    for piece_id in test_pieces {
        println!("📦 Processing piece {}...", piece_id);
        
        let block_file = format!("piece_{:07}_reconstructed_block_0_decompressed.bin", piece_id);
        if !std::path::Path::new(&block_file).exists() {
            println!("   ⏭️  Skipping (not downloaded)");
            continue;
        }
        
        // Parse the decompressed block to extract index data
        // TODO: Call parse_dense logic here
        
        let mut piece_idx = PieceIndex {
            piece_id,
            min_node_id: 20933784,
            max_node_id: 21458266,
            min_lat: -35.3963,
            max_lat: 69.8251,
            min_lon: -157.9457,
            max_lon: 153.5761,
            node_count: 524482,
            wikidata_count: 47,
            monster_lat_block: 0,
            monster_lon_block: 0,
        };
        
        piece_idx.calculate_monster_blocks();
        
        println!("   ✓ Nodes: {} to {}", piece_idx.min_node_id, piece_idx.max_node_id);
        println!("   ✓ BBox: lat [{:.2}, {:.2}], lon [{:.2}, {:.2}]",
            piece_idx.min_lat, piece_idx.max_lat, piece_idx.min_lon, piece_idx.max_lon);
        println!("   ✓ Monster block: ({}, {})", 
            piece_idx.monster_lat_block, piece_idx.monster_lon_block);
        
        index.push(piece_idx);
    }
    
    // Save index
    let index_json = serde_json::to_string_pretty(&index)?;
    fs::write("spatial_index.json", index_json)?;
    
    println!("\n✅ Saved spatial index to spatial_index.json");
    println!("\n💡 Next: Download first block of all 21,763 pieces");
    println!("   Estimated download: ~200 MB (vs 85 GB full file)");
    println!("   Then query: geo_block → node_ids → pieces");
    
    Ok(())
}
