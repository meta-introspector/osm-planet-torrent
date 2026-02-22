// OSM Planet → 24D Leech Lattice with Monster Group Encoding
// Each node gets: lat, lon, tags, 24D coords, 71-adic hash, 15D Maass shadow

use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crossbeam::channel::bounded;
use prost::Message;
use flate2::read::ZlibDecoder;
use serde::{Serialize, Deserialize};

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
const NAMAGIRI_SEEDS: [u64; 7] = [1729, 196883, 744, 691, 24, 71, 42];

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LeechNode {
    // Core
    node_id: i64,
    lat: f64,
    lon: f64,
    
    // Tags
    name: Option<String>,
    wikidata: Option<String>,
    wikipedia: Option<String>,
    admin_level: Option<u8>,
    place: Option<String>,
    highway: Option<String>,
    amenity: Option<String>,
    tourism: Option<String>,
    historic: Option<String>,
    
    // 24D Leech Lattice
    leech_coords: [i32; 24],
    
    // 71-adic encoding
    p71_hash: u64,
    
    // 15D Maass shadow
    maass_shadow: [i32; 15],
}

fn hash_71_adic(data: &[u8]) -> u64 {
    let mut hash = 1729u64;
    for (i, &byte) in data.iter().enumerate() {
        let seed = NAMAGIRI_SEEDS[i % 7];
        hash = hash.wrapping_mul(71)
            .wrapping_add(byte as u64)
            .wrapping_add(seed);
    }
    hash % (71u64.pow(6))
}

fn maass_shadow(p71: u64) -> [i32; 15] {
    let mut shadow = [0i32; 15];
    for (i, &prime) in MONSTER_PRIMES.iter().enumerate() {
        shadow[i] = (p71 % prime) as i32;
    }
    shadow
}

fn hash_string_to_prime(s: &str) -> u64 {
    let hash = s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    MONSTER_PRIMES[(hash % 15) as usize]
}

fn encode_to_leech(
    node_id: i64,
    lat: f64,
    lon: f64,
    name: &Option<String>,
    wikidata: &Option<String>,
    highway: &Option<String>,
    amenity: &Option<String>,
    tourism: &Option<String>,
    historic: &Option<String>,
    place: &Option<String>,
    admin_level: Option<u8>,
) -> ([i32; 24], u64, [i32; 15]) {
    let mut coords = [0i32; 24];
    
    // Dims 0-1: Lat/Lon (scaled to ±71)
    coords[0] = ((lat + 90.0) * 71.0 / 180.0) as i32 % 71;
    coords[1] = ((lon + 180.0) * 71.0 / 360.0) as i32 % 71;
    
    // Dim 2: Admin level
    coords[2] = admin_level.unwrap_or(0) as i32 % 71;
    
    // Dims 3-8: Name hash (6 dimensions)
    if let Some(n) = name {
        let hash = hash_71_adic(n.as_bytes());
        for i in 0..6 {
            coords[3 + i] = ((hash >> (i * 8)) & 0xFF) as i32 % 71;
        }
    }
    
    // Dim 9: Wikidata Q-ID
    if let Some(wd) = wikidata {
        if let Some(qid) = wd.strip_prefix("Q") {
            coords[9] = qid.parse::<i32>().unwrap_or(0) % 71;
        }
    }
    
    // Dims 10-14: Tag hashes
    let tags = [highway, amenity, tourism, historic, place];
    for (i, tag) in tags.iter().enumerate() {
        if let Some(t) = tag {
            coords[10 + i] = (hash_string_to_prime(t) % 71) as i32;
        }
    }
    
    // Compute 71-adic hash of all data
    let mut all_data = Vec::new();
    all_data.extend_from_slice(&node_id.to_le_bytes());
    all_data.extend_from_slice(&lat.to_le_bytes());
    all_data.extend_from_slice(&lon.to_le_bytes());
    if let Some(n) = name { all_data.extend_from_slice(n.as_bytes()); }
    if let Some(w) = wikidata { all_data.extend_from_slice(w.as_bytes()); }
    
    let p71 = hash_71_adic(&all_data);
    let shadow = maass_shadow(p71);
    
    (coords, p71, shadow)
}

