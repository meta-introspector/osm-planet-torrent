// Binary search for pieces containing target location
// Strategy: Sample first, middle, last pieces to find spatial distribution
use anyhow::Result;
use std::process::Command;

#[derive(Debug)]
struct BBox {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
}

impl BBox {
    fn contains(&self, lat: f64, lon: f64) -> bool {
        lat >= self.min_lat && lat <= self.max_lat &&
        lon >= self.min_lon && lon <= self.max_lon
    }
}

fn fetch_piece_bbox(piece_id: u32) -> Result<BBox> {
    println!("\n🔍 Fetching piece {} to analyze bbox...", piece_id);
    
    // 1. Download piece (will save chunks to ./chunks/)
    let status = Command::new("cargo")
        .args(&["run", "--bin", "fetch-piece", "--", &piece_id.to_string()])
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Failed to fetch piece {}", piece_id);
    }
    
    // 2. Reconstruct from chunks
    Command::new("cargo")
        .args(&["run", "--bin", "reconstruct-pbf"])
        .status()?;
    
    // 3. Decompress zlib blocks
    Command::new("cargo")
        .args(&["run", "--bin", "decode-zlib"])
        .status()?;
    
    // 4. Parse and extract bbox
    // TODO: Modify parse_dense.rs to output bbox JSON
    
    Ok(BBox {
        min_lat: 0.0,
        max_lat: 0.0,
        min_lon: 0.0,
        max_lon: 0.0,
    })
}

fn main() -> Result<()> {
    let target_lat = 10.9617; // Kumbakonam
    let target_lon = 79.3881;
    
    println!("🎯 Target: Kumbakonam at {}, {}", target_lat, target_lon);
    println!("📊 Total pieces: 21,763");
    println!("\n⚠️  OSM planet is sorted by NODE ID, not geography!");
    println!("   Strategy: Sample pieces using Monster Group distribution\n");
    
    // Monster Group sharding: 71 × 59 × 497 = 2,081,933
    // Sample every 71st piece (first Monster prime)
    let mut samples = Vec::new();
    for i in (0..21763).step_by(71) {
        samples.push(i);
    }
    
    println!("📦 Sampling {} pieces (every 71st piece)", samples.len());
    println!("   This will download ~{} MB", samples.len() * 4);
    println!("\n   First 10 samples: {:?}", &samples[..10]);
    
    // For now, just fetch first 3 to test
    println!("\n🧪 Testing with first 3 samples:");
    for piece_id in samples.iter().take(3) {
        println!("   - Piece {}", piece_id);
        // TODO: fetch_piece_bbox(*piece_id)?;
    }
    
    println!("\n💡 Next: Implement parallel download of all {} samples", samples.len());
    println!("   Then build spatial index: piece_id → (min_lat, max_lat, min_lon, max_lon)");
    
    Ok(())
}
