// Test modular sharding on piece_9055.pbf (4MB)
use anyhow::Result;
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("🧪 Testing modular sharding on planet file (first 1M nodes)");
    
    let reader = ElementReader::from_path("/mnt/data1/osm-planet/planet-latest.osm.pbf")?;
    
    let mut buckets: HashMap<(u8, u8, u8), u64> = HashMap::new();
    let mut total = 0;
    
    reader.for_each(|element| {
        match element {
            Element::Node(node) => {
                let id_bucket = (node.id() % 71) as u8;
                let lat_bucket = (((node.lat() + 90.0) * 100.0) as i64 % 41) as u8;
                let lon_bucket = (((node.lon() + 180.0) * 100.0) as i64 % 31) as u8;
                
                *buckets.entry((id_bucket, lat_bucket, lon_bucket)).or_insert(0) += 1;
                total += 1;
                
                if total % 100000 == 0 {
                    print!("\r  {} nodes, {} buckets", total, buckets.len());
                }
                
                if total >= 1_000_000 {
                    return;
                }
            }
            _ => {}
        }
    })?;
    
    println!();
    println!("✅ Processed {} nodes", total);
    println!("📦 Created {} buckets", buckets.len());
    println!();
    
    // Show distribution
    let mut sizes: Vec<_> = buckets.values().copied().collect();
    sizes.sort();
    
    println!("📊 Bucket size distribution:");
    println!("   Min: {}", sizes.first().unwrap_or(&0));
    println!("   Max: {}", sizes.last().unwrap_or(&0));
    println!("   Median: {}", sizes.get(sizes.len()/2).unwrap_or(&0));
    println!("   Mean: {:.1}", total as f64 / buckets.len() as f64);
    
    Ok(())
}
