// Extract Kumbakonam area (Ramanujan's house) from OSM planet
// Read only the relevant pieces and merge into one file

use anyhow::Result;
use osmpbf::{Element, ElementReader};
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    println!("🎯 Extracting Kumbakonam area (Ramanujan's house)");
    println!("📍 Location: 10.9617°N, 79.3881°E");
    println!("📏 Radius: 20 miles");
    println!();
    
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let output_file = "kumbakonam_area.jsonl";
    
    // Bounding box for Kumbakonam + 20 miles
    let target_lat = 10.9617;
    let target_lon = 79.3881;
    let radius_deg = 20.0 / 69.0;
    
    let min_lat = target_lat - radius_deg;
    let max_lat = target_lat + radius_deg;
    let min_lon = target_lon - radius_deg;
    let max_lon = target_lon + radius_deg;
    
    println!("📦 Bounding box:");
    println!("   Lat: [{:.4}, {:.4}]", min_lat, max_lat);
    println!("   Lon: [{:.4}, {:.4}]", min_lon, max_lon);
    println!();
    
    println!("📖 Reading planet file...");
    let reader = ElementReader::from_path(planet_file)?;
    
    let mut output = File::create(output_file)?;
    let mut node_count = 0;
    let mut wikidata_count = 0;
    let mut processed = 0u64;
    
    // Extract nodes in bounding box
    if let Ok(elements) = reader.par_map_reduce(
        |element| {
            let mut nodes = Vec::new();
            
            match element {
                Element::Node(node) => {
                    let lat = node.lat();
                    let lon = node.lon();
                    
                    // Check if in bounding box
                    if lat >= min_lat && lat <= max_lat &&
                       lon >= min_lon && lon <= max_lon {
                        
                        let mut tags = Vec::new();
                        let mut has_wikidata = false;
                        
                        for (key, val) in node.tags() {
                            tags.push((key.to_string(), val.to_string()));
                            if key == "wikidata" {
                                has_wikidata = true;
                            }
                        }
                        
                        nodes.push((node.id(), lat, lon, tags, has_wikidata));
                    }
                }
                _ => {}
            }
            
            nodes
        },
        Vec::new,
        |mut a, b| {
            a.extend(b);
            a
        },
    ) {
        for (id, lat, lon, tags, has_wikidata) in elements {
            let json = serde_json::json!({
                "id": id,
                "lat": lat,
                "lon": lon,
                "tags": tags,
            });
            writeln!(output, "{}", json)?;
            
            node_count += 1;
            if has_wikidata {
                wikidata_count += 1;
            }
        }
        
        processed += 1;
        if processed % 100 == 0 {
            println!("  Found {} nodes ({} with wikidata)", node_count, wikidata_count);
        }
    }
    
    println!();
    println!("✅ Extraction complete!");
    println!("📊 Results:");
    println!("   Nodes: {}", node_count);
    println!("   Wikidata entities: {}", wikidata_count);
    println!("   Output: {}", output_file);
    
    Ok(())
}
