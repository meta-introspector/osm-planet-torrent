// Split OSM planet file geographically into 71×59×47 buckets
// Read entire planet, assign each node to geographic bucket, write new PBF files

use anyhow::Result;
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use crossbeam::thread;

// Geographic bucket calculation
fn calculate_geo_bucket(lat: f64, lon: f64) -> (u8, u8) {
    // Lat: -90 to +90 → 71 blocks
    let lat_block = (((lat + 90.0) / 180.0) * 71.0).clamp(0.0, 70.0) as u8;
    
    // Lon: -180 to +180 → 59 blocks
    let lon_block = (((lon + 180.0) / 360.0) * 59.0).clamp(0.0, 58.0) as u8;
    
    (lat_block, lon_block)
}

// Size bucket based on node ID (temporal strata)
fn calculate_size_bucket(node_id: i64) -> u8 {
    // Split into 47 temporal strata
    // Old IDs = stable features, New IDs = recent edits
    let log_id = if node_id > 0 {
        (node_id as f64).log10()
    } else {
        0.0
    };
    
    // Map log(ID) to 0-46
    ((log_id / 12.0) * 47.0).clamp(0.0, 46.0) as u8
}

#[derive(Debug)]
struct NodeData {
    id: i64,
    lat: f64,
    lon: f64,
    tags: Vec<(String, String)>,
}

fn main() -> Result<()> {
    println!("🗺️  Geographic Split of OSM Planet");
    println!("📦 Target: 71 × 59 × 47 = 196,877 geographic buckets");
    println!();
    
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let output_dir = "geo_split";
    
    fs::create_dir_all(&output_dir)?;
    
    println!("📊 Strategy:");
    println!("  1. Read entire planet file (86 GB)");
    println!("  2. For each node: calculate (lat_block, lon_block, size_block)");
    println!("  3. Buffer nodes by bucket");
    println!("  4. Write bucket files: bucket_<lat>_<lon>_<size>.jsonl");
    println!();
    
    // Open planet file
    println!("📖 Reading planet file with 24 parallel threads...");
    let reader = ElementReader::from_path(planet_file)?;
    
    // Shared buckets with mutex
    let buckets: Arc<Mutex<HashMap<(u8, u8, u8), Vec<NodeData>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    
    let total_nodes = Arc::new(Mutex::new(0u64));
    let nodes_with_wikidata = Arc::new(Mutex::new(0u64));
    
    // Process with 24 threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(24)
        .build_global()
        .unwrap();
    
    // Process all elements in parallel
    for element in reader.par_map_reduce(
        |element| {
            let mut nodes = Vec::new();
            
            match element {
                Element::Node(node) => {
                    let lat = node.lat();
                    let lon = node.lon();
                    let id = node.id();
                    
                    let mut tags = Vec::new();
                    let mut has_wikidata = false;
                    
                    for (key, val) in node.tags() {
                        tags.push((key.to_string(), val.to_string()));
                        if key == "wikidata" {
                            has_wikidata = true;
                        }
                    }
                    
                    let (lat_block, lon_block) = calculate_geo_bucket(lat, lon);
                    let size_block = calculate_size_bucket(id);
                    
                    nodes.push((
                        (lat_block, lon_block, size_block),
                        NodeData { id, lat, lon, tags },
                        has_wikidata,
                    ));
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
        let mut buckets_lock = buckets.lock().unwrap();
        let mut total_lock = total_nodes.lock().unwrap();
        let mut wikidata_lock = nodes_with_wikidata.lock().unwrap();
        
        for (bucket_key, node, has_wikidata) in element {
            buckets_lock.entry(bucket_key).or_insert_with(Vec::new).push(node);
            *total_lock += 1;
            if has_wikidata {
                *wikidata_lock += 1;
            }
        }
        
        if *total_lock % 1_000_000 == 0 {
            println!("  Processed {} million nodes, {} buckets, {} with wikidata",
                *total_lock / 1_000_000, buckets_lock.len(), *wikidata_lock);
        }
    }
    
    let buckets = Arc::try_unwrap(buckets).unwrap().into_inner().unwrap();
    let total_nodes = *total_nodes.lock().unwrap();
    let nodes_with_wikidata = *nodes_with_wikidata.lock().unwrap();
    
    println!();
    println!("✅ Processed {} nodes into {} buckets", total_nodes, buckets.len());
    println!("   Nodes with wikidata: {}", nodes_with_wikidata);
    println!();
    
    // Write buckets to files
    println!("💾 Writing bucket files...");
    
    let mut bucket_count = 0;
    for ((lat_block, lon_block, size_block), nodes) in buckets {
        let bucket_file = format!("{}/bucket_{}_{}_{}_{}.jsonl",
            output_dir, lat_block, lon_block, size_block, nodes.len());
        
        let mut file = File::create(bucket_file)?;
        
        for node in nodes {
            let json = serde_json::json!({
                "id": node.id,
                "lat": node.lat,
                "lon": node.lon,
                "tags": node.tags,
            });
            writeln!(file, "{}", json)?;
        }
        
        bucket_count += 1;
        
        if bucket_count % 100 == 0 {
            println!("  Written {} buckets", bucket_count);
        }
    }
    
    println!();
    println!("✅ Geographic split complete!");
    println!("📊 Total buckets: {}", bucket_count);
    println!("💾 Output: {}/", output_dir);
    println!();
    println!("🎯 Now you can:");
    println!("  - Query by location → get specific bucket");
    println!("  - Download only needed buckets");
    println!("  - Share buckets on HuggingFace");
    
    Ok(())
}
