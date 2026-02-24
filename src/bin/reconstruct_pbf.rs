// Reconstruct PBF pieces from chunks and extract data
use std::fs;
use std::collections::BTreeMap;
use osmpbf::{Element, ElementReader};

fn main() -> anyhow::Result<()> {
    let chunks_dir = "chunks";
    
    println!("🔍 Scanning chunks directory...");
    
    // Group chunks by piece
    let mut pieces: BTreeMap<u32, BTreeMap<u64, Vec<u8>>> = BTreeMap::new();
    
    for entry in fs::read_dir(chunks_dir)? {
        let entry = entry?;
        let filename = entry.file_name();
        let name = filename.to_string_lossy();
        
        // Parse: piece_0000000_offset_0000000000.bin
        if let Some(parts) = parse_chunk_filename(&name) {
            let (piece_id, offset) = parts;
            let data = fs::read(entry.path())?;
            
            pieces.entry(piece_id)
                .or_insert_with(BTreeMap::new)
                .insert(offset, data);
        }
    }
    
    println!("✓ Found {} pieces with chunks", pieces.len());
    
    // Reconstruct each piece
    for (piece_id, chunks) in pieces {
        println!("\n📦 Piece {}: {} chunks", piece_id, chunks.len());
        
        // Reconstruct piece by concatenating chunks in order (no gap filling)
        let mut piece_data = Vec::new();
        
        for (_offset, data) in chunks {
            piece_data.extend_from_slice(&data);
        }
        
        println!("   Reconstructed size: {} bytes ({} KB)", piece_data.len(), piece_data.len() / 1024);
        
        // Try to parse as OSM PBF
        println!("   Parsing OSM PBF...");
        match parse_osm_pbf(&piece_data) {
            Ok((nodes, ways, relations, bbox)) => {
                println!("   ✓ Valid OSM PBF:");
                println!("     Nodes: {}", nodes);
                println!("     Ways: {}", ways);
                println!("     Relations: {}", relations);
                if let Some((min_lat, max_lat, min_lon, max_lon)) = bbox {
                    println!("     BBox: lat[{:.4}, {:.4}], lon[{:.4}, {:.4}]", 
                        min_lat, max_lat, min_lon, max_lon);
                }
                
                // Save reconstructed piece
                let output = format!("piece_{:07}_reconstructed.pbf", piece_id);
                fs::write(&output, &piece_data)?;
                println!("   ✓ Saved to {}", output);
            }
            Err(e) => {
                println!("   ✗ Parse error: {}", e);
            }
        }
    }
    
    Ok(())
}

fn parse_chunk_filename(name: &str) -> Option<(u32, u64)> {
    // piece_0000000_offset_0000000000.bin
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() >= 4 && parts[0] == "piece" && parts[2] == "offset" {
        let piece_id = parts[1].parse::<u32>().ok()?;
        let offset = parts[3].trim_end_matches(".bin").parse::<u64>().ok()?;
        Some((piece_id, offset))
    } else {
        None
    }
}

fn parse_osm_pbf(data: &[u8]) -> anyhow::Result<(usize, usize, usize, Option<(f64, f64, f64, f64)>)> {
    let reader = ElementReader::new(std::io::Cursor::new(data));
    
    let mut nodes = 0;
    let mut ways = 0;
    let mut relations = 0;
    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;
    let mut min_lon = f64::MAX;
    let mut max_lon = f64::MIN;
    let mut found_coords = false;
    
    for element in reader.par_map_reduce(
        |element| {
            match element {
                Element::Node(node) => {
                    (1, 0, 0, Some((node.lat(), node.lon())))
                }
                Element::Way(_) => (0, 1, 0, None),
                Element::Relation(_) => (0, 0, 1, None),
                _ => (0, 0, 0, None),
            }
        },
        || (0, 0, 0, None),
        |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3.or(b.3))
    ) {
        nodes += element.0;
        ways += element.1;
        relations += element.2;
        
        if let Some((lat, lon)) = element.3 {
            min_lat = min_lat.min(lat);
            max_lat = max_lat.max(lat);
            min_lon = min_lon.min(lon);
            max_lon = max_lon.max(lon);
            found_coords = true;
        }
    }
    
    let bbox = if found_coords {
        Some((min_lat, max_lat, min_lon, max_lon))
    } else {
        None
    };
    
    Ok((nodes, ways, relations, bbox))
}
