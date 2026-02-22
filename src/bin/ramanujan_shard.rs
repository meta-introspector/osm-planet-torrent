// Tile-shard with Ramanujan's primes: lat%71, lon%59, height%47
// This creates 71×59×47 = 196,883 tiles (Monster prime factorization)
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
}

fn main() -> Result<()> {
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    
    println!("🗺️  Ramanujan tile sharding: 71×59×47 (24 workers)");
    std::fs::create_dir_all("ramanujan_tiles")?;
    std::fs::create_dir_all("admin")?;
    
    let mut file = File::open(planet_file)?;
    let file_size = file.metadata()?.len();
    let piece_size = 4_194_304u64;
    let total_pieces = (file_size + piece_size - 1) / piece_size;
    
    println!("📈 {} GB, {} pieces", file_size / 1_000_000_000, total_pieces);
    println!("🎯 Tiles: 71 (lat) × 59 (lon) × 47 (level) = 196,883");
    
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
                                        
                                        for _ in 0..d.id.len() {
                                            id += d.id[kv_idx.min(d.id.len()-1)];
                                            lat += d.lat[kv_idx.min(d.lat.len()-1)];
                                            lon += d.lon[kv_idx.min(d.lon.len()-1)];
                                            
                                            let la = 1e-9 * (lo + g * lat as f64);
                                            let ln = 1e-9 * (oo + g * lon as f64);
                                            
                                            let mut admin_level = None;
                                            
                                            if let Some(st) = str_table {
                                                while kv_idx < d.keys_vals.len() {
                                                    let k = d.keys_vals[kv_idx];
                                                    kv_idx += 1;
                                                    if k == 0 { break; }
                                                    let v = d.keys_vals[kv_idx];
                                                    kv_idx += 1;
                                                    
                                                    if let Ok(key) = std::str::from_utf8(&st[k as usize]) {
                                                        if key == "admin_level" {
                                                            if let Ok(val) = std::str::from_utf8(&st[v as usize]) {
                                                                admin_level = val.parse().ok();
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            tx.send(Node { id, lat: la, lon: ln, admin_level }).ok();
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
            // Extract admin
            if let Some(level) = node.admin_level {
                if level >= 2 && level <= 10 {
                    let mut af = admin_clone.lock().unwrap();
                    writeln!(af, "{{\"id\":{},\"level\":{},\"lat\":{:.7},\"lon\":{:.7}}}", 
                        node.id, level, node.lat, node.lon)?;
                    admin_count += 1;
                }
            }
            
            // Ramanujan tiles: lat%71, lon%59, level%47
            let tile_lat = ((((node.lat + 90.0) * 100.0) as i64) % 71) as u8;
            let tile_lon = ((((node.lon + 180.0) * 100.0) as i64) % 59) as u8;
            let tile_level = node.admin_level.unwrap_or(0) % 47;
            
            let k = (tile_lat, tile_lon, tile_level);
            
            let mut t = tiles_clone.lock().unwrap();
            let f = t.entry(k).or_insert_with(|| {
                let p = format!("ramanujan_tiles/tile_{:02}_{:02}_{:02}.csv", tile_lat, tile_lon, tile_level);
                File::create(p).unwrap()
            });
            writeln!(f, "{},{:.7},{:.7}", node.id, node.lat, node.lon)?;
            drop(t);
            
            total += 1;
            if total % 10_000_000 == 0 {
                let tile_count = tiles_clone.lock().unwrap().len();
                println!("📊 {}M nodes | {} tiles | {} admin", total / 1_000_000, tile_count, admin_count);
            }
        }
        
        println!("\n✅ {} nodes → {} tiles, {} admin", total, tiles_clone.lock().unwrap().len(), admin_count);
        Ok(())
    });
    
    reader.join().unwrap()?;
    for p in parsers { p.join().unwrap(); }
    writer.join().unwrap()?;
    
    Ok(())
}
