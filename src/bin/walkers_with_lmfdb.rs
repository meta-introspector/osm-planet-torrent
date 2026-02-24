// 24 Ramanujan Walkers + LMFDB/OEIS/Wikidata nodes to discover
use std::fs::File;
use std::io::Read;
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    
    eprintln!("🚶 24 Ramanujan Walkers + Math Database Discovery");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Load LMFDB shards
    eprintln!("\n📊 Loading LMFDB shards from spool...");
    let lmfdb_path = "/mnt/data1/spool/experiments_monster/lmfdb_71_shards.json";
    let mut lmfdb_file = File::open(lmfdb_path)?;
    let mut lmfdb_data = String::new();
    lmfdb_file.read_to_string(&mut lmfdb_data)?;
    let lmfdb: Value = serde_json::from_str(&lmfdb_data)?;
    
    let shards = lmfdb["shards"].as_object().ok_or("No shards")?;
    eprintln!("   Found {} LMFDB shards", shards.len());
    
    // Count total functions
    let total_functions: usize = shards.values()
        .map(|v| v.as_array().map(|a| a.len()).unwrap_or(0))
        .sum();
    eprintln!("   Total LMFDB functions: {}", total_functions);
    
    // Create discoverable nodes from LMFDB
    let mut discoverable_nodes = Vec::new();
    
    for (shard_id, items) in shards.iter() {
        let shard_num: u32 = shard_id.parse().unwrap_or(0);
        
        for item in items.as_array().unwrap_or(&vec![]) {
            let name = item["name"].as_str().unwrap_or("unknown");
            let file = item["file"].as_str().unwrap_or("");
            let has_71 = item["has_71"].as_bool().unwrap_or(false);
            
            if has_71 {
                discoverable_nodes.push(json!({
                    "source": "LMFDB",
                    "shard": shard_num,
                    "name": name,
                    "file": file,
                    "has_71": true,
                    "type": "function",
                }));
            }
        }
    }
    
    eprintln!("   Discoverable nodes with 71: {}", discoverable_nodes.len());
    
    // Simulate 24 walkers discovering nodes
    eprintln!("\n🚶 Simulating 24 walkers...");
    
    let mut discoveries = Vec::new();
    let walker_names = vec![
        "Ramanujan-α", "Ramanujan-β", "Ramanujan-γ", "Ramanujan-δ",
        "Ramanujan-ε", "Ramanujan-ζ", "Ramanujan-η", "Ramanujan-θ",
        "Ramanujan-ι", "Ramanujan-κ", "Ramanujan-λ", "Ramanujan-μ",
        "Ramanujan-ν", "Ramanujan-ξ", "Ramanujan-ο", "Ramanujan-π",
        "Ramanujan-ρ", "Ramanujan-σ", "Ramanujan-τ", "Ramanujan-υ",
        "Ramanujan-φ", "Ramanujan-χ", "Ramanujan-ψ", "Ramanujan-ω",
    ];
    
    for (i, walker) in walker_names.iter().enumerate() {
        let target_shard = (i * 3) % 71;
        
        // Find nodes in this shard
        let found: Vec<_> = discoverable_nodes.iter()
            .filter(|n| n["shard"].as_u64().unwrap_or(0) == target_shard as u64)
            .take(3)
            .collect();
        
        if !found.is_empty() {
            eprintln!("   {} discovered {} nodes in shard {}", 
                walker, found.len(), target_shard);
            
            discoveries.push(json!({
                "walker": walker,
                "shard": target_shard,
                "discovered": found,
            }));
        }
    }
    
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🎯 Discovery complete!");
    eprintln!("   Walkers: {}", walker_names.len());
    eprintln!("   Total discoveries: {}", discoveries.len());
    eprintln!("   LMFDB nodes found: {}", discoverable_nodes.len());
    
    let output = json!({
        "type": "WalkersWithLMFDBDiscovery",
        "walkers": walker_names.len(),
        "lmfdb_shards": shards.len(),
        "lmfdb_functions": total_functions,
        "discoverable_nodes": discoverable_nodes.len(),
        "discoveries": discoveries,
        "speedrun_ms": start.elapsed().as_millis(),
    });
    
    let mut out = File::create("/tmp/walkers_with_lmfdb.json")?;
    serde_json::to_writer_pretty(&mut out, &output)?;
    
    eprintln!("\n✅ /tmp/walkers_with_lmfdb.json");
    eprintln!("⚡ {}ms", start.elapsed().as_millis());
    Ok(())
}
