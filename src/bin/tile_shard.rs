// Extract admin boundaries and tile-shard OSM planet
// Level 1: Extract admin_level tags (2=country, 4=state, 6=city)
// Level 2: Shard by tile (Z=8: 256×256)
// Level 3: Within tile, shard by node_id % 71
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crossbeam::channel::{bounded, Sender, Receiver};
use prost::Message;
use flate2::read::ZlibDecoder;

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

struct Node {
    id: i64,
    lat: f64,
    lon: f64,
    admin_level: Option<u8>,
    name: String,
}

fn main() -> Result<()> {
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    
    println!("🗺️  Tile-based sharding with admin extraction (24 workers)");
    std::fs::create_dir_all("tiles")?;
    std::fs::create_dir_all("admin")?;
    
    let mut file = File::open(planet_file)?;
    let file_size = file.metadata()?.len();
    let piece_size = 4_194_304u64;
    let total_pieces = (file_size + piece_size - 1) / piece_size;
    
    println!("📈 {} GB, {} pieces", file_size / 1_000_000_000, total_pieces);
    
    let (piece_tx, piece_rx) = bounded(48);
    let (node_tx, node_rx) = bounded(100_000);
    
    let tiles = Arc::new(Mutex::new(HashMap::<(u8, u8, u8), File>::new()));
    let admin_file = Arc::new(Mutex::new(File::create("admin/boundaries.jsonl")?));
    
    // Reader
    let reader = std::thread::spawn(move || -> Result<()> {
        for piece_id in 0..total_pieces as u32 {
            let mut buf = vec![0u8; piece_size as usize];
            let n = file.read(&mut buf)?;
            buf.truncate(n);
            piece_tx.send((piece_id, buf)).ok();
        }
        Ok(())
    });
    
    // 24 parsers
    let mut parsers = vec![];
    for _ in 0..24 {
        let rx = piece_rx.clone();
        let tx = node_tx.clone();
        
        parsers.push(std::thread::spawn(move || {
            while let Ok((_, buf)) = rx.recv() {
                for pos in 0..buf.len().saturating_sub(2) {
                    if buf[pos] == 0x78 && (buf[pos+1] == 0x9c || buf[pos+1] == 0xda) {
                        let mut dec = ZlibDecoder::new(&buf[pos..]);
                        let mut data = Vec::new();
                        if dec.read_to_end(&mut data).is_ok() {
                            if let Ok(block) = PrimitiveBlock::decode(&data[..]) {
                                let g = block.granularity.unwrap_or(100) as f64;
                                let lo = block.lat_offset.unwrap_or(0) as f64;
                                let oo = block.lon_offset.unwrap_or(0) as f64;
                                
                                let str_table = block.stringtable.as_ref().map(|s| &s.s);
                                
                                for grp in &block.primitivegroup {
                                    if let Some(d) = &grp.dense {
                                        let mut id = 0i64;
                                        let mut lat = 0i64;
                                        let mut lon = 0i64;
                                        let mut kv_idx = 0;
                                        
                                        for i in 0..d.id.len() {
                                            id += d.id[i];
                                            lat += d.lat[i];
                                            lon += d.lon[i];
                                            
                                            let la = 1e-9 * (lo + g * lat as f64);
                                            let ln = 1e-9 * (oo + g * lon as f64);
                                            
                                            let mut admin_level = None;
                                            let mut name = String::new();
                                            
                                            // Parse tags
                                            while kv_idx < d.keys_vals.len() {
                                                let k = d.keys_vals[kv_idx];
                                                kv_idx += 1;
                                                if k == 0 { break; }
                                                let v = d.keys_vals[kv_idx];
                                                kv_idx += 1;
                                                
                                                if let Some(st) = str_table {
                                                    if let Ok(key) = std::str::from_utf8(&st[k as usize]) {
                                                        if key == "admin_level" {
                                                            if let Ok(val) = std::str::from_utf8(&st[v as usize]) {
                                                                admin_level = val.parse().ok();
                                                            }
                                                        } else if key == "name" {
                                                            if let Ok(val) = std::str::from_utf8(&st[v as usize]) {
                                                                name = val.to_string();
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            tx.send(Node { id, lat: la, lon: ln, admin_level, name }).ok();
                                        }
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }));
    }
    
    drop(piece_rx);
    drop(node_tx);
    
    // Writer
    let tiles_clone = tiles.clone();
    let admin_clone = admin_file.clone();
    let writer = std::thread::spawn(move || -> Result<()> {
        let mut total = 0u64;
        let mut admin_count = 0u64;
        
        while let Ok(node) = node_rx.recv() {
            // Extract admin boundaries
            if let Some(level) = node.admin_level {
                if level >= 2 && level <= 10 && !node.name.is_empty() {
                    let mut af = admin_clone.lock().unwrap();
                    writeln!(af, "{{\"id\":{},\"level\":{},\"name\":\"{}\",\"lat\":{:.7},\"lon\":{:.7}}}", 
                        node.id, level, node.name.replace("\"", "\\\""), node.lat, node.lon)?;
                    admin_count += 1;
                }
            }
            
            // Tile coordinates (Z=8)
            let tile_x = (((node.lon + 180.0) / 360.0 * 256.0) as u8).min(255);
            let tile_y = (((90.0 - node.lat) / 180.0 * 256.0) as u8).min(255);
            let node_bucket = (node.id % 71) as u8;
            
            let k = (tile_x, tile_y, node_bucket);
            
            let mut t = tiles_clone.lock().unwrap();
            let f = t.entry(k).or_insert_with(|| {
                std::fs::create_dir_all(format!("tiles/tile_{}_{}", tile_x, tile_y)).ok();
                let p = format!("tiles/tile_{}_{}/nodes_{:02}.csv", tile_x, tile_y, node_bucket);
                File::create(p).unwrap()
            });
            writeln!(f, "{},{:.7},{:.7}", node.id, node.lat, node.lon)?;
            drop(t);
            
            total += 1;
            if total % 10_000_000 == 0 {
                println!("📊 {}M nodes | {} admin", total / 1_000_000, admin_count);
            }
        }
        
        println!("\n✅ {} nodes, {} admin boundaries", total, admin_count);
        Ok(())
    });
    
    reader.join().unwrap()?;
    for p in parsers { p.join().unwrap(); }
    writer.join().unwrap()?;
    
    Ok(())
}
