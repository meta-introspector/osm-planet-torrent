// 24 Ramanujan Agents Walking Earth - Shared Memory Simulation
// Each agent loads OSM data from shmem blocks and walks influenced by FRACTRAN

use std::fs::File;
use std::io::{Read, Seek};
use serde_json::json;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

// 24 Ramanujan agents (24 = 2³ × 3, related to Monster)
const RAMANUJAN_AGENTS: [(&str, f64, f64, u32); 24] = [
    ("Ramanujan-α", 10.9617, 79.3881, 13668),   // Kumbakonam
    ("Ramanujan-β", 13.0827, 80.2707, 14137),   // Chennai
    ("Ramanujan-γ", 11.2189, 78.1677, 13645),   // Namakkal
    ("Ramanujan-δ", 51.5074, -0.1278, 16793),   // London
    ("Ramanujan-ε", 52.2053, 0.1218, 16945),    // Cambridge
    ("Ramanujan-ζ", 40.7128, -74.0060, 15234),  // New York
    ("Ramanujan-η", 35.6895, 139.6917, 17823),  // Tokyo
    ("Ramanujan-θ", 48.8566, 2.3522, 16234),    // Paris
    ("Ramanujan-ι", 51.5333, 9.9333, 16456),    // Göttingen
    ("Ramanujan-κ", 47.5596, 7.5886, 16123),    // Basel
    ("Ramanujan-λ", 40.3573, -74.6672, 15345),  // Princeton
    ("Ramanujan-μ", 47.4979, 19.0402, 16567),   // Budapest
    ("Ramanujan-ν", 53.4808, -2.2426, 16678),   // Manchester
    ("Ramanujan-ξ", 55.7558, 37.6173, 17234),   // Moscow
    ("Ramanujan-ο", 39.9042, 116.4074, 17456),  // Beijing
    ("Ramanujan-π", -33.8688, 151.2093, 18234), // Sydney
    ("Ramanujan-ρ", -23.5505, -46.6333, 14567), // São Paulo
    ("Ramanujan-σ", 19.4326, -99.1332, 15678),  // Mexico City
    ("Ramanujan-τ", 28.6139, 77.2090, 14234),   // Delhi
    ("Ramanujan-υ", 31.2304, 121.4737, 17567),  // Shanghai
    ("Ramanujan-φ", 1.3521, 103.8198, 13234),   // Singapore
    ("Ramanujan-χ", -1.2921, 36.8219, 13456),   // Nairobi
    ("Ramanujan-ψ", 30.0444, 31.2357, 14678),   // Cairo
    ("Ramanujan-ω", -34.6037, -58.3816, 14789), // Buenos Aires
];

struct RamanujanAgent {
    name: String,
    lat: f64,
    lon: f64,
    piece: u32,
    shard: u32,
    fractran_state: u128,
    steps: usize,
}

impl RamanujanAgent {
    fn new(name: &str, lat: f64, lon: f64, piece: u32) -> Self {
        let shard = piece % 71;
        Self {
            name: name.to_string(),
            lat,
            lon,
            piece,
            shard,
            fractran_state: 2,
            steps: 0,
        }
    }
    
    fn load_shmem_block(&mut self, file: &mut File) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Load 4KB block from shared memory (simulated via file)
        let piece_size = 4 * 1024 * 1024;
        let offset = self.piece as u64 * piece_size;
        
        file.seek(std::io::SeekFrom::Start(offset))?;
        let mut block = vec![0u8; 4096]; // 4KB shmem block
        file.read_exact(&mut block)?;
        
