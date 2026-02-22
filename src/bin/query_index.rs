// Query spatial index to find pieces containing target location or node IDs
use anyhow::Result;
use serde::{Deserialize, Serialize};
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
    monster_lat_block: u8,
    monster_lon_block: u8,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} geo <lat> <lon>     - Find pieces by location", args[0]);
        eprintln!("  {} node <node_id>      - Find piece by node ID", args[0]);
        eprintln!("  {} monster <lat> <lon> - Find Monster geo block", args[0]);
        eprintln!("\nExamples:");
        eprintln!("  {} geo 10.9617 79.3881  # Kumbakonam", args[0]);
        eprintln!("  {} node 2824755486      # Noor Nagar", args[0]);
        std::process::exit(1);
    }
    
    // Load spatial index
    let index_data = fs::read_to_string("spatial_index.json")?;
    let index: Vec<PieceIndex> = serde_json::from_str(&index_data)?;
    
    println!("📊 Loaded spatial index: {} pieces\n", index.len());
    
    match args[1].as_str() {
        "geo" => {
            let lat: f64 = args[2].parse()?;
            let lon: f64 = args[3].parse()?;
            
            println!("🎯 Searching for location: {}, {}", lat, lon);
            
            let mut found = Vec::new();
            for piece in &index {
                if lat >= piece.min_lat && lat <= piece.max_lat &&
                   lon >= piece.min_lon && lon <= piece.max_lon {
                    found.push(piece);
                }
            }
            
            println!("✓ Found {} pieces containing this location:\n", found.len());
            for piece in found {
                println!("   Piece {}: nodes {} to {}", 
                    piece.piece_id, piece.min_node_id, piece.max_node_id);
                println!("      Monster block: ({}, {})", 
                    piece.monster_lat_block, piece.monster_lon_block);
                println!("      {} wikidata entities", piece.wikidata_count);
            }
        }
        
        "node" => {
            let node_id: u64 = args[2].parse()?;
            
            println!("🎯 Searching for node ID: {}", node_id);
            
            for piece in &index {
                if node_id >= piece.min_node_id && node_id <= piece.max_node_id {
                    println!("✓ Found in piece {}", piece.piece_id);
                    println!("   Node range: {} to {}", piece.min_node_id, piece.max_node_id);
                    println!("   BBox: lat [{:.2}, {:.2}], lon [{:.2}, {:.2}]",
                        piece.min_lat, piece.max_lat, piece.min_lon, piece.max_lon);
                    println!("   Monster block: ({}, {})", 
                        piece.monster_lat_block, piece.monster_lon_block);
                    break;
                }
            }
        }
        
        "monster" => {
            let lat: f64 = args[2].parse()?;
            let lon: f64 = args[3].parse()?;
            
            let monster_lat = (((lat + 90.0) / 180.0) * 71.0) as u8;
            let monster_lon = (((lon + 180.0) / 360.0) * 59.0) as u8;
            
            println!("🎯 Location: {}, {}", lat, lon);
            println!("📍 Monster geo block: ({}, {})", monster_lat, monster_lon);
            println!("   Lat block {}/71: {:.2}° to {:.2}°", 
                monster_lat,
                (monster_lat as f64 * 180.0 / 71.0) - 90.0,
                ((monster_lat + 1) as f64 * 180.0 / 71.0) - 90.0);
            println!("   Lon block {}/59: {:.2}° to {:.2}°",
                monster_lon,
                (monster_lon as f64 * 360.0 / 59.0) - 180.0,
                ((monster_lon + 1) as f64 * 360.0 / 59.0) - 180.0);
            
            // Find all pieces in this monster block
            let pieces: Vec<_> = index.iter()
                .filter(|p| p.monster_lat_block == monster_lat && p.monster_lon_block == monster_lon)
                .collect();
            
            println!("\n✓ {} pieces in this Monster block", pieces.len());
            for piece in pieces {
                println!("   Piece {}: {} wikidata entities", 
                    piece.piece_id, piece.wikidata_count);
            }
        }
        
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
