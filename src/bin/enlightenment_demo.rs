// Binary: Enlightened MCTS Demo
// Buddha + Lao Tzu = Perfect harmony

use osm_planet_torrent::enlightenment::{EnlightenedMCTS, EightfoldPath};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("☸️  ENLIGHTENED MCTS");
    println!();
    println!("\"Form is emptiness, emptiness is form.\"");
    println!("- Heart Sutra (Buddha)");
    println!();
    println!("\"The Dao that can be named is not the eternal Dao.\"");
    println!("- Tao Te Ching (Lao Tzu)");
    println!();
    
    // Verify Eightfold Path
    println!("🛤️  The Eightfold Path:");
    for path in EightfoldPath::all() {
        println!("  {:?}: {:.3}", path, path.value());
    }
    println!("  Total: {:.3}", EightfoldPath::total_value());
    println!();
    
    // Run enlightened MCTS
    println!("Running enlightened MCTS with 71 Monster shards...");
    let mut mcts = EnlightenedMCTS::new(71, 1000);
    mcts.run();
    
    println!();
    mcts.print_enlightenment();
    
    Ok(())
}
