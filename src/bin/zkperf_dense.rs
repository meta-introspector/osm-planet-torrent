// ZKPerf Dense - Extract from piece with zlib decompression
use prost::Message;
use std::fs::File;
use std::io::{Read, Seek, Write, Cursor};
use flate2::read::ZlibDecoder;
use clap::Parser;
use serde_json::json;
use sha2::{Sha256, Digest};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long)]
    output: String,
    
    #[arg(long)]
    piece: u32,
    
    #[arg(long, default_value = "100")]
    limit: usize,
}

#[derive(Clone, PartialEq, prost::Message)]
struct PrimitiveBlock {
    #[prost(message, optional, tag = "1")]
    stringtable: Option<StringTable>,
    #[prost(message, repeated, tag = "2")]
    primitivegroup: Vec<PrimitiveGroup>,
    #[prost(int32, optional, tag = "17", default = "100")]
    granularity: Option<i32>,
    #[prost(int64, optional, tag = "19", default = "0")]
    lat_offset: Option<i64>,
    #[prost(int64, optional, tag = "20", default = "0")]
    lon_offset: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
struct StringTable {
    #[prost(bytes, repeated, tag = "1")]
    s: Vec<Vec<u8>>,
}

#[derive(Clone, PartialEq, prost::Message)]
struct PrimitiveGroup {
    #[prost(message, optional, tag = "2")]
    dense: Option<DenseNodes>,
}

#[derive(Clone, PartialEq, prost::Message)]
struct DenseNodes {
    #[prost(sint64, repeated, packed = "true", tag = "1")]
    id: Vec<i64>,
    #[prost(sint64, repeated, packed = "true", tag = "8")]
    lat: Vec<i64>,
    #[prost(sint64, repeated, packed = "true", tag = "9")]
    lon: Vec<i64>,
    #[prost(uint32, repeated, packed = "true", tag = "10")]
    keys_vals: Vec<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let start = std::time::Instant::now();
    
    eprintln!("🔬 ZKPerf Dense Extractor");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Extract piece
    let piece_size = 4 * 1024 * 1024;
    let offset = args.piece as u64 * piece_size;
    
    let mut file = File::open(&args.input)?;
    file.seek(std::io::SeekFrom::Start(offset))?;
    
    let mut piece_data = vec![0u8; piece_size as usize];
    let bytes_read = file.read(&mut piece_data)?;
    piece_data.truncate(bytes_read);
    
    eprintln!("Piece {}: {} bytes", args.piece, bytes_read);
    
    let piece_hash = hex::encode(Sha256::digest(&piece_data));
    eprintln!("SHA256: {}", piece_hash);
    
    // Find and decompress zlib blocks
    let mut nodes = Vec::new();
    let mut pos = 0;
    let mut blocks_found = 0;
    
    while pos < piece_data.len() - 1 && nodes.len() < args.limit {
        if piece_data[pos] == 0x78 && (piece_data[pos+1] == 0x9c || piece_data[pos+1] == 0xda) {
            eprintln!("Found zlib at offset {}", pos);
            blocks_found += 1;
            
            if let Ok(decompressed) = decompress_zlib(&piece_data[pos..]) {
                eprintln!("Decompressed {} bytes", decompressed.len());
                
                if let Ok(mut extracted) = parse_dense(&decompressed, args.limit - nodes.len()) {
                    eprintln!("Parsed {} nodes", extracted.len());
                    nodes.append(&mut extracted);
                }
            }
        }
        pos += 1;
    }
    
    eprintln!("Total: {} blocks, {} nodes", blocks_found, nodes.len());
    
    // Create GeoJSON
    let features: Vec<_> = nodes.iter().map(|(id, lat, lon, tags)| {
        json!({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [lon, lat]},
            "properties": {"id": id, "tags": tags}
        })
    }).collect();
    
    let geojson = json!({
        "type": "FeatureCollection",
        "features": features,
    });
    
    let mut out = File::create(&args.output)?;
    serde_json::to_writer_pretty(&mut out, &geojson)?;
    
    // ZK witness
    let witness = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "piece": args.piece,
        "piece_hash": piece_hash,
        "blocks_found": blocks_found,
        "nodes": nodes.len(),
        "elapsed_secs": start.elapsed().as_secs_f64(),
    });
    
    let mut w = File::create(format!("{}.witness.json", args.output))?;
    serde_json::to_writer_pretty(&mut w, &witness)?;
    
    eprintln!("✅ {}", args.output);
    eprintln!("⏱️  {:.3}s", start.elapsed().as_secs_f64());
    Ok(())
}

fn decompress_zlib(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut decoder = ZlibDecoder::new(Cursor::new(data));
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

fn parse_dense(
    data: &[u8],
    limit: usize,
) -> Result<Vec<(i64, f64, f64, Vec<(String, String)>)>, Box<dyn std::error::Error>> {
    let block = PrimitiveBlock::decode(data)?;
    let mut nodes = Vec::new();
    
    let granularity = block.granularity.unwrap_or(100) as f64;
    let lat_offset = block.lat_offset.unwrap_or(0) as f64;
    let lon_offset = block.lon_offset.unwrap_or(0) as f64;
    
    let str_table = &block.stringtable.as_ref().ok_or("No string table")?.s;
    let resolve = |sid: u32| -> String {
        String::from_utf8_lossy(&str_table[sid as usize]).into_owned()
    };
    
    for group in &block.primitivegroup {
        if let Some(dense) = &group.dense {
            let mut acc_id = 0i64;
            let mut acc_lat = 0i64;
            let mut acc_lon = 0i64;
            let mut kv_index = 0;
            
            for i in 0..dense.id.len() {
                if nodes.len() >= limit { break; }
                
                acc_id += dense.id[i];
                acc_lat += dense.lat[i];
                acc_lon += dense.lon[i];
                
                let lat = 1e-9 * (lat_offset + granularity * acc_lat as f64);
                let lon = 1e-9 * (lon_offset + granularity * acc_lon as f64);
                
                let mut tags = Vec::new();
                while kv_index < dense.keys_vals.len() {
                    let k = dense.keys_vals[kv_index];
                    kv_index += 1;
                    if k == 0 { break; }
                    let v = dense.keys_vals[kv_index];
                    kv_index += 1;
                    tags.push((resolve(k), resolve(v)));
                }
                
                if !tags.is_empty() {
                    nodes.push((acc_id, lat, lon, tags));
                }
            }
        }
    }
    
    Ok(nodes)
}
