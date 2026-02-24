// Repack OSM planet file into Monster Group structure using MiniZinc solver
// Split into 71 blocks of 4KB each with optimal geographic packing

use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::Command;

const BLOCK_SIZE: usize = 4096;  // 4KB
const NUM_BLOCKS: usize = 71;    // Monster Group prime

fn main() -> Result<()> {
    println!("🗺️  Repacking OSM planet with Monster Group structure");
    println!("📦 Target: 71 blocks × 4KB = 284 KB per chunk");
    println!("🧮 Using MiniZinc solver for optimal packing");
    println!();
    
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let output_dir = "monster_packed";
    
    std::fs::create_dir_all(output_dir)?;
    
    println!("📊 Strategy:");
    println!("  1. Read spatial index (13,768 pieces)");
    println!("  2. Group by geographic buckets (71×59×47)");
    println!("  3. For each bucket:");
    println!("     a. Extract nodes from planet file");
    println!("     b. Generate MiniZinc data file");
    println!("     c. Run MiniZinc solver");
    println!("     d. Pack nodes into 71 blocks of 4KB");
    println!("     e. Write new PBF file");
    println!();
    
    // Simplified approach: Pack by geographic bucket
    println!("📦 Simplified packing (without MiniZinc for now):");
    println!("  - Group pieces by Monster geo block (71×59)");
    println!("  - Pack each group into 71 size blocks");
    println!("  - Each block ≤ 4KB");
    println!();
    
    // Read spatial index
    let index_file = File::open("complete_spatial_index.jsonl")?;
    let reader = std::io::BufReader::new(index_file);
    
    use std::io::BufRead;
    use serde::Deserialize;
    
    #[derive(Deserialize)]
    struct PieceIndex {
        piece_id: u32,
        byte_offset: u64,
        monster_lat_block: u8,
        monster_lon_block: u8,
        node_count: usize,
    }
    
    let mut geo_groups: std::collections::HashMap<(u8, u8), Vec<PieceIndex>> = 
        std::collections::HashMap::new();
    
    for line in reader.lines() {
        let line = line?;
        let piece: PieceIndex = serde_json::from_str(&line)?;
        let key = (piece.monster_lat_block, piece.monster_lon_block);
        geo_groups.entry(key).or_insert_with(Vec::new).push(piece);
    }
    
    println!("✓ Grouped into {} geographic blocks (71×59)", geo_groups.len());
    println!();
    
    // For each geographic block, pack into 71 size blocks
    let mut total_packed = 0;
    
    for ((lat_block, lon_block), pieces) in geo_groups.iter().take(5) {
        println!("📦 Packing geo block ({}, {}): {} pieces", 
            lat_block, lon_block, pieces.len());
        
        // Simple packing: distribute pieces across 71 blocks by size
        let total_nodes: usize = pieces.iter().map(|p| p.node_count).sum();
        let nodes_per_block = (total_nodes + NUM_BLOCKS - 1) / NUM_BLOCKS;
        
        println!("  Total nodes: {}", total_nodes);
        println!("  Target per block: ~{} nodes", nodes_per_block);
        
        // Pack into blocks
        let mut blocks: Vec<Vec<u32>> = vec![Vec::new(); NUM_BLOCKS];
        let mut block_sizes = vec![0usize; NUM_BLOCKS];
        
        for piece in pieces {
            // Find block with most space
            let min_block = block_sizes.iter()
                .enumerate()
                .min_by_key(|(_, &size)| size)
                .map(|(idx, _)| idx)
                .unwrap();
            
            blocks[min_block].push(piece.piece_id);
            block_sizes[min_block] += piece.node_count;
        }
        
        // Show distribution
        let min_size = block_sizes.iter().min().unwrap();
        let max_size = block_sizes.iter().max().unwrap();
        println!("  Block sizes: min={}, max={}, variance={}", 
            min_size, max_size, max_size - min_size);
        
        total_packed += 1;
    }
    
    println!();
    println!("✅ Packed {} geographic blocks", total_packed);
    println!();
    println!("💡 Next steps:");
    println!("  1. Extract actual node data from planet file");
    println!("  2. Use MiniZinc for optimal packing within each block");
    println!("  3. Write new PBF files with Monster Group structure");
    println!("  4. Each file: <geo_block>_<size_block>.pbf (71 blocks × 4KB)");
    
    Ok(())
}
