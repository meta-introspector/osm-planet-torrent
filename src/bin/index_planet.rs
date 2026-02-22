// Index the complete OSM planet file by reading it sequentially
// Extract metadata from each 4MB piece and build spatial index
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use serde::{Serialize, Deserialize};
use prost::Message;
use flate2::read::ZlibDecoder;

#[derive(Debug, Serialize, Deserialize)]
struct PieceIndex {
    piece_id: u32,
    byte_offset: u64,
    min_node_id: u64,
    max_node_id: u64,
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    node_count: usize,
    wikidata_count: usize,
    monster_lat_block: u8,
    monster_lon_block: u8,
    bucket_id: u8,
    bucket_lat: u8,
    bucket_lon: u8,
}

// Simplified protobuf structs
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

fn index_piece(data: &[u8], piece_id: u32, byte_offset: u64) -> Option<PieceIndex> {
    // Find first zlib block
    for pos in 0..data.len().saturating_sub(2) {
        if data[pos] == 0x78 && (data[pos+1] == 0x9c || data[pos+1] == 0xda) {
            let mut decoder = ZlibDecoder::new(&data[pos..]);
            let mut decompressed = Vec::new();
            if decoder.read_to_end(&mut decompressed).is_ok() {
                if let Ok(block) = PrimitiveBlock::decode(&decompressed[..]) {
                    return extract_metadata(block, piece_id, byte_offset);
                }
            }
        }
    }
    None
}

fn extract_metadata(block: PrimitiveBlock, piece_id: u32, byte_offset: u64) -> Option<PieceIndex> {
    let granularity = block.granularity.unwrap_or(100) as f64;
    let lat_offset = block.lat_offset.unwrap_or(0) as f64;
    let lon_offset = block.lon_offset.unwrap_or(0) as f64;
    
    let mut min_node_id = u64::MAX;
    let mut max_node_id = 0u64;
    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;
    let mut min_lon = f64::MAX;
    let mut max_lon = f64::MIN;
    let mut node_count = 0;
    let mut wikidata_count = 0;
    
    let str_table = block.stringtable.as_ref()?;
    
    for group in &block.primitivegroup {
        if let Some(dense) = &group.dense {
            node_count += dense.id.len();
            
            let mut acc_id = 0i64;
            let mut acc_lat = 0i64;
            let mut acc_lon = 0i64;
            let mut kv_index = 0;
            
            for i in 0..dense.id.len() {
                acc_id += dense.id[i];
                acc_lat += dense.lat[i];
                acc_lon += dense.lon[i];
                
                min_node_id = min_node_id.min(acc_id as u64);
                max_node_id = max_node_id.max(acc_id as u64);
                
                let lat_deg = 1e-9 * (lat_offset + granularity * acc_lat as f64);
                let lon_deg = 1e-9 * (lon_offset + granularity * acc_lon as f64);
                
                min_lat = min_lat.min(lat_deg);
                max_lat = max_lat.max(lat_deg);
                min_lon = min_lon.min(lon_deg);
                max_lon = max_lon.max(lon_deg);
                
                while kv_index < dense.keys_vals.len() {
                    let k = dense.keys_vals[kv_index];
                    kv_index += 1;
                    if k == 0 { break; }
                    
                    kv_index += 1;
                    
                    let key = String::from_utf8_lossy(&str_table.s[k as usize]);
                    if key == "wikidata" {
                        wikidata_count += 1;
                    }
                }
            }
        }
    }
    
    if node_count == 0 {
        return None;
    }
    
    let center_lat = (min_lat + max_lat) / 2.0;
    let center_lon = (min_lon + max_lon) / 2.0;
    let monster_lat_block = (((center_lat + 90.0) / 180.0) * 71.0) as u8;
    let monster_lon_block = (((center_lon + 180.0) / 360.0) * 59.0) as u8;
    
    // Calculate modular buckets
    let center_id = (min_node_id + max_node_id) / 2;
    let center_lat = (min_lat + max_lat) / 2.0;
    let center_lon = (min_lon + max_lon) / 2.0;
    
    let bucket_id = (center_id % 71) as u8;
    let bucket_lat = ((((center_lat + 90.0) * 100.0) as i64) % 41) as u8;
    let bucket_lon = ((((center_lon + 180.0) * 100.0) as i64) % 31) as u8;

    Some(PieceIndex {
        piece_id,
        byte_offset,
        min_node_id,
        max_node_id,
        min_lat,
        max_lat,
        min_lon,
        max_lon,
        node_count,
        wikidata_count,
        monster_lat_block,
        monster_lon_block,
        bucket_id,
        bucket_lat,
        bucket_lon,
    })
}

fn main() -> Result<()> {
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let index_file = "complete_spatial_index.jsonl";
    let piece_size = 4_194_304u64; // 4 MB
    
    println!("🗺️  Indexing OSM planet file");
    println!("📦 File: {}", planet_file);
    println!("📊 Piece size: {} MB", piece_size / 1024 / 1024);
    println!("💾 Index: {}", index_file);
    println!();
    
    let mut file = File::open(planet_file)?;
    let file_size = file.metadata()?.len();
    let total_pieces = (file_size + piece_size - 1) / piece_size;
    
    println!("📈 Total size: {} GB", file_size / 1024 / 1024 / 1024);
    println!("📈 Total pieces: {}", total_pieces);
    println!();
    
    let mut index_output = File::create(index_file)?;
    let mut indexed_count = 0;
    
    for piece_id in 0..total_pieces as u32 {
        let byte_offset = piece_id as u64 * piece_size;
        file.seek(SeekFrom::Start(byte_offset))?;
        
        let mut buffer = vec![0u8; piece_size as usize];
        let bytes_read = file.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        
        if let Some(idx) = index_piece(&buffer, piece_id, byte_offset) {
            let json = serde_json::to_string(&idx)?;
            use std::io::Write;
            writeln!(index_output, "{}", json)?;
            indexed_count += 1;
            
            if piece_id % 100 == 0 {
                let progress = (piece_id as f64 / total_pieces as f64) * 100.0;
                println!("📊 Progress: {:.1}% | Piece {} | Indexed: {} | Monster block: ({}, {}) | {} wikidata",
                    progress, piece_id, indexed_count, idx.monster_lat_block, idx.monster_lon_block, idx.wikidata_count);
            }
        }
    }
    
    println!();
    println!("✅ Indexing complete!");
    println!("📊 Total pieces indexed: {}/{}", indexed_count, total_pieces);
    println!("💾 Index saved to: {}", index_file);
    
    Ok(())
}
