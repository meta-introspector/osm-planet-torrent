use librqbit::*;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

pub async fn download_specific_piece(
    torrent_url: &str,
    output_dir: &str,
    piece_index: u32,
    storage_factory: Option<librqbit::storage::BoxStorageFactory>,
) -> Result<Vec<u8>> {
    println!("🔗 Downloading piece {} only...", piece_index);
    
    let mut log = File::create("piece_download.log")?;
    writeln!(log, "=== Selective Piece Download ===")?;
    writeln!(log, "Torrent: {}", torrent_url)?;
    writeln!(log, "Target piece: {}", piece_index)?;
    
    // Use memory-only storage to avoid creating 80GB file
    let session = Session::new_with_opts(
        PathBuf::from(output_dir),
        SessionOptions {
            disable_dht: false,
            disable_dht_persistence: true,
            ..Default::default()
        }
    ).await?;
    
    let handle = match session.add_torrent(
        AddTorrent::from_cli_argument(torrent_url)?,
        Some(AddTorrentOptions {
            overwrite: true,
            only_files: Some(vec![0]),
            paused: false,
            storage_factory,
            ..Default::default()
        })
    ).await? {
        AddTorrentResponse::Added(_, h) | AddTorrentResponse::AlreadyManaged(_, h) => h,
        AddTorrentResponse::ListOnly(_) => return Err(anyhow::anyhow!("List only")),
    };
    
    // Log metadata
    handle.with_metadata(|meta| {
        let lengths = meta.info.lengths();
        println!("📦 Torrent: {:?}", meta.info.name());
        println!("   Total pieces: {}", lengths.total_pieces());
        println!("   Piece length: {} KB", lengths.default_piece_length() / 1024);
        println!("   Target piece: {}", piece_index);
        
        writeln!(log, "Total pieces: {}", lengths.total_pieces()).ok();
        writeln!(log, "Piece length: {} KB", lengths.default_piece_length() / 1024).ok();
    })?;
    
    println!("\n⏳ Waiting for torrent to start...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    println!("⏳ Waiting for piece {} to download...", piece_index);
    
    // Wait for specific piece
    let mut last_stats = std::time::Instant::now();
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Show stats every 5 seconds
        if last_stats.elapsed() > Duration::from_secs(5) {
            let stats = handle.stats();
            println!("📊 {}", stats);
            last_stats = std::time::Instant::now();
        }
        
        match handle.is_piece_downloaded(piece_index) {
            Ok(true) => {
                println!("✓ Piece {} downloaded!", piece_index);
                writeln!(log, "Piece {} downloaded", piece_index)?;
                break;
            }
            Ok(false) => {
                print!(".");
                std::io::stdout().flush().ok();
            }
            Err(e) => {
                // Torrent might not be live yet, keep waiting
                if e.to_string().contains("no chunk tracker") {
                    print!(".");
                    std::io::stdout().flush().ok();
                } else {
                    println!("\n✗ Error checking piece: {}", e);
                    return Err(e);
                }
            }
        }
    }
    
    println!("\n✓ Piece download complete");
    
    // Read piece data from storage
    // TODO: Get piece data without reading from disk
    Ok(vec![])
}
