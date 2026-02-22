// Decode zlib compressed OSM PBF data
use std::fs;
use std::io::{Read, Cursor};
use flate2::read::ZlibDecoder;

fn main() -> anyhow::Result<()> {
    let pieces = vec![
        "piece_0000000_reconstructed.pbf",
        "piece_0000001_reconstructed.pbf",
        "piece_0000002_reconstructed.pbf",
        "piece_0000003_reconstructed.pbf",
        "piece_0000004_reconstructed.pbf",
        "piece_0021762_reconstructed.pbf",
    ];
    
    for piece_file in pieces {
        if !std::path::Path::new(piece_file).exists() {
            continue;
        }
        
        println!("\n📦 Decoding {}", piece_file);
        let data = fs::read(piece_file)?;
        
        // Find zlib compressed blocks (starts with 0x78 0x9c or 0x78 0xda)
        let mut pos = 0;
        let mut block_num = 0;
        
        while pos < data.len() - 1 {
            if (data[pos] == 0x78 && (data[pos+1] == 0x9c || data[pos+1] == 0xda)) {
                println!("   Found zlib block at offset {}", pos);
                
                // Try to decompress
                match decompress_zlib(&data[pos..]) {
                    Ok(decompressed) => {
                        println!("   ✓ Decompressed {} bytes -> {} bytes", 
                            data.len() - pos, decompressed.len());
                        
                        // Save decompressed data
                        let output = format!("{}_block_{}_decompressed.bin", 
                            piece_file.trim_end_matches(".pbf"), block_num);
                        fs::write(&output, &decompressed)?;
                        println!("   ✓ Saved to {}", output);
                        
                        // Show sample
                        let sample = String::from_utf8_lossy(&decompressed[..decompressed.len().min(200)]);
                        println!("   Sample: {:?}", &sample[..sample.len().min(100)]);
                        
                        block_num += 1;
                        break; // Move to next piece
                    }
                    Err(e) => {
                        println!("   ✗ Decompression failed: {}", e);
                    }
                }
            }
            pos += 1;
        }
    }
    
    Ok(())
}

fn decompress_zlib(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(Cursor::new(data));
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}
