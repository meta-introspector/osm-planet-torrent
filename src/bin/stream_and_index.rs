// Download OSM planet and index pieces as they arrive
// Real-time indexing - no need to wait for full 85 GB download!
use librqbit::Session;
use anyhow::Result;
use std::fs;
use std::path::Path;
use prost::Message;
use flate2::read::ZlibDecoder;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct PieceIndex {
    piece_id: u32,
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
    byte_offset: u64,
    indexed_at: String,
}

// Protobuf definitions (simplified)
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

fn index_piece(piece_path: &Path, piece_id: u32, byte_offset: u64) -> Result<PieceIndex> {
    let data = fs::read(piece_path)?;
    
    // Find first zlib block
    for pos in 0..data.len().saturating_sub(2) {
        if data[pos] == 0x78 && (data[pos+1] == 0x9c || data[pos+1] == 0xda) {
            // Try to decompress
            let mut decoder = ZlibDecoder::new(&data[pos..]);
            let mut decompressed = Vec::new();
            if decoder.read_to_end(&mut decompressed).is_ok() {
                // Parse PrimitiveBlock
                if let Ok(block) = PrimitiveBlock::decode(&decompressed[..]) {
                    return extract_metadata(block, piece_id, byte_offset);
                }
            }
        }
    }
    
    anyhow::bail!("Could not parse piece {}", piece_id)
}

fn extract_metadata(block: PrimitiveBlock, piece_id: u32, byte_offset: u64) -> Result<PieceIndex> {
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
    
    let str_table = block.stringtable.as_ref().unwrap();
    
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
                
                // Check for wikidata tags
                while kv_index < dense.keys_vals.len() {
                    let k = dense.keys_vals[kv_index];
                    kv_index += 1;
                    if k == 0 { break; }
                    
                    let v = dense.keys_vals[kv_index];
                    kv_index += 1;
                    
                    let key = String::from_utf8_lossy(&str_table.s[k as usize]);
                    if key == "wikidata" {
                        wikidata_count += 1;
                    }
                }
            }
        }
    }
    
    let center_lat = (min_lat + max_lat) / 2.0;
    let center_lon = (min_lon + max_lon) / 2.0;
    let monster_lat_block = (((center_lat + 90.0) / 180.0) * 71.0) as u8;
    let monster_lon_block = (((center_lon + 180.0) / 360.0) * 59.0) as u8;
    
    Ok(PieceIndex {
        piece_id,
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
        byte_offset,
        indexed_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let torrent_url = "https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf.torrent";
    let torrent_cache = "planet-latest.osm.pbf.torrent";
    let output_dir = "/mnt/data1/osm-planet";
    let index_file = "streaming_spatial_index.jsonl";
    
    println!("🌍 Streaming OSM planet download + real-time indexing");
    println!("📦 Torrent: {}", torrent_url);
    println!("💾 Output: {}", output_dir);
    println!("📊 Index: {}", index_file);
    println!();
    
    // Download and cache torrent file if not exists
    let torrent_bytes = if Path::new(torrent_cache).exists() {
        println!("✓ Using cached torrent file: {}", torrent_cache);
        fs::read(torrent_cache)?
    } else {
        println!("📥 Downloading torrent file...");
        let response = reqwest::get(torrent_url).await?;
        let bytes = response.bytes().await?;
        fs::write(torrent_cache, &bytes)?;
        println!("✓ Cached torrent file: {} ({} bytes)", torrent_cache, bytes.len());
        bytes.to_vec()
    };
    
    let session = Session::new(output_dir.into()).await?;
    let response = session.add_torrent(
        librqbit::AddTorrent::TorrentFileBytes(torrent_bytes.into()),
        None
    ).await?;
    
    let handle = response.into_handle()
        .ok_or_else(|| anyhow::anyhow!("Failed to get torrent handle"))?;
    
    println!("✓ Torrent started, indexing pieces as they download...\n");
    
    let piece_size = 4194304u64; // 4 MB
    let mut indexed_pieces = std::collections::HashSet::new();
    
    loop {
        let stats = handle.stats();
        let progress = stats.progress_bytes;
        let total = stats.total_bytes;
        
        // Calculate which pieces are complete
        let complete_pieces = progress / piece_size;
        
        // Index any new complete pieces
        for piece_id in 0..complete_pieces as u32 {
            if indexed_pieces.contains(&piece_id) {
                continue;
            }
            
            let piece_path = Path::new(output_dir).join("planet-latest.osm.pbf");
            if piece_path.exists() {
                // Try to index this piece
                match index_piece(&piece_path, piece_id, piece_id as u64 * piece_size) {
                    Ok(idx) => {
                        println!("✓ Indexed piece {}: nodes {} to {}, monster block ({}, {}), {} wikidata",
                            piece_id, idx.min_node_id, idx.max_node_id,
                            idx.monster_lat_block, idx.monster_lon_block, idx.wikidata_count);
                        
                        // Append to index file
                        let json = serde_json::to_string(&idx)?;
                        fs::write(index_file, format!("{}\n", json))?;
                        
                        indexed_pieces.insert(piece_id);
                    }
                    Err(e) => {
                        eprintln!("✗ Failed to index piece {}: {}", piece_id, e);
                    }
                }
            }
        }
        
        // Show progress
        let percent = if total > 0 {
            (progress as f64 / total as f64) * 100.0
        } else { 0.0 };
        
        let speed_info = if let Some(live) = &stats.live {
            format!("{}", live.download_speed)
        } else {
            "N/A".to_string()
        };
        
        println!("📊 Download: {:.2}% | Indexed: {} pieces | Speed: {}",
            percent, indexed_pieces.len(), speed_info);
        
        if progress >= total && total > 0 {
            break;
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
    
    println!("\n✅ Download and indexing complete!");
    println!("📊 Total pieces indexed: {}", indexed_pieces.len());
    println!("💾 Index saved to: {}", index_file);
    
    Ok(())
}
