use librqbit::*;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

pub async fn stream_and_filter(
    torrent_file: &str, 
    output_dir: &str,
    lat: f64, 
    lon: f64, 
    radius_miles: f64
) -> Result<Vec<(String, String, f64, f64)>> {
    println!("🔗 Starting torrent download...");
    
    let mut log = File::create("filter.log")?;
    writeln!(log, "=== OSM Torrent Filter Log ===")?;
    writeln!(log, "Torrent: {}", torrent_file)?;
    writeln!(log, "Output: {}", output_dir)?;
    writeln!(log, "Filter: lat={}, lon={}, radius={} miles\n", lat, lon, radius_miles)?;
    
    let session = Session::new(PathBuf::from(output_dir)).await?;
    
    println!("📥 Adding torrent: {}", torrent_file);
    writeln!(log, "Adding torrent...")?;
    
    let handle = match session.add_torrent(
        AddTorrent::from_url(torrent_file),
        Some(AddTorrentOptions {
            overwrite: true,
            ..Default::default()
        })
    ).await? {
        AddTorrentResponse::Added(id, handle) => {
            println!("✓ Torrent added: {:?}", id);
            writeln!(log, "Torrent added: {:?}", id)?;
            handle
        },
        AddTorrentResponse::AlreadyManaged(id, handle) => {
            println!("✓ Torrent already managed: {:?}", id);
            writeln!(log, "Torrent already managed: {:?}", id)?;
            handle
        },
        AddTorrentResponse::ListOnly(_) => return Err(anyhow::anyhow!("List only")),
    };
    
    // Log torrent info
    handle.with_metadata(|meta| {
        let file_count = meta.info.iter_file_details().count();
        println!("📦 Torrent info:");
        println!("   Name: {:?}", meta.info.name());
        println!("   Files: {}", file_count);
        println!("   Total size: {} MB", meta.info.lengths().total_length() / 1_000_000);
        
        writeln!(log, "\nTorrent metadata:").ok();
        writeln!(log, "  Name: {:?}", meta.info.name()).ok();
        writeln!(log, "  Files: {}", file_count).ok();
        writeln!(log, "  Total size: {} MB", meta.info.lengths().total_length() / 1_000_000).ok();
        
        for (idx, file) in meta.info.iter_file_details().enumerate() {
            println!("   [{}] {} ({} MB)", idx, file.filename, file.len / 1_000_000);
            writeln!(log, "  File[{}]: {} ({} MB)", idx, file.filename, file.len / 1_000_000).ok();
        }
    })?;
    
    println!("\n⏳ Downloading OSM data...");
    println!("   Filtering for lat={}, lon={}, radius={} miles", lat, lon, radius_miles);
    writeln!(log, "\nStarting download...")?;
    
    // Monitor progress
    tokio::spawn({
        let handle = handle.clone();
        let mut log = File::create("download.log").unwrap();
        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;
                let stats = handle.stats();
                println!("📊 Progress: {}", stats);
                writeln!(log, "{}", stats).ok();
                log.flush().ok();
            }
        }
    });
    
    handle.wait_until_completed().await?;
    
    println!("\n✓ Download complete, now parsing...");
    writeln!(log, "\nDownload complete!")?;
    
    // Parse the downloaded file
    let bbox = crate::download::BoundingBox::from_center(lat, lon, radius_miles);
    let pbf_path = PathBuf::from(output_dir).join("planet-260209.osm.pbf");
    
    println!("📖 Reading PBF: {}", pbf_path.display());
    writeln!(log, "Reading PBF: {}", pbf_path.display())?;
    
    let pbf_data = std::fs::read(&pbf_path)?;
    println!("📦 PBF size: {} MB", pbf_data.len() / 1_000_000);
    writeln!(log, "PBF size: {} MB", pbf_data.len() / 1_000_000)?;
    
    println!("🔍 Filtering entities in bounding box...");
    writeln!(log, "\nFiltering entities...")?;
    
    let results = crate::download::extract_wikidata_from_pbf(&pbf_data, &bbox)?;
    
    println!("\n✓ Filtered {} entities with Wikidata tags", results.len());
    writeln!(log, "\nFiltered {} entities:", results.len())?;
    
    for (name, qid, lat, lon) in &results {
        writeln!(log, "  {} ({}) at {:.6}, {:.6}", name, qid, lat, lon)?;
    }
    
    log.flush()?;
    println!("📝 Full log: filter.log, download.log");
    
    Ok(results)
}
