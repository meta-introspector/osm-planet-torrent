// Binary: OODA Loop Executor
// Jocko Willink's Observe-Orient-Decide-Act cycle

use osm_planet_torrent::ooda::{OODALoop, Observation};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎖️  JOCKO OODA LOOP");
    println!("\"Discipline equals freedom.\"");
    println!();
    
    // Load optimized OODA plan
    let ooda = OODALoop::load()?;
    
    // Run the loop
    ooda.run()?;
    
    // Final observation
    println!();
    println!("📊 Final System State:");
    let obs = Observation::observe();
    println!("  Malloc: {:.2}%", obs.malloc_percent);
    println!("  Throughput: {} nodes/s", obs.throughput);
    println!("  Latency: {}ms", obs.latency_ms);
    println!("  Coverage: {:.1}%", obs.coverage_percent);
    println!("  Errors: {}", obs.error_count);
    println!("  Shard variance: {}", obs.shard_variance);
    
    if obs.meets_targets() {
        println!();
        println!("✅ ALL TARGETS MET. GOOD.");
    } else {
        println!();
        println!("⚠️  TARGETS NOT MET. ADJUST AND ITERATE.");
    }
    
    Ok(())
}