        Ok(block)
    }
    
    fn walk_step(&mut self, block: &[u8]) {
        // Apply FRACTRAN transformation based on block data
        let byte_sum: u64 = block.iter().take(16).map(|&b| b as u64).sum();
        let prime_idx = (byte_sum % 15) as usize;
        let prime = MONSTER_PRIMES[prime_idx];
        
        // FRACTRAN step
        self.fractran_state = self.fractran_state.wrapping_mul(prime as u128);
        self.shard = (self.fractran_state % 71) as u32;
        self.steps += 1;
        
        // Update position (random walk influenced by FRACTRAN)
        let delta_lat = ((self.fractran_state % 1000) as f64 - 500.0) / 100000.0;
        let delta_lon = (((self.fractran_state / 1000) % 1000) as f64 - 500.0) / 100000.0;
        
        self.lat += delta_lat;
        self.lon += delta_lon;
        
        // Wrap around Earth
        if self.lat > 90.0 { self.lat = 90.0; }
        if self.lat < -90.0 { self.lat = -90.0; }
        if self.lon > 180.0 { self.lon -= 360.0; }
        if self.lon < -180.0 { self.lon += 360.0; }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    
    eprintln!("🚶 24 Ramanujan Agents Walking Earth");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("Shared Memory: 4KB blocks per agent");
    eprintln!("FRACTRAN: Monster prime influenced walk");
    
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let mut file = File::open(planet_file)?;
    
    // Initialize 24 agents
    let mut agents: Vec<_> = RAMANUJAN_AGENTS.iter()
        .map(|(name, lat, lon, piece)| RamanujanAgent::new(name, *lat, *lon, *piece))
        .collect();
    
    eprintln!("\n🎯 Initializing {} agents...", agents.len());
    
    // Simulate 10 steps
    let num_steps = 10;
    let mut walk_history = Vec::new();
    
    for step in 0..num_steps {
        eprintln!("\n📍 Step {}/{}", step + 1, num_steps);
        
        let mut step_data = Vec::new();
        
        for agent in &mut agents {
            // Load shmem block
            let block = agent.load_shmem_block(&mut file)?;
            
            // Walk step
            agent.walk_step(&block);
            
            if step == 0 || step == num_steps - 1 {
                eprintln!("   {} at ({:.4}, {:.4}) shard {}", 
                    agent.name, agent.lat, agent.lon, agent.shard);
            }
            
            step_data.push(json!({
                "agent": agent.name,
                "position": [agent.lon, agent.lat],
                "shard": agent.shard,
                "fractran": agent.fractran_state.to_string(),
            }));
        }
        
        walk_history.push(json!({
            "step": step + 1,
            "agents": step_data,
        }));
    }
    
    // Calculate statistics
    let total_distance: f64 = agents.iter()
        .map(|a| (a.lat.powi(2) + a.lon.powi(2)).sqrt())
        .sum();
    
    let unique_shards: std::collections::HashSet<_> = agents.iter()
        .map(|a| a.shard)
        .collect();
    
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🎯 Simulation complete!");
    eprintln!("   Agents: {}", agents.len());
    eprintln!("   Steps: {}", num_steps);
    eprintln!("   Unique shards visited: {}", unique_shards.len());
    eprintln!("   Total distance: {:.2}", total_distance);
    
    let output = json!({
        "type": "Ramanujan24WalkersSimulation",
        "agents": agents.len(),
        "steps": num_steps,
        "shmem_block_size": 4096,
        "walk_history": walk_history,
        "final_positions": agents.iter().map(|a| json!({
            "name": a.name,
            "position": [a.lon, a.lat],
            "shard": a.shard,
            "fractran": a.fractran_state.to_string(),
            "steps": a.steps,
        })).collect::<Vec<_>>(),
        "statistics": {
            "unique_shards": unique_shards.len(),
            "total_distance": total_distance,
            "avg_fractran": agents.iter()
                .map(|a| a.fractran_state as f64)
                .sum::<f64>() / agents.len() as f64,
        },
        "speedrun_ms": start.elapsed().as_millis(),
    });
    
    let mut out = File::create("/tmp/ramanujan_24_walkers.json")?;
    serde_json::to_writer_pretty(&mut out, &output)?;
    
    eprintln!("\n✅ /tmp/ramanujan_24_walkers.json");
    eprintln!("⚡ {}ms", start.elapsed().as_millis());
    Ok(())
}
