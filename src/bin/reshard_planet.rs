// Re-shard OSM planet with 24 parallel workers using crossbeam
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
}

struct Node {
    id: i64,
    lat: f64,
    lon: f64,
}

fn main() -> Result<()> {
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let output_dir = "shards";
    
    println!("🗺️  Re-sharding OSM planet (24 workers)");
    std::fs::create_dir_all(output_dir)?;
    
    let mut file = File::open(planet_file)?;
    let file_size = file.metadata()?.len();
    let piece_size = 4_194_304u64;
    let total_pieces = (file_size + piece_size - 1) / piece_size;
    
    println!("📈 {} GB, {} pieces", file_size / 1_000_000_000, total_pieces);
    
    // Channels
    let (piece_tx, piece_rx): (Sender<(u32, Vec<u8>)>, Receiver<(u32, Vec<u8>)>) = bounded(48);
    let (node_tx, node_rx): (Sender<Node>, Receiver<Node>) = bounded(100_000);
    
    // Shared bucket files
    let buckets = Arc::new(Mutex::new(HashMap::<(u8, u8, u8), File>::new()));
    let counts = Arc::new(Mutex::new(HashMap::<(u8, u8, u8), usize>::new()));
    
    // Reader thread
    let reader = std::thread::spawn(move || -> Result<()> {
        for piece_id in 0..total_pieces as u32 {
            let mut buf = vec![0u8; piece_size as usize];
            let n = file.read(&mut buf)?;
            buf.truncate(n);
            piece_tx.send((piece_id, buf)).ok();
        }
        Ok(())
    });
    
    // 24 parser workers
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
                                
                                for grp in &block.primitivegroup {
                                    if let Some(d) = &grp.dense {
                                        let mut id = 0i64;
                                        let mut lat = 0i64;
                                        let mut lon = 0i64;
                                        
                                        for i in 0..d.id.len() {
                                            id += d.id[i];
                                            lat += d.lat[i];
                                            lon += d.lon[i];
                                            
                                            let la = 1e-9 * (lo + g * lat as f64);
                                            let ln = 1e-9 * (oo + g * lon as f64);
                                            
                                            tx.send(Node { id, lat: la, lon: ln }).ok();
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
    
    // Writer thread
    let buckets_clone = buckets.clone();
    let counts_clone = counts.clone();
    let writer = std::thread::spawn(move || -> Result<()> {
        let mut total = 0u64;
        
        while let Ok(node) = node_rx.recv() {
            let bid = (node.id % 71) as u8;
            let bla = ((((node.lat + 90.0) * 100.0) as i64) % 41) as u8;
            let bln = ((((node.lon + 180.0) * 100.0) as i64) % 31) as u8;
            let k = (bid, bla, bln);
            
            let mut b = buckets_clone.lock().unwrap();
            let f = b.entry(k).or_insert_with(|| {
                let p = format!("{}/b_{}_{:02}_{:02}.csv", output_dir, bid, bla, bln);
                File::create(p).unwrap()
            });
            writeln!(f, "{},{:.7},{:.7}", node.id, node.lat, node.lon)?;
            drop(b);
            
            let mut c = counts_clone.lock().unwrap();
            *c.entry(k).or_insert(0) += 1;
            drop(c);
            
            total += 1;
            if total % 10_000_000 == 0 {
                println!("📊 Nodes: {}M", total / 1_000_000);
            }
        }
        
        Ok(())
    });
    
    reader.join().unwrap()?;
    for p in parsers { p.join().unwrap(); }
    writer.join().unwrap()?;
    
    let c = counts.lock().unwrap();
    println!("\n✅ {} buckets created", c.len());
    
    Ok(())
}
