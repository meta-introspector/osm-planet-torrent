// Calculate which piece contains a given node ID
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <node_id> [node_id...]", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} 2824755486", args[0]);
        std::process::exit(1);
    }
    
    // From piece 1 analysis:
    // Piece 1 contains nodes ~20,933,784 to ~21,458,266
    // Range: ~524,482 nodes per piece
    let nodes_per_piece = 524_482;
    
    println!("📊 Estimated nodes per piece: {}", nodes_per_piece);
    println!("   (Based on piece 1 analysis)\n");
    
    for node_id_str in &args[1..] {
        let node_id: u64 = node_id_str.parse().expect("Invalid node ID");
        let piece_id = node_id / nodes_per_piece;
        
        println!("🎯 Node {}", node_id);
        println!("   → Piece {} (approx)", piece_id);
        println!("   → Byte offset: {} MB", piece_id * 4);
        
        // Check if we already have this piece
        let piece_file = format!("piece_{:07}_reconstructed.pbf", piece_id);
        if std::path::Path::new(&piece_file).exists() {
            println!("   ✓ Already downloaded!");
        } else {
            println!("   ⏳ Need to download");
        }
        println!();
    }
    
    println!("💡 To download a piece:");
    println!("   cargo run --bin fetch-piece -- <piece_id>");
}
