#!/bin/bash
# Create minimal Parquet writer

cd /home/mdupont/projects/osm-planet-torrent

cat > src/bin/tile_parquet.rs << 'RUST'
use crossbeam::channel::bounded;
use osm_pbf_iter::BlobDecode;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::parser::parse_message_type;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 OSM → Parquet tiles (71×59)");
    
    let pbf = File::open("/mnt/data1/osm-planet/planet-latest.osm.pbf")?;
    let blobs = BlobDecode::new(BufReader::new(pbf));
    
    let (tx, rx) = bounded(10000);
    let tiles = Arc::new(Mutex::new(HashMap::<(u8, u8), Vec<(i64, f64, f64)>>::new()));
    
    // 24 writers
    let mut writers = vec![];
    for id in 0..24 {
        let rx = rx.clone();
        let tiles = Arc::clone(&tiles);
        writers.push(std::thread::spawn(move || {
            let mut count = 0u64;
            while let Ok((node_id, lat, lon)) = rx.recv() {
                let tile_lat = (((lat + 90.0) * 100.0) as i64 % 71) as u8;
                let tile_lon = (((lon + 180.0) * 100.0) as i64 % 59) as u8;
                tiles.lock().unwrap()
                    .entry((tile_lat, tile_lon))
                    .or_insert_with(Vec::new)
                    .push((node_id, lat, lon));
                count += 1;
                if count % 1_000_000 == 0 {
                    println!("💾 Writer {}: {} nodes", id, count);
                }
            }
        }));
    }
    
    drop(rx);
    for w in writers { w.join().unwrap(); }
    
    // Write Parquet
    std::fs::create_dir_all("tiles_parquet")?;
    let tiles = tiles.lock().unwrap();
    for ((lat, lon), nodes) in tiles.iter() {
        let path = format!("tiles_parquet/tile_{:02}_{:02}.parquet", lat, lon);
        write_parquet(&path, nodes)?;
        println!("✅ {}: {} nodes", path, nodes.len());
    }
    
    Ok(())
}

fn write_parquet(path: &str, nodes: &[(i64, f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let schema = "message schema { REQUIRED INT64 node_id; REQUIRED DOUBLE lat; REQUIRED DOUBLE lon; }";
    let schema = Arc::new(parse_message_type(schema)?);
    let file = File::create(path)?;
    let props = WriterProperties::builder().build();
    let mut writer = SerializedFileWriter::new(file, schema, Arc::new(props))?;
    
    let mut row_group = writer.next_row_group()?;
    for (id, lat, lon) in nodes {
        let mut row = parquet::record::Row::new();
        row.push((*id).into());
        row.push((*lat).into());
        row.push((*lon).into());
        row_group.append_row(row)?;
    }
    row_group.close()?;
    writer.close()?;
    
    Ok(())
}
RUST

echo "✅ Created src/bin/tile_parquet.rs"