fn main() -> Result<()> {
    println!("🗺️  OSM Planet → 24D Leech Lattice + Monster Group");
    println!("📊 Extracting: wikidata, tags, 24D coords, 71-adic hash, 15D shadow");
    
    std::fs::create_dir_all("tiles_leech")?;
    std::fs::create_dir_all("admin")?;
    
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let mut file = File::open(planet_file)?;
    let file_size = file.metadata()?.len();
    let piece_size = 4_194_304u64;
    let total_pieces = (file_size + piece_size - 1) / piece_size;
    
    println!("📈 {} GB, {} pieces", file_size / 1_000_000_000, total_pieces);
    
    let (piece_tx, piece_rx) = bounded(48);
    let (node_tx, node_rx) = bounded(100_000);
    
    // Tile writers: one file per tile, written incrementally
    let tile_writers = Arc::new(Mutex::new(HashMap::<(u8, u8), File>::new()));
    
    // Reader thread
    let reader = std::thread::spawn(move || -> Result<()> {
        for piece_id in 0..total_pieces as u32 {
            let mut buf = vec![0u8; piece_size as usize];
            let n = file.read(&mut buf)?;
            buf.truncate(n);
            piece_tx.send((piece_id, buf)).ok();
            
            if piece_id % 1000 == 0 {
                println!("📖 Piece {}/{}", piece_id, total_pieces);
            }
        }
        Ok(())
    });
    
    // 24 parser threads
    let mut parsers = vec![];
    for worker_id in 0..24 {
        let rx = piece_rx.clone();
        let tx = node_tx.clone();
        
        parsers.push(std::thread::spawn(move || {
            let mut nodes_processed = 0u64;
            
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
                                            
                                            let mut name = None;
                                            let mut wikidata = None;
                                            let mut wikipedia = None;
                                            let mut admin_level = None;
                                            let mut place = None;
                                            let mut highway = None;
                                            let mut amenity = None;
                                            let mut tourism = None;
                                            let mut historic = None;
                                            
                                            // Parse tags
                                            while kv_idx < d.keys_vals.len() {
                                                let k = d.keys_vals[kv_idx];
                                                kv_idx += 1;
                                                if k == 0 { break; }
                                                let v = d.keys_vals[kv_idx];
                                                kv_idx += 1;
                                                
                                                if let Some(st) = str_table {
                                                    if let Ok(key) = std::str::from_utf8(&st[k as usize]) {
                                                        if let Ok(val) = std::str::from_utf8(&st[v as usize]) {
                                                            match key {
                                                                "name" => name = Some(val.to_string()),
                                                                "wikidata" => wikidata = Some(val.to_string()),
                                                                "wikipedia" => wikipedia = Some(val.to_string()),
                                                                "admin_level" => admin_level = val.parse().ok(),
                                                                "place" => place = Some(val.to_string()),
                                                                "highway" => highway = Some(val.to_string()),
                                                                "amenity" => amenity = Some(val.to_string()),
                                                                "tourism" => tourism = Some(val.to_string()),
                                                                "historic" => historic = Some(val.to_string()),
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            // Encode to Leech lattice
                                            let (leech_coords, p71_hash, maass_shadow) = encode_to_leech(
                                                id, la, ln, &name, &wikidata, &highway, &amenity,
                                                &tourism, &historic, &place, admin_level
                                            );
                                            
                                            let node = LeechNode {
                                                node_id: id,
                                                lat: la,
                                                lon: ln,
                                                name,
                                                wikidata,
                                                wikipedia,
                                                admin_level,
                                                place,
                                                highway,
                                                amenity,
                                                tourism,
                                                historic,
                                                leech_coords,
                                                p71_hash,
                                                maass_shadow,
                                            };
                                            
                                            tx.send(node).ok();
                                            nodes_processed += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            println!("✅ Worker {} processed {} nodes", worker_id, nodes_processed);
        }));
    }
    
    drop(piece_rx);
    drop(node_tx);
    
    // 24 writer threads (one per worker)
    let mut writers = vec![];
    for writer_id in 0..24 {
        let rx = node_rx.clone();
        let tile_writers = Arc::clone(&tile_writers);
        
        writers.push(std::thread::spawn(move || {
            let mut nodes_written = 0u64;
            
            while let Ok(node) = rx.recv() {
                // Calculate Ramanujan tile
                let tile_lat = (((node.lat + 90.0) * 100.0) as i64 % 71) as u8;
                let tile_lon = (((node.lon + 180.0) * 100.0) as i64 % 59) as u8;
                
                // Get or create file for this tile
                let mut writers = tile_writers.lock().unwrap();
                let file = writers.entry((tile_lat, tile_lon))
                    .or_insert_with(|| {
                        let filename = format!("tiles_leech/tile_{:02}_{:02}_00.jsonl", tile_lat, tile_lon);
                        File::create(&filename).unwrap()
                    });
                
                // Write node as JSONL
                if let Ok(json) = serde_json::to_string(&node) {
                    writeln!(file, "{}", json).ok();
                }
                
                nodes_written += 1;
                
                if nodes_written % 100_000 == 0 {
                    println!("💾 Writer {} wrote {} nodes", writer_id, nodes_written);
                }
            }
            
            println!("✅ Writer {} finished: {} nodes", writer_id, nodes_written);
        }));
    }
    
    drop(node_rx);
    
    reader.join().unwrap()?;
    for p in parsers {
        p.join().unwrap();
    }
    for w in writers {
        w.join().unwrap();
    }
    
    // Flush and close all files
    println!("💾 Flushing all tile files...");
    let mut writers = tile_writers.lock().unwrap();
    for ((lat, lon), file) in writers.iter_mut() {
        file.flush()?;
        println!("✅ Flushed tile_{:02}_{:02}_00.jsonl", lat, lon);
    }
    drop(writers);
    
    let total_tiles = tile_writers.lock().unwrap().len();
    println!("🎉 Complete! {} tiles written", total_tiles);
    
    println!("🎉 Complete! OSM planet encoded in 24D Leech lattice");
    
    Ok(())
}
