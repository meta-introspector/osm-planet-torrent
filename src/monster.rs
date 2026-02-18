use serde::{Deserialize, Serialize};

// Monster Group constants
pub const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
pub const OMEGA_PRIME: u64 = 71; // Largest Monster prime
pub const DISK_BLOCKS_PER_SHARD: u64 = 113; // Our metric

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterProjection {
    pub total_pieces: usize,
    pub piece_length: i64,
    pub total_size_gb: f64,
    pub shards: Vec<ShardInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShardInfo {
    pub shard_id: u64,
    pub prime: u64,
    pub pieces: Vec<usize>,
    pub size_mb: f64,
    pub disk_blocks: u64,
}

pub fn calculate_monster_projection(num_pieces: usize, piece_length: i64) -> MonsterProjection {
    let total_size_gb = (num_pieces as f64 * piece_length as f64) / (1024.0 * 1024.0 * 1024.0);
    
    // Distribute pieces across 71 shards
    let mut shards = Vec::new();
    for shard_id in 0..OMEGA_PRIME {
        let mut pieces = Vec::new();
        for piece in 0..num_pieces {
            if (piece as u64) % OMEGA_PRIME == shard_id {
                pieces.push(piece);
            }
        }
        
        let size_mb = (pieces.len() as f64 * piece_length as f64) / (1024.0 * 1024.0);
        let disk_blocks = (size_mb / 4.0).ceil() as u64 * DISK_BLOCKS_PER_SHARD;
        
        // Find corresponding Monster prime
        let prime = if shard_id < MONSTER_PRIMES.len() as u64 {
            MONSTER_PRIMES[shard_id as usize]
        } else {
            shard_id // Use shard_id if beyond Monster primes
        };
        
        shards.push(ShardInfo {
            shard_id,
            prime,
            pieces,
            size_mb,
            disk_blocks,
        });
    }
    
    MonsterProjection {
        total_pieces: num_pieces,
        piece_length,
        total_size_gb,
        shards,
    }
}

pub fn print_monster_projection(proj: &MonsterProjection) {
    println!("\n🐉 Monster Group Projection:");
    println!("  Total: {} pieces, {:.2} GB", proj.total_pieces, proj.total_size_gb);
    println!("  Shards: {} (mod {})", OMEGA_PRIME, OMEGA_PRIME);
    println!("  Metric: {} disk blocks per shard", DISK_BLOCKS_PER_SHARD);
    println!("\n  Top 15 Shards (Monster Primes):");
    
    for i in 0..15.min(proj.shards.len()) {
        let shard = &proj.shards[i];
        println!("    Shard {:2} (prime {:2}): {:4} pieces, {:6.2} MB, {:5} blocks",
            shard.shard_id, shard.prime, shard.pieces.len(), shard.size_mb, shard.disk_blocks);
    }
    
    let total_blocks: u64 = proj.shards.iter().map(|s| s.disk_blocks).sum();
    println!("\n  Total disk blocks: {} ({:.2} GB at 4KB/block)",
        total_blocks, total_blocks as f64 * 4.0 / 1024.0 / 1024.0);
}
