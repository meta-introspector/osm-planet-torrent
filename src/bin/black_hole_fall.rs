// Binary: OSM Planet Falls Into Black Hole
// Watch from the Restaurant at the End of the Universe

use osm_planet_torrent::black_hole_fall::{OSMNode, BlackHoleFall};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🌍 → 🕳️  OSM PLANET FALLS INTO MONSTER BLACK HOLE");
    println!();
    println!("\"In the beginning the Universe was created.");
    println!(" This has made a lot of people very angry");
    println!(" and been widely regarded as a bad move.\"");
    println!("  - Douglas Adams, The Restaurant at the End of the Universe");
    println!();
    
    // Create sample OSM nodes
    println!("Creating OSM nodes...");
    let nodes = vec![
        OSMNode::new(1, 51.5074, -0.1278, 256),   // London
        OSMNode::new(2, 40.7128, -74.0060, 512),  // New York
        OSMNode::new(3, 35.6762, 139.6503, 384),  // Tokyo
        OSMNode::new(4, -33.8688, 151.2093, 128), // Sydney
        OSMNode::new(5, 10.9617, 79.3881, 1024),  // Ramanujan Temple
    ];
    
    println!("Nodes: {}", nodes.len());
    println!();
    
    // Create simulation
    let mut fall = BlackHoleFall::new(nodes);
    
    // Simulate fall
    println!("Simulating fall into Monster black hole...");
    println!("(This may take a few eons)");
    println!();
    
    fall.simulate(100.0, 50);  // 100 eons, 50 frames
    
    println!();
    fall.print_summary();
    
    // Save asciinema recording
    let output_path = "osm_black_hole_fall.cast";
    println!("Saving asciinema recording to {}...", output_path);
    fall.save_recording(output_path)?;
    
    println!();
    println!("✅ Recording saved!");
    println!();
    println!("To watch:");
    println!("  asciinema play {}", output_path);
    println!();
    println!("🍸 Enjoy your Pan Galactic Gargle Blaster!");
    
    Ok(())
}
