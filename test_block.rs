use std::fs;

fn main() {
    // These are raw PrimitiveBlock protobuf data (already decompressed)
    let blocks = vec![
        "piece_0000001_reconstructed_block_0_decompressed.bin",
        "piece_0000004_reconstructed_block_0_decompressed.bin",
    ];
    
    for block_file in blocks {
        println!("\n📦 {}", block_file);
        let data = fs::read(block_file).unwrap();
        println!("   Size: {} bytes", data.len());
        
        // Show first 200 bytes as string
        let sample = String::from_utf8_lossy(&data[..data.len().min(200)]);
        println!("   Sample: {}", &sample[..sample.len().min(100)]);
    }
}
