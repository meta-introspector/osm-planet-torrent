// FRACTRAN OSM Speedrun
use std::fs::File;
use std::io::{Read, Seek, Write};
use clap::Parser;
use serde_json::json;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long)]
    output: String,
    
    #[arg(long)]
    piece: u32,
}

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let start = std::time::Instant::now();
    
    eprintln!("🔢 FRACTRAN OSM Speedrun");
    
    let piece_size = 4 * 1024 * 1024;
    let offset = args.piece as u64 * piece_size;
    
    let mut file = File::open(&args.input)?;
    file.seek(std::io::SeekFrom::Start(offset))?;
    
    let mut data = vec![0u8; piece_size as usize];
    let bytes_read = file.read(&mut data)?;
    data.truncate(bytes_read);
    
    eprintln!("Piece {}: {} bytes", args.piece, bytes_read);
    
    let state = encode_fractran(&data);
    let shard = (state % 71) as u64;
    
    eprintln!("FRACTRAN: {} (shard {})", state, shard);
    
    let triples = generate_triples(&data, args.piece, shard, 10);
    
    let output = json!({
        "type": "SemanticGraph",
        "piece": args.piece,
        "fractran_state": state.to_string(),
        "shard": shard,
        "triples": triples,
        "speedrun_ms": start.elapsed().as_millis(),
    });
    
    let mut out = File::create(&args.output)?;
    serde_json::to_writer_pretty(&mut out, &output)?;
    
    eprintln!("✅ {}ms", start.elapsed().as_millis());
    Ok(())
}

fn encode_fractran(data: &[u8]) -> u128 {
    let mut state: u128 = 2;
    let mut count = 0;
    
    for &byte in data.iter() {
        if byte > 0 && count < 15 {
            state *= MONSTER_PRIMES[count].pow((byte % 8) as u32) as u128;
            count += 1;
        }
    }
    
    state
}

fn generate_triples(
    data: &[u8],
    piece: u32,
    shard: u64,
    limit: usize,
) -> Vec<serde_json::Value> {
    let mut triples = Vec::new();
    
    for i in 0..limit.min(data.len() / 4) {
        let offset = i * 4;
        if offset + 4 > data.len() { break; }
        
        let value = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        
        if value > 0 {
            triples.push(json!({
                "s": format!("osm:piece/{}/offset/{}", piece, offset),
                "p": "fractran:encodes",
                "o": value,
                "shard": shard,
                "prime": MONSTER_PRIMES[i % 15],
            }));
        }
    }
    
    triples
}
