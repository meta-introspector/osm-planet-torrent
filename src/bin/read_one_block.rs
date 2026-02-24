// Read ONE block from OSM planet for Kumbakonam area
use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

#[derive(Deserialize)]
struct IndexEntry {
    piece_id: u32,
    byte_offset: u64,
    node_count: u64,
}

fn main() -> Result<()> {
    println!("🎯 Reading ONE block for Kumbakonam");
    
    let index_file = "complete_spatial_index.jsonl";
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    
    // Find piece with most nodes
    println!("📖 Finding piece with most nodes...");
    let file = File::open(index_file)?;
    let reader = BufReader::new(file);
    
    let mut best: Option<IndexEntry> = None;
    
    for line in reader.lines() {
        let entry: IndexEntry = serde_json::from_str(&line?)?;
        
        if best.is_none() || entry.node_count > best.as_ref().unwrap().node_count {
            best = Some(entry);
        }
    }
    
    let best = best.unwrap();
    println!("✅ Found piece {} with {} nodes", best.piece_id, best.node_count);
    println!("   Offset: {} bytes", best.byte_offset);
    println!();
    
    // Read 4MB block (standard piece size)
    let block_size = 4 * 1024 * 1024;
    println!("📦 Reading {} MB block from planet file...", block_size / 1024 / 1024);
    let mut planet = File::open(planet_file)?;
    planet.seek(SeekFrom::Start(best.byte_offset))?;
    
    let mut buffer = vec![0u8; block_size];
    planet.read_exact(&mut buffer)?;
    
    println!("✅ Read {} bytes", buffer.len());
    
    // Save to file
    let output = format!("piece_{}.pbf", best.piece_id);
    std::fs::write(&output, &buffer)?;
    
    println!("💾 Saved to: {}", output);
    println!();
    println!("🎯 Next: Parse this piece to see nodes");
    
    Ok(())
}
