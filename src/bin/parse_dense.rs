// Parse decompressed PrimitiveBlock and extract DenseNodes
use prost::Message;
use std::fs;

// Generated from osmformat.proto
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

fn main() -> anyhow::Result<()> {
    let blocks = vec![
        "piece_0000001_reconstructed_block_0_decompressed.bin",
        "piece_0000004_reconstructed_block_0_decompressed.bin",
    ];
    
    for block_file in blocks {
        println!("\n📦 {}", block_file);
        let data = fs::read(block_file)?;
        println!("   Size: {} bytes", data.len());
        
        let block = match PrimitiveBlock::decode(&data[..]) {
            Ok(b) => b,
            Err(e) => {
                println!("   ✗ Decode error: {}", e);
                continue;
            }
        };
        
        let granularity = block.granularity.unwrap_or(100) as f64;
        let lat_offset = block.lat_offset.unwrap_or(0) as f64;
        let lon_offset = block.lon_offset.unwrap_or(0) as f64;
        
        let str_table = &block.stringtable.as_ref().unwrap().s;
        let resolve = |sid: u32| -> String {
            String::from_utf8_lossy(&str_table[sid as usize]).into_owned()
        };
        
        let mut min_lat = f64::MAX;
        let mut max_lat = f64::MIN;
        let mut min_lon = f64::MAX;
        let mut max_lon = f64::MIN;
        
        for group in &block.primitivegroup {
            if let Some(dense) = &group.dense {
                println!("   DenseNodes: {} nodes", dense.id.len());
                
                // Delta decode
                let mut acc_id = 0i64;
                let mut acc_lat = 0i64;
                let mut acc_lon = 0i64;
                
                let mut kv_index = 0;
                let mut wikidata_count = 0;
                
                for i in 0..dense.id.len() {
                    acc_id += dense.id[i];
                    acc_lat += dense.lat[i];
                    acc_lon += dense.lon[i];
                    
                    let lat_deg = 1e-9 * (lat_offset + granularity * acc_lat as f64);
                    let lon_deg = 1e-9 * (lon_offset + granularity * acc_lon as f64);
                    
                    // Track bbox
                    min_lat = min_lat.min(lat_deg);
                    max_lat = max_lat.max(lat_deg);
                    min_lon = min_lon.min(lon_deg);
                    max_lon = max_lon.max(lon_deg);
                    
                    // Parse tags
                    let mut has_wikidata = false;
                    let mut name = String::new();
                    let mut wikidata_qid = String::new();
                    
                    while kv_index < dense.keys_vals.len() {
                        let k = dense.keys_vals[kv_index];
                        kv_index += 1;
                        if k == 0 { break; }
                        
                        let v = dense.keys_vals[kv_index];
                        kv_index += 1;
                        
                        let key = resolve(k);
                        let val = resolve(v);
                        
                        if key == "wikidata" {
                            has_wikidata = true;
                            wikidata_qid = val;
                        } else if key == "name" {
                            name = val;
                        }
                    }
                    
                    if has_wikidata {
                        wikidata_count += 1;
                        println!("     🎯 Node {} ({}) at {:.4}, {:.4} - {}", 
                            acc_id, name, lat_deg, lon_deg, wikidata_qid);
                    }
                }
                
                println!("   ✓ Wikidata entities: {}", wikidata_count);
            }
        }
        
        println!("   📍 BBox: lat [{:.4}, {:.4}], lon [{:.4}, {:.4}]",
            min_lat, max_lat, min_lon, max_lon);
    }
    
    Ok(())
}
