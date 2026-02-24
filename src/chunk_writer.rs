// Write downloaded chunks to Parquet files
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkRecord {
    pub piece_id: u32,
    pub chunk_offset: u32,
    pub chunk_size: u32,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

pub fn write_chunk_to_parquet(
    piece_id: u32,
    chunk_offset: u32,
    data: &[u8],
    output_dir: &str,
) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;
    
    let filename = format!("{}/piece_{:07}_chunk_{:08}.json", output_dir, piece_id, chunk_offset);
    
    let record = ChunkRecord {
        piece_id,
        chunk_offset,
        chunk_size: data.len() as u32,
        data: data.to_vec(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
    };
    
    // For now write as JSON, TODO: convert to Parquet
    let json = serde_json::to_string(&record)?;
    std::fs::write(&filename, json)?;
    
    println!("✓ Wrote chunk to {}", filename);
    
    Ok(())
}
