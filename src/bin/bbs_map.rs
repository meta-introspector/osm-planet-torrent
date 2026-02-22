use flate2::read::GzDecoder;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tile_path = "/mnt/data1/osm-planet/leech-tiles/tiles_leech/tile_14_38_00.jsonl.gz";
    
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║          KUMBAKONAM BBS MAP - Ultima Online Style            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    
    let file = File::open(tile_path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);
    
    let mut grid = vec![vec![' '; 80]; 40];
    let mut legend = HashMap::new();
    
    let (min_lat, max_lat) = (10.8, 11.0);
    let (min_lon, max_lon) = (79.3, 79.5);
    
    for (line_num, line) in reader.lines().enumerate() {
        let line_str = match line {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Line {}: Read error: {}", line_num, e);
                continue;
            }
        };
        
        let v: Value = match serde_json::from_str(&line_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Line {}: JSON error: {}", line_num, e);
                eprintln!("Content: {}", &line_str[..line_str.len().min(100)]);
                continue;
            }
        };
        
        let lat = v["lat"].as_f64().unwrap_or(0.0);
        let lon = v["lon"].as_f64().unwrap_or(0.0);
        
        if lat < min_lat || lat > max_lat || lon < min_lon || lon > max_lon {
            continue;
        }
        
        let x = ((lon - min_lon) / (max_lon - min_lon) * 79.0) as usize;
        let y = 39 - ((lat - min_lat) / (max_lat - min_lat) * 39.0) as usize;
        
        let symbol = if !v["place"].is_null() {
            legend.insert('⌂', "Town/City");
            '⌂'
        } else if v["amenity"].as_str() == Some("place_of_worship") {
            legend.insert('†', "Temple");
            '†'
        } else if !v["highway"].is_null() {
            legend.insert('═', "Road");
            '═'
        } else if !v["amenity"].is_null() {
            legend.insert('■', "Amenity");
            '■'
        } else {
            legend.insert('·', "Node");
            '·'
        };
        
        if x < 80 && y < 40 {
            grid[y][x] = symbol;
        }
    }
    
    println!("    ┌{}┐", "─".repeat(80));
    for row in &grid {
        print!("    │");
        for &c in row {
            print!("{}", c);
        }
        println!("│");
    }
    println!("    └{}┘", "─".repeat(80));
    println!();
    
    println!("Legend:");
    for (sym, desc) in legend {
        println!("  {} = {}", sym, desc);
    }
    
    Ok(())
}
