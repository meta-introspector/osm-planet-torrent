// Analyze temporal strata of OSM nodes
// Hypothesis: Old node IDs = stable important features (roads, buildings)
//            New node IDs = recent minor edits (tags, details)
// Question: For Kumbakonam area, which node ID ranges actually matter?

use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
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
}

fn main() -> Result<()> {
    println!("🔬 Temporal Strata Analysis of OSM Nodes");
    println!("📍 Target: Kumbakonam (10.9617°N, 79.3881°E) + 20 miles");
    println!();
    
    let target_lat = 10.9617;
    let target_lon = 79.3881;
    let radius_deg = 20.0 / 69.0;  // 20 miles
    
    let min_lat = target_lat - radius_deg;
    let max_lat = target_lat + radius_deg;
    let min_lon = target_lon - radius_deg;
    let max_lon = target_lon + radius_deg;
    
    println!("📦 Bounding box: lat [{:.4}, {:.4}], lon [{:.4}, {:.4}]", 
        min_lat, max_lat, min_lon, max_lon);
    println!();
    
    // Read spatial index
    let file = File::open("complete_spatial_index.jsonl")?;
    let reader = BufReader::new(file);
    
    let mut overlapping_pieces = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let piece: PieceIndex = serde_json::from_str(&line)?;
        
        // Check overlap
        if piece.min_lat <= max_lat && piece.max_lat >= min_lat &&
           piece.min_lon <= max_lon && piece.max_lon >= min_lon {
            overlapping_pieces.push(piece);
        }
    }
    
    println!("✓ Found {} overlapping pieces", overlapping_pieces.len());
    println!();
    
    // Analyze node ID distribution
    let mut node_id_ranges: Vec<(u64, u64, u32, usize, usize)> = overlapping_pieces.iter()
        .map(|p| (p.min_node_id, p.max_node_id, p.piece_id, p.node_count, p.wikidata_count))
        .collect();
    
    node_id_ranges.sort_by_key(|r| r.0);  // Sort by min_node_id
    
    // Group into temporal strata (by node ID age)
    let strata = vec![
        ("Ancient", 0u64, 1_000_000),           // Very old nodes
        ("Old", 1_000_000, 10_000_000),         // Old stable features
        ("Mature", 10_000_000, 100_000_000),    // Established features
        ("Recent", 100_000_000, 1_000_000_000), // Recent additions
        ("New", 1_000_000_000, 10_000_000_000), // Very recent
        ("Latest", 10_000_000_000, u64::MAX),   // Latest edits
    ];
    
    println!("📊 Temporal Strata Analysis:");
    println!();
    
    for (name, min_id, max_id) in &strata {
        let pieces_in_strata: Vec<_> = node_id_ranges.iter()
            .filter(|(min, max, _, _, _)| {
                // Piece overlaps with this strata
                *min <= *max_id && *max >= *min_id
            })
            .collect();
        
        if pieces_in_strata.is_empty() {
            continue;
        }
        
        let total_nodes: usize = pieces_in_strata.iter().map(|(_, _, _, n, _)| n).sum();
        let total_wikidata: usize = pieces_in_strata.iter().map(|(_, _, _, _, w)| w).sum();
        let piece_count = pieces_in_strata.len();
        
        println!("🔹 {} (node IDs {}-{})", name, min_id, max_id);
        println!("   Pieces: {}", piece_count);
        println!("   Nodes: {}", total_nodes);
        println!("   Wikidata: {}", total_wikidata);
        println!("   Download: ~{} MB", piece_count * 4);
        
        if total_wikidata > 0 {
            println!("   ⭐ Wikidata density: {:.2} per piece", 
                total_wikidata as f64 / piece_count as f64);
        }
        println!();
    }
    
    // Find the "sweet spot" - which strata have the most value?
    println!("💡 Value Analysis:");
    println!();
    
    // Calculate wikidata per MB for each strata
    for (name, min_id, max_id) in &strata {
        let pieces_in_strata: Vec<_> = node_id_ranges.iter()
            .filter(|(min, max, _, _, _)| *min <= *max_id && *max >= *min_id)
            .collect();
        
        if pieces_in_strata.is_empty() {
            continue;
        }
        
        let total_wikidata: usize = pieces_in_strata.iter().map(|(_, _, _, _, w)| w).sum();
        let download_mb = pieces_in_strata.len() * 4;
        
        if total_wikidata > 0 {
            let value_density = total_wikidata as f64 / download_mb as f64;
            println!("  {}: {:.2} wikidata/MB", name, value_density);
        }
    }
    
    println!();
    println!("🎯 Recommendation:");
    println!("  Focus on strata with highest wikidata density");
    println!("  These contain the most valuable geographic entities");
    
    // Show top 20 pieces by wikidata count
    println!();
    println!("📊 Top 20 pieces by Wikidata count:");
    
    let mut by_wikidata = overlapping_pieces.clone();
    by_wikidata.sort_by_key(|p| std::cmp::Reverse(p.wikidata_count));
    
    for (i, piece) in by_wikidata.iter().take(20).enumerate() {
        println!("  {}. Piece {}: {} wikidata, node IDs {}-{}", 
            i+1, piece.piece_id, piece.wikidata_count, 
            piece.min_node_id, piece.max_node_id);
    }
    
    let top20_wikidata: usize = by_wikidata.iter().take(20).map(|p| p.wikidata_count).sum();
    let total_wikidata: usize = overlapping_pieces.iter().map(|p| p.wikidata_count).sum();
    
    println!();
    println!("💡 Top 20 pieces contain {}/{} wikidata ({:.1}%)", 
        top20_wikidata, total_wikidata, 
        100.0 * top20_wikidata as f64 / total_wikidata as f64);
    println!("   Download: 80 MB vs {} MB for all", overlapping_pieces.len() * 4);
    println!("   Savings: {:.1}%!", 
        100.0 * (1.0 - 80.0 / (overlapping_pieces.len() * 4) as f64));
    
    Ok(())
}
