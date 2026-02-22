// Re-shard existing tiles from tile-shard into Ramanujan tiles (71×59×47)
use anyhow::Result;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("🗺️  Re-sharding tiles → Ramanujan tiles (71×59×47)");
    
    std::fs::create_dir_all("ramanujan_tiles")?;
    
    let mut ram_tiles: HashMap<(u8, u8, u8), File> = HashMap::new();
    let mut total = 0u64;
    
    // Read all existing tile files
    for entry in read_dir("tiles")? {
        let tile_dir = entry?;
        if !tile_dir.file_type()?.is_dir() { continue; }
        
        for node_file in read_dir(tile_dir.path())? {
            let file = node_file?;
            if !file.path().extension().map_or(false, |e| e == "csv") { continue; }
            
            let reader = BufReader::new(File::open(file.path())?);
            
            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() != 3 { continue; }
                
                let id: i64 = parts[0].parse()?;
                let lat: f64 = parts[1].parse()?;
                let lon: f64 = parts[2].parse()?;
                
                // Ramanujan tiles
                let tile_lat = ((((lat + 90.0) * 100.0) as i64) % 71) as u8;
                let tile_lon = ((((lon + 180.0) * 100.0) as i64) % 59) as u8;
                let tile_level = 0u8; // Will extract from admin_level later
                
                let k = (tile_lat, tile_lon, tile_level);
                
                let f = ram_tiles.entry(k).or_insert_with(|| {
                    let p = format!("ramanujan_tiles/tile_{:02}_{:02}_{:02}.csv", tile_lat, tile_lon, tile_level);
                    File::create(p).unwrap()
                });
                
                writeln!(f, "{},{:.7},{:.7}", id, lat, lon)?;
                
                total += 1;
                if total % 1_000_000 == 0 {
                    println!("📊 {}M nodes → {} tiles", total / 1_000_000, ram_tiles.len());
                }
            }
        }
    }
    
    println!("\n✅ {} nodes → {} Ramanujan tiles", total, ram_tiles.len());
    
    Ok(())
}
