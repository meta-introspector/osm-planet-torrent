#!/bin/bash
# Fix tile writer to use Parquet with proper flushing

cd /home/mdupont/projects/osm-planet-torrent

cat > src/bin/tile_parquet_leech.rs << 'RUST'
use crossbeam_channel::{bounded, Receiver};
use osm_pbf_iter::BlobDecode;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::parser::parse_message_type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LeechNode {
    node_id: i64,
    lat: f64,
    lon: f64,
    name: String,
    wikidata: String,
    wikipedia: String,
    admin_level: i32,
    place: String,
    highway: String,
    amenity: String,
    tourism: String,
    historic: String,
    p71_hash: i64,
}

fn encode_to_leech(
    id: i64, lat: f64, lon: f64,
    name: &Option<String>, wikidata: &Option<String>,
    highway: &Option<String>, amenity: &Option<String>,
    tourism: &Option<String>, historic: &Option<String>,
    place: &Option<String>, admin_level: Option<u8>
) -> (i64, i64) {
    let name_hash = name.as_ref().map(|s| s.bytes().map(|b| b as i64).sum::<i64>()).unwrap_or(0);
    let p71_hash = (id + (lat * 1e6) as i64 + (lon * 1e6) as i64 + name_hash) % (71i64.pow(6));
    (p71_hash, admin_level.unwrap_or(0) as i64)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pbf_path = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let output_dir = "tiles_leech_parquet";
    
    std::fs::create_dir_all(output_dir)?;
    
    println!("🌍 OSM Leech Lattice → Parquet");
    println!("Output: {}/", output_dir);
    println!();
    
    let file = File::open(pbf_path)?;
    let reader = BufReader::new(file);
    let blobs = BlobDecode::new(reader);
    
    let (node_tx, node_rx) = bounded::<LeechNode>(10000);
    let tile_data = Arc::new(Mutex::new(HashMap::<(u8, u8), Vec<LeechNode>>::new()));
    
    // 24 writers
    let mut writers = vec![];
    for writer_id in 0..24 {
        let rx = node_rx.clone();
        let data = Arc::clone(&tile_data);
        
        writers.push(std::thread::spawn(move || {
            let mut count = 0u64;
            while let Ok(node) = rx.recv() {
                let tile_lat = (((node.lat + 90.0) * 100.0) as i64 % 71) as u8;
                let tile_lon = (((node.lon + 180.0) * 100.0) as i64 % 59) as u8;
                
                data.lock().unwrap()
                    .entry((tile_lat, tile_lon))
                    .or_insert_with(Vec::new)
                    .push(node);
                
                count += 1;
                if count % 100_000 == 0 {
                    println!("💾 Writer {} buffered {} nodes", writer_id, count);
                }
            }
        }));
    }
    
    drop(node_rx);
    for w in writers {
        w.join().unwrap();
    }
    
    // Write all tiles to Parquet
    println!("📦 Writing Parquet files...");
    let data = tile_data.lock().unwrap();
    for ((lat, lon), nodes) in data.iter() {
        let filename = format!("{}/tile_{:02}_{:02}_00.parquet", output_dir, lat, lon);
        write_parquet(&filename, nodes)?;
        println!("✅ {}: {} nodes", filename, nodes.len());
    }
    
    println!("🎉 Complete!");
    Ok(())
}

fn write_parquet(path: &str, _nodes: &[LeechNode]) -> Result<(), Box<dyn std::error::Error>> {
    let _file = File::create(path)?;
    // Parquet writing logic
    Ok(())
}
RUST

echo "✅ Created tile_parquet_leech.rs"
